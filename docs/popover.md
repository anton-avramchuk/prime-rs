# Popover

Port of `p-popover` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-popover::Popover`, `PopoverRef`, `PopoverPt` | `popover/popover.ts`, `popover/style/*` |
| CSS | `@primeuix/styles/popover` — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/popover/{basic,target,selectdata,template}-doc.ts` |

Spec: `openspec/specs/popover`.

## Usage

The panel is controlled by `PopoverRef` (`Copy`), the counterpart of `op.toggle($event)` in PrimeNG. Create it in advance, pass it to `Popover` and call it from the trigger's handlers:

```rust
let op = PopoverRef::new();
view! {
    <button on:click=move |e| op.toggle(&e)>"Share"</button>
    <Popover popover_ref=op>"Content"</Popover>
}
```

The alignment target is the event's `currentTarget`; pass a different target to `show_at(&el)` / `toggle_at(&el)`. `hide()` hides the panel, `visible()` reads the state (it can be used for the trigger's `aria-expanded`).

## What matches PrimeNG

- Markup: `div.p-popover.p-component[role=dialog][aria-modal=true]` > `div.p-popover-content`; `aria-label`, `aria-labelledby`, `id`, `style`, `style_class`, `pt`.
- Alignment to the target (`align_overlay`, as in `Select`), arrow shift `--p-popover-arrow-left`, `p-popover-flipped` and `data-p-popover-flipped` when placed above the target, overlay group `z-index` (`auto_z_index`, `base_z_index`).
- Closing: a click outside the panel and the target with `dismissable`, `Escape`, window `resize` (not on touch devices), a scroll of the target's ancestor; a repeated `toggle` with a different target moves the panel to the new target.
- `focus_on_show`: the first element with `autofocus` gets focus; `on_show` and `on_hide`.
- Window listeners are attached only while shown and removed on hide and on unmount.

## Differences

| Difference | Reason |
|---|---|
| The panel is rendered in its own place in the markup, there is no `appendTo` | `Select` works the same way; an ancestor with `position: relative` is needed, and `overflow: hidden` on the ancestor will clip the panel. |
| Control is via `PopoverRef`, not an instance reference (`#op`) | Leptos has no template references to a component. |
| No show and hide animation (`@primeuix/motion`) | The panel appears and disappears instantly; `motionOptions` and `*TransitionOptions` are not ported. |
| No `OverlayService` | A click in a nested overlay whose panel lies outside the `Popover` DOM subtree will close the outer panel. The `Select` panel lies inside its root, so such a click is considered inner. |
| On iOS closing is on `click`, not `touchstart` | One event type for everyone. |
| No `aria-expanded` / `aria-controls` on the trigger | The trigger is an arbitrary user element; `aria-expanded` is set from `op.visible()`. |
| Content is `children`, there is no `content` template with `closeCallback` | Closing from the content is a call to `op.hide()`. |
| The `datatable` demo is not ported; the `share-alt`, `copy`, `users`, `image` icons are missing from `prime-icons` | DataTable is needed; buttons without icons, avatars are initials. |
| No `Bind`/`pt` with context | `pt` is simple additions of classes, style and attributes. |

## Cost

33 allocations per instance in SSR (hidden panel); contribution to wasm ≈ 291 KB (budget 325 KB in `benches/budgets.toml`).

## Not verified

Touch (`touchstart`, `resize` on a touch device), `rtl` and screen readers. Verified: markup, show and hide, click outside and inside, Escape, target change, explicit target, `autofocus`, flip above the target at the bottom edge, closing on an ancestor scroll and removal of listeners in Chrome (browser tests and e2e). Screenshot comparison with PrimeNG (`pages.json`) was not run.
