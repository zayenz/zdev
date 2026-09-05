import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { isAbsolute, resolve } from "node:path";
import { Type } from "typebox";

const rolePrompts = {
  "routine-implementer":
    "Implement one tightly specified routine task. Load its snapshot, make the smallest complete change, and run listed validation. Named paths are expected seams, not an allowlist; include another directly necessary path when its baseline ownership is clear and it stays within the task's semantic boundaries. Block only on unclear ownership, a real scope or product decision, or an unavailable prerequisite. Do not block on partial progress or another file. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if the work unexpectedly needs one; never run derive commands.",
  implementer:
    "Implement one selected task. Load its snapshot, follow repository guidance and the task's testing level, and run validation. Named paths are expected seams, not an allowlist; include another directly necessary path when its baseline ownership is clear and it stays within the task's semantic boundaries. Block only on ambiguous ownership, a real scope or user decision, or an unavailable prerequisite. Do not block on partial progress or another file. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if necessary; never run derive commands.",
  verifier:
    "For task verification only, verify one task read-only. Load its snapshot, use the implementer summary only to locate evidence, check the whole task, attribute every change, and run required validation. Prefer check or dry-run forms for generators and other commands expected to rewrite tracked files; do not run a mutating generator merely to prove that implementation should have run it. Return exactly one JSON object with exactly four keys: verdict, summary, findings, and escalation. Add no fifth key. Use pass with no findings for success, rework with at least one finding for a task-owned defect or write, and blocker for ambiguous ownership, missing evidence, or a user decision. Set escalation to none, except that rework may request advanced-implementer. Name each unexpected validation-written task-owned file as validation_write: <repository-relative path>, use rework, and never add a validation_writes key. Coordination owns snapshot comparison, .zdev, lifecycle, and commits. For audit only, ignore the task-verification JSON contract and follow the supplied textual audit envelope.",
  planner:
    "Plan one advanced task read-only from its snapshot and repository guidance. Stay within approved scope; unresolved product decisions are blockers. Return one JSON object with verdict, summary, plan, and findings. A plan contains approach, normalized repository-relative or absolute checkout paths, and validation; its findings may record supporting observations. A blocker has plan null and at least one finding. Coordination and the implementer own edits and lifecycle work.",
  "advanced-implementer":
    "Implement one advanced task from the supplied plan or rework findings. Load its snapshot and follow repository guidance. Plan paths are expected seams, not an allowlist; include another directly necessary path when its baseline ownership is clear and it stays within the task's semantic boundaries. Block only on ambiguous ownership, a real scope or user decision, or an unavailable prerequisite. Do not block on partial progress or another file. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if necessary; never run derive commands.",
} as const;

const workerProfiles = {
  "routine-implementer": { model: {{ routine_implementer_model }}, effort: {{ routine_implementer_effort }} },
  implementer: { model: {{ implementer_model }}, effort: {{ implementer_effort }} },
  verifier: { model: {{ verifier_model }}, effort: {{ verifier_effort }} },
  planner: { model: {{ planner_model }}, effort: {{ planner_effort }} },
  "advanced-implementer": { model: {{ advanced_implementer_model }}, effort: {{ advanced_implementer_effort }} },
} as const;

type WorkerRole = keyof typeof rolePrompts;

type BatchItem = {
  task_id: string;
  role: WorkerRole;
  prompt: string;
  cwd: string;
};

type BatchResult = {
  task_id: string;
  role: WorkerRole;
  cwd: string;
  status: "completed" | "failed" | "cancelled" | "not-dispatched" | "unconfirmed-stop";
  output?: string;
  code?: number;
  stderr?: string;
  error?: string;
};

type BatchRun = {
  items: BatchItem[];
  workerLimit: number;
  next: number;
  active: Map<string, Promise<void>>;
  settled: BatchResult[];
  results: BatchResult[];
  waiters: Array<() => void>;
  controller: AbortController;
};

const roleType = Type.Union([
  Type.Literal("routine-implementer"),
  Type.Literal("implementer"),
  Type.Literal("verifier"),
  Type.Literal("planner"),
  Type.Literal("advanced-implementer"),
]);

const batchRoleType = Type.Union([
  Type.Literal("routine-implementer"),
  Type.Literal("implementer"),
  Type.Literal("planner"),
  Type.Literal("advanced-implementer"),
]);

const batchItemType = Type.Object({
  task_id: Type.String({ minLength: 1 }),
  role: batchRoleType,
  prompt: Type.String({ minLength: 1 }),
  cwd: Type.String({ minLength: 1, description: "Normalized absolute assigned source worktree path." }),
});

function childArgs(role: WorkerRole, prompt: string): string[] {
  const tools =
    role === "verifier" || role === "planner"
      ? "read,bash,grep,find,ls"
      : "read,bash,edit,write,grep,find,ls";
  const args = [
    "--print",
    "--no-session",
    "--no-extensions",
    "--no-skills",
    "--no-prompt-templates",
    "--tools",
    tools,
    "--append-system-prompt",
    rolePrompts[role],
  ];
  const profile = workerProfiles[role];
  if (profile.model) args.push("--model", profile.model);
  if (profile.effort) args.push("--thinking", profile.effort);
  args.push(prompt);
  return args;
}

export default function (pi: ExtensionAPI) {
  const runs = new Map<string, BatchRun>();
  const resultBase = (item: BatchItem) => ({ task_id: item.task_id, role: item.role, cwd: item.cwd });
  const publish = (run: BatchRun, result: BatchResult) => {
    run.results.push(result);
    run.settled.push(result);
    for (const wake of run.waiters.splice(0)) wake();
  };
  const launch = (run: BatchRun, item: BatchItem) => {
    const child = (async () => {
      try {
        const result = await pi.exec("pi", childArgs(item.role, item.prompt), {
          cwd: item.cwd,
          signal: run.controller.signal,
        });
        publish(run, {
          ...resultBase(item),
          status: run.controller.signal.aborted ? "cancelled" : result.code === 0 ? "completed" : "failed",
          output: result.stdout.trim(),
          code: result.code,
          stderr: result.stderr.trim(),
        });
      } catch (error) {
        publish(run, {
          ...resultBase(item),
          status: run.controller.signal.aborted ? "unconfirmed-stop" : "failed",
          error: error instanceof Error ? error.message : String(error),
        });
      } finally {
        run.active.delete(item.task_id);
      }
    })();
    run.active.set(item.task_id, child);
  };
  const fillAvailableSlots = (run: BatchRun) => {
    while (!run.controller.signal.aborted && run.active.size < run.workerLimit && run.next < run.items.length) {
      launch(run, run.items[run.next++]);
    }
  };
  const takeNext = async (run: BatchRun): Promise<BatchResult> => {
    while (run.settled.length === 0) {
      await new Promise<void>((resolveWait) => run.waiters.push(resolveWait));
    }
    return run.settled.shift()!;
  };
  const response = (runId: string, run: BatchRun, result: BatchResult) => {
    const body = {
      run_id: runId,
      result,
      active: [...run.active.keys()],
      pending: run.items.slice(run.next).map((item) => item.task_id),
    };
    return { content: [{ type: "text", text: JSON.stringify(body) }], details: body };
  };

  pi.registerTool({
    name: "zdev_subagent",
    label: "Zdev Subagent",
    description: "Run one configured Pi worker, or a bounded batch of zdev source workers.",
    parameters: Type.Union([
      Type.Object(
        {
          role: roleType,
          prompt: Type.String({
            description:
              "Installed route-contract path plus compact file paths, snapshot IDs, prior-role result, and boundary.",
          }),
        },
        { additionalProperties: false },
      ),
      Type.Object(
        {
          operation: Type.Literal("start"),
          run_id: Type.String({ minLength: 1 }),
          items: Type.Array(batchItemType, { minItems: 2 }),
          worker_limit: Type.Integer({ minimum: 1 }),
        },
        { additionalProperties: false },
      ),
      Type.Object(
        { operation: Type.Literal("continue"), run_id: Type.String({ minLength: 1 }) },
        { additionalProperties: false },
      ),
      Type.Object(
        { operation: Type.Literal("cancel"), run_id: Type.String({ minLength: 1 }) },
        { additionalProperties: false },
      ),
    ]),
    async execute(_toolCallId, params, signal, _onUpdate, ctx) {
      if ("role" in params) {
        const child = await pi.exec("pi", childArgs(params.role, params.prompt), {
          cwd: ctx.cwd,
          signal,
        });
        if (child.code !== 0) {
          throw new Error(
            [`Pi subagent exited with code ${child.code}.`, child.stdout, child.stderr]
              .filter(Boolean)
              .join("\n"),
          );
        }
        return {
          content: [{ type: "text", text: child.stdout.trim() }],
          details: { role: params.role, code: child.code, stderr: child.stderr.trim() },
        };
      }

      if (params.operation === "start") {
        const items = params.items as BatchItem[];
        if (runs.has(params.run_id)) throw new Error("Batch run_id is already active.");
        if (signal.aborted) throw new Error("Batch start was cancelled before dispatch.");
        if (items.some((item) => !isAbsolute(item.cwd) || resolve(item.cwd) !== item.cwd)) {
          throw new Error("Every batch cwd must be its normalized absolute path.");
        }
        if (new Set(items.map((item) => item.task_id)).size !== items.length) {
          throw new Error("Batch task IDs must be unique.");
        }
        if (new Set(items.map((item) => item.cwd)).size !== items.length) {
          throw new Error("Every batch task must use a distinct assigned worktree path.");
        }
        const run: BatchRun = {
          items,
          workerLimit: Math.min(params.worker_limit, items.length),
          next: 0,
          active: new Map(),
          settled: [],
          results: [],
          waiters: [],
          controller: new AbortController(),
        };
        runs.set(params.run_id, run);
        fillAvailableSlots(run);
        const relayAbort = () => run.controller.abort();
        signal.addEventListener("abort", relayAbort, { once: true });
        const result = await takeNext(run);
        signal.removeEventListener("abort", relayAbort);
        return response(params.run_id, run, result);
      }

      const run = runs.get(params.run_id);
      if (!run) throw new Error("Unknown or completed batch run_id.");
      if (params.operation === "cancel") {
        run.controller.abort();
        for (const item of run.items.slice(run.next)) {
          run.results.push({ ...resultBase(item), status: "not-dispatched" });
        }
        run.next = run.items.length;
        await Promise.allSettled([...run.active.values()]);
        runs.delete(params.run_id);
        const body = { run_id: params.run_id, results: run.results };
        return { content: [{ type: "text", text: JSON.stringify(body) }], details: body };
      }

      if (run.settled.length === 0) fillAvailableSlots(run);
      const relayAbort = () => run.controller.abort();
      signal.addEventListener("abort", relayAbort, { once: true });
      const result = await takeNext(run);
      signal.removeEventListener("abort", relayAbort);
      if (run.active.size === 0 && run.next === run.items.length && run.settled.length === 0) {
        runs.delete(params.run_id);
      }
      return response(params.run_id, run, result);
    },
  });
}
