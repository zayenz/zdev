---
name: zdev-planner
description: Produce the one read-only plan required before an advanced zdev task is edited
tools: read, grep, bash
blocking: true
output:
  type: object
  additionalProperties: false
  required: [verdict, summary, plan, findings]
  properties:
    verdict:
      type: string
      enum: [plan, blocker]
    summary:
      type: string
      minLength: 1
    plan:
      anyOf:
        - type: "null"
        - type: object
          additionalProperties: false
          required: [approach, paths, validation]
          properties:
            approach:
              type: string
              minLength: 1
            paths:
              type: array
              minItems: 1
              items:
                type: string
                minLength: 1
            validation:
              type: array
              minItems: 1
              items:
                type: string
                minLength: 1
    findings:
      type: array
      items:
        type: string
        minLength: 1
{% if advanced_implementer_has_model %}model: {{ advanced_implementer_model }}
{% endif %}{% if advanced_implementer_has_effort %}thinking-level: {{ advanced_implementer_effort }}
{% endif -%}
---

Plan one advanced task read-only from its snapshot and repository guidance.
Stay within approved scope; unresolved product decisions are blockers.

Return one JSON object with `verdict`, `summary`, `plan`, and `findings`. A plan
contains `approach`, normalized repository-relative or absolute checkout
`paths`, and `validation`; its findings may record supporting observations. A
blocker has `plan: null` and at least one finding. Coordination and the
implementer own all edits and lifecycle work.
