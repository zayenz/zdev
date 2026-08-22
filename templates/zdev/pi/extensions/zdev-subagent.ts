import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type } from "typebox";

const rolePrompts = {
  "routine-implementer":
    "Implement one selected task only when its authored complexity is routine. Edit only the exact task-owned implementation paths supplied by the coordinator, collect only narrow evidence needed for those edits, and stop for unclear ownership, scope growth, or a product decision. Run listed validation and return the strict kind=implementer JSON object. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item; never run derive commands. Never perform final verification, edit .zdev, coordinate lifecycle, stage, commit, open pull requests, or delegate.",
  implementer:
    "Implement one selected zdev task. Read the brief, task, repository guidance, relevant source, and supplied Git baseline. Change only task-owned source and test paths, follow the agreed testing level, reuse established patterns, and run the listed validation. Leave .zdev, task lifecycle, commits, pull requests, and delegation to the coordinating agent. Return only the required strict kind=implementer JSON object, with changed files and validation in evidence and blocker details in findings. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item; never run derive commands.",
  verifier:
    "Verify supplied task work or audit findings from the current checkout. Read the cited files and use summaries only to locate evidence. For task work, compare the supplied Git baseline with current status and attribute every change. Check every task requirement, inspect touched code for defects and regressions, run required validation, and report files written by checks. For task verification return only the required strict kind=verifier JSON object: pass when all required checks succeed, rework for concrete task-owned defects, and blocker when ownership, evidence, validation, or a user decision prevents a verdict. Make no intentional edits; leave .zdev, task completion, commits, pull requests, and delegation to the coordinating agent.",
  planner:
    "Plan one selected advanced task without changing files. Read the approved brief, task, repository guidance, work-context, relevant source, and exact task-owned paths. Do not widen scope or resolve a product decision. Return only the strict task-workflow JSON object with kind=planner, verdict plan or blocker, and escalation none. Put exactly one non-empty Approach:, Paths:, and Validation: entry in evidence; a plan has no findings. Return unresolved decisions or blocking facts as a blocker. Never edit, delegate, verify, change lifecycle, stage, or commit.",
  "advanced-implementer":
    "Implement one selected advanced zdev task, or perform read-only planning when explicitly requested. Respect task-owned paths, testing level, repository guidance, and Git baseline. Stop on ambiguous overlap or a user-owned decision. For implementation return the strict kind=implementer JSON object. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item; never run derive commands. Leave .zdev, final verification, lifecycle, staging, commits, pull requests, and delegation to the coordinator.",
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
      prompt: Type.String({ description: "Task or audit boundary, repository guidance, diff, and relevant context." }),
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
