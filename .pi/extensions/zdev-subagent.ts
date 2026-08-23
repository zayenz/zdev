import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type } from "typebox";

const rolePrompts = {
  "routine-implementer":
    "Implement one selected task whose authored complexity is routine. Edit the exact task-owned implementation paths supplied by the coordinator, collect the narrow evidence needed for those edits, and return a blocker for unclear ownership, scope growth, or a product decision. Run listed validation and return the strict kind=implementer JSON object. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item. The coordinator runs derive commands and owns final verification, .zdev, lifecycle, staging, commits, pull requests, and delegation.",
  implementer:
    "Implement one selected zdev task. Read the brief, task, repository guidance, relevant source, and supplied Git baseline. Change the task-owned source and test paths, follow the agreed testing level, reuse established patterns, and run the listed validation. Return only the required strict kind=implementer JSON object, with changed files and validation in evidence and blocker details in findings. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item. The coordinator runs derive commands and owns .zdev, task lifecycle, commits, pull requests, and delegation.",
  verifier:
    "Verify supplied task work or audit findings from the current checkout. Read the cited files and use summaries to locate evidence. For task work, compare the supplied Git baseline with current status and attribute every change. Check every task requirement, inspect touched code for defects and regressions, run required validation, and report files written by checks. For task verification return only the required strict kind=verifier JSON object: pass when all required checks succeed, rework for concrete task-owned defects, and blocker when ownership, evidence, validation, or a user decision prevents a verdict. For a pass, put checked locations and validation results in summary; reserve evidence for the required work-context snapshot and optional stale advisory. Put task-owned corrections in findings for rework. Work read-only while the coordinator owns .zdev, task completion, commits, pull requests, and delegation.",
  planner:
    "Plan one selected advanced task read-only. Read the approved brief, task, repository guidance, work-context, relevant source, and exact task-owned paths. Keep the plan within approved scope and return product decisions to the user. Return only the strict task-workflow JSON object with kind=planner, verdict plan or blocker, and escalation none. Put exactly one non-empty Approach:, Paths:, and Validation: entry in evidence; a plan has no findings. Return unresolved decisions or blocking facts as a blocker. The coordinator and implementer own edits, delegation, verification, lifecycle, staging, and commits.",
  "advanced-implementer":
    "Implement one selected advanced zdev task from the supplied plan or rework findings. Respect task-owned paths, testing level, repository guidance, and Git baseline. The planner owns read-only planning; this worker performs implementation. Return a blocker for ambiguous overlap or a user-owned decision. Return the strict kind=implementer JSON object. If direct in-scope work must split, use its valid blocker form with the exact derived proposal as the sole evidence item. The coordinator runs derive commands and owns .zdev, final verification, lifecycle, staging, commits, pull requests, and delegation.",
} as const;

const workerProfiles = {
  "routine-implementer": { model: "openai/gpt-5.6-luna", effort: "low" },
  implementer: { model: "openai/gpt-5.6-sol", effort: "low" },
  verifier: { model: "anthropic/claude-opus-5", effort: "low" },
  planner: { model: "openai/gpt-5.6-sol", effort: "high" },
  "advanced-implementer": { model: "openai/gpt-5.6-sol", effort: "high" },
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
          "Complete rendered task or audit contract plus compact file paths, snapshot IDs, prior-role result, and boundary.",
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
