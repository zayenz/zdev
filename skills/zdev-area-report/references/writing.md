# Writing and evidence

Write calmly, precisely and with enough context for a technically interested
reader who has not followed the branch. Start with the concrete reason for
the report and its main judgment. Explain an unfamiliar term at first use.
Introduce the purpose of a mechanism before its implementation. Use a real
example to carry an abstraction when possible.

Prefer direct paragraphs, sentence-case headings and tables for genuinely
parallel comparisons. Let the argument determine the number of sections.
Keep useful detail in the body and put long inventories behind a disclosure.
Source paths support an explanation; they do not replace it.

Group related work by the behavior or design question it explains, retaining
declared slices where useful. A report need not reproduce the full task register.
For a diagram, state the question it answers, label the relationships and explain
the conclusion in text. Use a small inline SVG only when it helps that explanation.

Put selected supporting tables, excerpts or code in an in-page appendix when
readers need them to judge a claim without the checkout. Include enough context
to interpret them; avoid bulk task or diff dumps. If evidence is collapsed, check
that links to it make the destination visible to the reader.

## Evidence that changes the conclusion

For a measurement, identify the workload, measured revision, baseline,
configuration, unit and boundary. Give absolute values and deltas alongside
ratios. Where repetition matters, state sample count and spread. A 60% increase
from 0.1 to 0.16 ms adds 0.06 ms; assess its effect at the actual invocation
rate. Conversely, repeated leaks and scaling defects matter even if one small
run is cheap. Use materiality appropriate to the area rather than importing a
fixed percentage or byte threshold from another report.

Separate live owned bytes, allocator accounting, process RSS and peak RSS.
Subtraction across independent processes is not an allocation breakdown. A
source-header byte count is not compiler time or executable size. A timeout
with the same status is not equal solver performance. An input rejected by both
implementations is not necessarily a regression. A failed historical budget
is not automatically an exhibited material defect.

Label source inspection, reproduced behaviour, historical experiments and
inference distinctly. Explain whether older evidence applies after a repair.
State missing comparisons and unavailable checks where they affect the claim.
Record the user's accepted tradeoffs rather than reopening them by accident.

For findings, name the trigger, location, observable consequence and bounded
correction. Keep a plausible hazard as an open question until the evidence
supports something stronger. For another system's design, cite primary
documentation, papers or source and explain which assumptions match this area.

## Editing pass

Read the design explanation without its validation material. The reader should
still understand the problem, the solution and a normal usage path. Check API
examples against the reviewed source; explicitly label proposed interfaces and
illustrative pseudocode so they cannot be mistaken for working examples.

Remove promotional claims, generic openings, vague attributions and conclusions
that only repeat the introduction. Replace ornamental verbs with direct ones.
Use specific nouns rather than cycling through synonyms for the same object.
Avoid forced groups of three, canned contrasts, em dashes, decorative emphasis
and curly quotation marks. Vary sentence length according to the thought.

Keep authored judgment, but give its reason. Do not flatten a mixed result into
either praise or a list of warnings. End with concrete next steps and the
evidence needed to decide them. The reader should be able to understand the
assessment without opening every source link or reading the original chat.
