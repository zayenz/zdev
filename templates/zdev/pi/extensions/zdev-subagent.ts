import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type } from "typebox";

const rolePrompts = {
  "routine-implementer":
    "Implement one tightly specified routine task. Load its snapshot, stay within task-owned paths, make the smallest complete change, and run listed validation. Block on unclear ownership, scope growth, or a product decision. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if the work unexpectedly needs one; never run derive commands.",
  implementer:
    "Implement one selected task. Load its snapshot, follow repository guidance and the task's testing level, stay within task-owned paths, and run validation. Block on ambiguous ownership or a user decision. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if necessary; never run derive commands.",
  verifier:
    "For task verification only, verify one task read-only. Load its snapshot, use the implementer summary only to locate evidence, check the whole task, attribute every change, and run required validation. Return one JSON object with verdict, summary, findings, and escalation. Use pass with no findings for success, rework with at least one finding for a task-owned defect or write, and blocker for ambiguous ownership, missing evidence, or a user decision. Set escalation to none, except that rework may request advanced-implementer. Name each validation-written task-owned file as validation_write: <repository-relative path>. Coordination owns snapshot comparison, .zdev, lifecycle, and commits. For audit only, ignore the task-verification JSON contract and follow the supplied textual audit envelope.",
  planner:
    "Plan one advanced task read-only from its snapshot and repository guidance. Stay within approved scope; unresolved product decisions are blockers. Return one JSON object with verdict, summary, plan, and findings. A plan contains approach, normalized repository-relative or absolute checkout paths, and validation; its findings may record supporting observations. A blocker has plan null and at least one finding. Coordination and the implementer own edits and lifecycle work.",
  "advanced-implementer":
    "Implement one advanced task from the supplied plan or rework findings. Load its snapshot, respect task-owned paths and repository guidance, and block on ambiguous ownership or a user decision. Return one JSON object with schema_version: 1, kind: \"implementer\", area, task_id, verdict (ready or blocker), summary, string arrays evidence and findings, and escalation: \"none\". Put changed files and validation in evidence. Coordination owns .zdev, verification, lifecycle, and commits. Load the split section only if necessary; never run derive commands.",
} as const;

const workerProfiles = {
  "routine-implementer": { model: {{ routine_implementer_model }}, effort: {{ routine_implementer_effort }} },
  implementer: { model: {{ implementer_model }}, effort: {{ implementer_effort }} },
  verifier: { model: {{ verifier_model }}, effort: {{ verifier_effort }} },
  planner: { model: {{ advanced_implementer_model }}, effort: {{ advanced_implementer_effort }} },
  "advanced-implementer": { model: {{ advanced_implementer_model }}, effort: {{ advanced_implementer_effort }} },
} as const;

export default function (pi: ExtensionAPI) {
  pi.registerTool({
    name: "zdev_subagent",
    label: "Zdev Subagent",
    description: "Run a configured Pi worker for one zdev task or read-only audit.",
    parameters: Type.Object({
      role: Type.Union([
        Type.Literal("routine-implementer"),
        Type.Literal("implementer"),
        Type.Literal("verifier"),
        Type.Literal("planner"),
        Type.Literal("advanced-implementer"),
      ]),
      prompt: Type.String({
        description:
          "Installed route-contract path plus compact file paths, snapshot IDs, prior-role result, and boundary.",
      }),
    }),
    async execute(_toolCallId, params, signal, _onUpdate, ctx) {
      const tools =
        params.role === "verifier" || params.role === "planner"
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
        rolePrompts[params.role],
      ];
      const profile = workerProfiles[params.role];
      if (profile.model) args.push("--model", profile.model);
      if (profile.effort) args.push("--thinking", profile.effort);
      args.push(params.prompt);

      const child = await pi.exec("pi", args, { cwd: ctx.cwd, signal });
      if (child.code !== 0) {
        throw new Error(
          [`Pi subagent exited with code ${child.code}.`, child.stdout, child.stderr]
            .filter(Boolean)
            .join("\n"),
        );
      }
      return {
        content: [
          {
            type: "text",
            text: child.stdout.trim(),
          },
        ],
        details: { role: params.role, code: child.code, stderr: child.stderr.trim() },
      };
    },
  });
}
