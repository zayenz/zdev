import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type } from "typebox";

const rolePrompts = {
  implementer:
    "Implement one selected zdev task. Read the brief, task, repository guidance, relevant source, and supplied Git baseline. Change only task-owned source and test paths, follow the agreed testing level, reuse established patterns, and run the listed validation. Leave .zdev, task lifecycle, commits, pull requests, and delegation to the coordinating agent. Return changed files, validation results, and blockers.",
  verifier:
    "Verify supplied task work or audit findings from the current checkout. Read the cited files and use summaries only to locate evidence. For task work, compare the supplied Git baseline with current status and attribute every change. Check every task requirement, inspect touched code for defects and regressions, run required validation, and report files written by checks. Begin with PASS, REWORK, or BLOCKER. Use PASS when all required checks succeed, REWORK for concrete task-owned defects, and BLOCKER when ownership, evidence, validation, or a user decision prevents a verdict. Make no intentional edits; leave .zdev, task completion, commits, pull requests, and delegation to the coordinating agent.",
} as const;

const workerProfiles = {
  implementer: { model: {{ implementer_model }}, effort: {{ implementer_effort }} },
  verifier: { model: {{ verifier_model }}, effort: {{ verifier_effort }} },
} as const;

export default function (pi: ExtensionAPI) {
  pi.registerTool({
    name: "zdev_subagent",
    label: "Zdev Subagent",
    description: "Run a Pi implementer or verifier for one zdev task or read-only audit.",
    parameters: Type.Object({
      role: Type.Union([Type.Literal("implementer"), Type.Literal("verifier")]),
      prompt: Type.String({ description: "Task or audit boundary, repository guidance, diff, and relevant context." }),
    }),
    async execute(_toolCallId, params, signal, _onUpdate, ctx) {
      const tools =
        params.role === "implementer"
          ? "read,bash,edit,write,grep,find,ls"
          : "read,bash,grep,find,ls";
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
            text: [`stdout:\n${child.stdout.trim()}`, `stderr:\n${child.stderr.trim()}`].join("\n\n"),
          },
        ],
        details: { role: params.role, code: child.code },
      };
    },
  });
}
