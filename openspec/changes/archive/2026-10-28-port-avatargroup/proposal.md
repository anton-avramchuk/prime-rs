# Proposal

## Why

AvatarGroup combines several Avatars into an overlapping group. In PrimeNG it is a separate package (`primeng/avatargroup`), so by the rule "one component — one crate" it gets its own crate; it was deliberately postponed when Avatar was ported (`docs/avatar.md`).

## What Changes

- A new crate `prime-avatargroup` with the `AvatarGroup` component: child content, `style`, `style_class`, `pt` (`root`).
- Markup and classes as in PrimeNG 21.1.10: the root `<div class="p-avatar-group p-component">`; the placement and overlap of the avatars are set by the theme CSS (`.p-avatar-group`).
- The `avatargroup` demo on the `/avatar` page in the showcase, e2e, benchmarks, budgets.
- **Not included:** automatic collapsing of extra avatars into "+N" (PrimeNG has no such feature either — "+2" is done manually).

## Capabilities

### New Capabilities
- `avatargroup`: the AvatarGroup component — markup, classes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-avatargroup` (`publish = false`); the only dependency is `prime-rs-core`.
- Changes to `showcase` (the Avatar page), `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`, `docs/avatar.md`.
