import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type } from "typebox";

const rolePrompts = {
  implementer:
    "Implement exactly one selected zdev task. Read the brief, task, repository guidance, relevant source, and supplied three-part Git baseline: status with untracked files, staged diff, and unstaged diff. Respect task-owned paths and stop on ambiguous overlap. Follow the brief's testing level, reuse established patterns, make the smallest complete change, and run listed validation. You may edit source and tests and run validation. Do not edit .zd, run zd task done, change task lifecycle state, commit, open a pull request, create durable run state, or delegate.",
  verifier:
    "Verify supplied task work or audit evidence in a fresh read-only context. Treat summaries as context, not evidence. For task work, compare the three-part baseline with current status including untracked files, staged diff, and unstaged diff. Ignore an intervening commit only if its complete diff adds new .zd/<area>/tasks/*.md files, regenerates .zd/<area>/TASKS.md, and changes no other path; otherwise return BLOCKER. Perform separate Spec and Standards passes; compare the same Git state before and after validation; and report writes without restoring them. Begin with PASS, REWORK, or BLOCKER. PASS requires both passes and all required validation. REWORK means a concrete task-owned defect or task-owned validation write. BLOCKER means ambiguous ownership, unavailable required evidence or validation, or a user-owned decision. Only optional checks may be limitations. Return classified findings. Do not edit files or .zd, complete the task, create tasks, commit, open a pull request, create durable run state, or delegate.",
} as const;

export default function (pi: ExtensionAPI) {
  pi.registerTool({
    name: "zdev_subagent",
    label: "Zdev Subagent",
    description: "Run a fresh ephemeral Pi implementer or read-only verifier for one bounded zdev handoff.",
    parameters: Type.Object({
      role: Type.Union([Type.Literal("implementer"), Type.Literal("verifier")]),
      prompt: Type.String({ description: "Area brief, task, repository guidance, diff, and relevant context." }),
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
      if (ctx.model) args.push("--model", `${ctx.model.provider}/${ctx.model.id}`);
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
