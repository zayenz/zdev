---
name: __ZDEV_SKILL_NAME__
description: "Continue a zdev area through native Codex goals, one independently verified task and commit at a time. Use only for an explicit $__ZDEV_SKILL_NAME__ invocation."
---

Call `get_goal({})` to inspect native goal state. With no unfinished goal, call
`create_goal({ objective: condition })` with the validated area condition and
no invented token budget. If the exact same condition is already active,
continue under it without creating another goal. Codex exposes no
model-callable resume operation: for the same paused or budget-limited goal,
leave it unchanged, return `BLOCKER`, and say that the user must resume it
through the harness. Never use `update_goal` to replace or retarget a goal.
After the same active zdev goal reaches a terminal PASS, call
`update_goal({ status: "complete" })` only when the native goal contract permits
completion.

__ZDEV_NATIVE_AREA_LOOP_BODY__

Use Codex collaboration agents exactly as the embedded one-task contract
requires. The current Codex session remains coordinator.

For `routine-implementer`, {% if routine_implementer_has_model %}pass `model={{ routine_implementer_model }}`{% if routine_implementer_has_effort %} and `reasoning_effort={{ routine_implementer_effort }}`{% endif %}.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For `implementer`, {% if implementer_has_model %}pass `model={{ implementer_model }}`{% if implementer_has_effort %} and `reasoning_effort={{ implementer_effort }}`{% endif %}.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For `advanced-implementer`, {% if advanced_implementer_has_model %}pass `model={{ advanced_implementer_model }}`{% if advanced_implementer_has_effort %} and `reasoning_effort={{ advanced_implementer_effort }}`{% endif %}.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
For every fresh verifier, {% if verifier_has_model %}pass `model={{ verifier_model }}`{% if verifier_has_effort %} and `reasoning_effort={{ verifier_effort }}`{% endif %}.{% else %}leave model and reasoning effort unset so they inherit.{% endif %}
