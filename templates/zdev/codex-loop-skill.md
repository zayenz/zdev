---
name: __ZDEV_SKILL_NAME__
description: "__ZDEV_SKILL_DESCRIPTION__"
---

Call `get_goal({})` to inspect native goal state. With no unfinished goal, call
`create_goal({ objective: condition })` with the validated area condition and
no invented token budget. If the exact same condition is already active,
continue under it without creating another goal. For the same paused or
budget-limited goal, preserve it and ask the user to resume it through Codex.
After the user resumes, inspect it again and continue under the same goal.
Use `update_goal` only to record a terminal state for the same active goal.
After the same active zdev goal reaches a terminal PASS, call
`update_goal({ status: "complete" })` only when the native goal contract permits
completion.

__ZDEV_NATIVE_AREA_LOOP_BODY__

Use Codex collaboration agents exactly as the embedded one-task contract
requires. The current Codex session remains coordinator. Spawn every role with
`fork_turns="none"` and the compact filesystem-backed message defined by the
installed `zdev-implement` or `zdev-verify` contract.

For `routine-implementer`, {% if routine_implementer_has_model %}pass `model={{ routine_implementer_model }}`{% if routine_implementer_has_effort %} and `reasoning_effort={{ routine_implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For `implementer`, {% if implementer_has_model %}pass `model={{ implementer_model }}`{% if implementer_has_effort %} and `reasoning_effort={{ implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For `advanced-implementer`, {% if advanced_implementer_has_model %}pass `model={{ advanced_implementer_model }}`{% if advanced_implementer_has_effort %} and `reasoning_effort={{ advanced_implementer_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For every fresh verifier, {% if verifier_has_model %}pass `model={{ verifier_model }}`{% if verifier_has_effort %} and `reasoning_effort={{ verifier_effort }}`{% endif %} together with `fork_turns="none"`.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
