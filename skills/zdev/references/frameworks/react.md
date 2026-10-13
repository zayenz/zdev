# React

Apply these rules with [coding guidance](../coding.md). Use [web guidance](../web.md)
for browser behavior and [React Router guidance](react-router.md) for affected
framework-mode routes.

## State and effects

Avoid `useEffect` unless there is a concrete external system to synchronize with.
Compute derived values during rendering; handle user actions in event handlers;
load route data through the framework. Keep one owner for each state value.
Intentional editable drafts may differ from their source, but copying props into
state merely to synchronize them adds another source of truth. Reset state through
component identity when that matches the intended lifecycle.

For a necessary effect, identify the external system and values controlling the
synchronization. Include real dependencies, provide cleanup, and prevent stale
asynchronous work from applying results. Correct setup and cleanup must tolerate
remounting. Fix dependency problems at their source rather than suppressing hook
lint rules. A custom hook must not merely conceal an unnecessary effect.

Keep rendering pure and hooks at the top level of components or custom hooks.
Use stable domain identity for list keys when items can move, appear, or disappear.
Keep component boundaries aligned with state ownership and useful responsibilities.

## Performance and validation

Address actual request waterfalls and unnecessary shipped code before detailed
render optimizations. Parallelize independent reads where dependencies and failure
handling permit. Use memoization for demonstrated expensive work or an established
identity contract, not as routine decoration. Scope Next.js and server-component
advice to projects and responsibilities that actually use them.

Run existing formatting, hook linting, and type checks. For affected behavior,
exercise relevant interactions, pending/error states, navigation, and cleanup.
Choose useful evidence through common completion guidance rather than adding a
separate UI test requirement.

References: [React effect guidance](https://react.dev/learn/you-might-not-need-an-effect),
[Vercel React practices](https://vercel.com/blog/introducing-react-best-practices).
