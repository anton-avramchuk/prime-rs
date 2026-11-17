# Dialog

Port of `p-dialog` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-dialog::Dialog`, `DialogPosition`, `DialogPt` | `dialog/dialog.ts`, `dialog/style/*` |
| CSS | `@primeuix/styles/dialog` — `cargo xtask gen-theme` (`dialogstyle.ts` has no `For PrimeNG` block; position and alignment are inline styles written by the component itself) |
| Demos | `apps/showcase/doc/dialog/{basic,position,modal,withoutmodal,maximizable,responsive,longcontent,template,headless}-doc.ts` |

Spec: `openspec/specs/dialog`.

## Usage

```rust
let visible = RwSignal::new(false);
view! {
    <Button label="Show" on_click=Callback::new(move |_| visible.set(true)) />
    <Dialog visible=visible header="Edit Profile" modal=true style="width: 25rem">
        <p>"Content"</p>
    </Dialog>
}
```

`visible` is a two-way signal: the dialog is closed by the button, `Escape` or (with `dismissable_mask`) a click on the mask, and `false` is written to the signal. The header and footer are set via `header_content` and `footer` (`ViewFn`), `headless` replaces the entire window content. Width by screen width — `breakpoints=vec![("575px".into(), "90vw".into())]`.

## What matches PrimeNG

- Markup and classes: `div.p-dialog-mask` (`p-overlay-mask` when `modal`, `p-dialog-<position>`) > `div.p-dialog.p-component[role=dialog][aria-modal=true]` > `p-resizable-handle`, `p-dialog-header` (`p-dialog-title`, `p-dialog-header-actions` with `p-dialog-maximize-button` and `p-dialog-close-button`), `p-dialog-content`, `p-dialog-footer`; `data-p`. A non-modal dialog also has a mask (with `pointer-events: none`), and the window is its descendant.
- Nine screen positions (mask inline styles are the same as in `dialogstyle.ts`).
- Animations — `p-dialog-enter-active` / `p-dialog-leave-active` and `p-overlay-mask-*-active` classes; the window is removed from the DOM when the leave animation ends.
- Dragging by the header (except the buttons block), `keep_in_viewport`, `min_x`, `min_y`; resizing by the bottom right corner (until dragged, the offset is doubled, as in PrimeNG); maximizing with scroll locking for a non-modal dialog without `block_scroll`; `breakpoints` via `<style>` in `head`.
- `Escape` closes the topmost dialog by `z-index`; a click on the mask itself closes when `dismissable_mask`; `z-index` of the modal window group, mask one below the window; `FocusTrap`; on show, focus goes to the first focusable element of the content, then the footer and header.

## Differences

| Difference | Reason |
|---|---|
| The mask and window are always in `body` (via `Portal`), no `appendTo` | Easier to work around ancestors' `transform`; `Portal` already exists in `prime-dom`. |
| No `@primeuix/motion`, `motionOptions`, `maskMotionOptions`, `transitionOptions` | Animations are PrimeNG CSS classes; a fallback timer (700 ms) removes the window if `animationend` does not arrive. |
| `on_show` is called right after the window enters the DOM, not after the enter animation | Same for `Drawer`; makes no difference for application logic. |
| `on_hide` is called at the start of closing, for any way of closing | In PrimeNG — after the leave animation. |
| Dragging and resizing use window `mousemove`/`mouseup` listeners attached for the duration of the gesture; update rate is the browser's | In PrimeNG the listeners stay for the whole time the dialog is shown; same result. |
| No `closeButtonProps`, `maximizeButtonProps`, `rtl`, `closeTabindex`, `content` template; icons are `ViewFn` | Buttons are `Button` with the same defaults (secondary, text, round), customized via `pt` and icons. |
| No `OverlayService.emitParentDrag` | Needed only for overlays attached to the window. |
| The `overlaysinside` demo is not ported | `Select` has no `appendTo`: its panel would be clipped by the window bounds. |
| The `headless` demo is simplified (no logo and gradient); in `position` the buttons have no arrow icons | No icons in `prime-icons`; `backdrop-blur` replaced by `backdrop-filter` in `mask_style`. |
| No `Bind`/`pt` with context | `pt` is plain additions of classes, style and attributes. |

## Cost

66 allocations per instance in SSR (hidden dialog); contribution to wasm ≈ 684 KB (budget 765 KB in `benches/budgets.toml`), of which ≈ 358 KB is `Button`.

## Not verified

Touch (dragging and resizing are mouse-only, as in PrimeNG), `rtl`, screen readers. Verified in Chrome (browser tests and e2e with a real mouse and keyboard): showing in `body`, buttons, closing via button, Escape and mask, modality and scroll locking, positions, dragging and screen boundary, resizing, maximizing, `breakpoints`, focus and focus trap, two dialogs stacked and re-showing during leave. Screenshot comparison with PrimeNG (`pages.json`) was not run. A dialog opened before hydration is not supported (the server does not send it).
