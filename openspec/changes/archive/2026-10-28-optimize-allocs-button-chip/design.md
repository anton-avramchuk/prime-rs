# Design

## Context

`render-bench`: Button 19, full Button 45, Chip 10, removable Chip 35 allocations per instance. The main items: `Arc<Props>` + `Arc<Pt>` (2), `NodeRef` (arena + signal), `.into_any()` on every block, `Pt::new()` and temporary strings in Chip's `style_of`.

## Decisions

### D1. `Arc` only in the reactive branch
`Props` and `Pt` live on the stack up to the fork. The static branch borrows them; the reactive one wraps them in `Arc` for closures. The click handler of a static button gets `off = props.is_disabled()` instead of `Arc<Props>`.

### D2. `NodeRef` only on the client
Needed for autofocus and ripple (`setup_client`), which exist only on `wasm32`. The attribute is attached by the spread `{..node}`: on `wasm32` it is `node_ref(...)`, on the native target — `()`.

### D3. No `.into_any()`
The Button indicator is `EitherOf4`; the label, badge, image, icon, label and remove element of Chip are `Option<impl IntoView>`/`Either`.

### D4. Chip style
`Pt::merge_style_parts(&[...])` joins the parts into one string; without `style`, `pt.style` and in the visible state it returns `None` without allocations.

## Risks / Trade-offs

- [Two forms of the click handler] → covered by SSR, parity and browser tests.

## Migration Plan

Internal refactoring; rollback — `git revert`.

## Open Questions

None.
