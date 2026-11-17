# Proposal

## Why

Dialog is the sixth component of wave 3 and PrimeNG's main modal component: a window over the page with a header, content and footer that can be dragged, resized and maximized to full screen. It is built on the same facilities as `Drawer` (`Portal`, mask, `block_body_scroll`, `FocusTrap`, `ZGroup::Modal` layers) and adds pointer handling (dragging and resizing) — which `Toast` and `Menu` will need later.

## What Changes

- New crate `prime-dialog` (port of `p-dialog` from PrimeNG 21.1.10): `Dialog` — mask `div.p-dialog-mask` (with `p-overlay-mask` when `modal` and `p-dialog-<position>` for the position) > `div.p-dialog.p-component[role=dialog][aria-modal=true]` with `div.p-resizable-handle` (when `resizable`), `div.p-dialog-header` (title `span.p-dialog-title`, `div.p-dialog-header-actions` with the `p-dialog-maximize-button` and `p-dialog-close-button` buttons), `div.p-dialog-content` and an optional `div.p-dialog-footer`. The mask and window are rendered into `body` via `Portal`.
- Properties: `visible` (two-way `RwSignal<bool>`), `header`, `position` (`DialogPosition`: `Center` by default, `Top`, `Bottom`, `Left`, `Right`, `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`), `modal` (no), `dismissable_mask` (no), `close_on_escape` (yes), `closable` (yes), `draggable` (yes), `resizable` (yes), `maximizable` (no), `keep_in_viewport` (yes), `min_x`, `min_y`, `show_header` (yes), `block_scroll` (no), `focus_on_show` (yes), `focus_trap` (yes), `auto_z_index` (yes), `base_z_index` (0), `breakpoints` (`Vec<(String, String)>`: screen width → window width), `role` (`dialog`), `close_aria_label`, `content_style`, `content_style_class`, `mask_style`, `mask_style_class`, `style`, `style_class`, `pt`; views `header_content`, `footer`, `close_icon`, `maximize_icon`, `minimize_icon`, `headless` (`ViewFn`); callbacks `on_show`, `on_hide`, `on_resize_init`, `on_resize_end`, `on_drag_end`, `on_maximize`.
- Behavior: the window appears and is removed with animation (`p-dialog-enter-active` / `p-dialog-leave-active` on the window, `p-overlay-mask-*-active` on the modal mask) and stays in the DOM until `animationend`; dragging by the header (except buttons) limited to the screen area; resizing by the bottom right corner; the maximize button toggles `p-dialog-maximized` and locks page scrolling; a mask click with `dismissable_mask` and `closable`, the close button and `Escape` on the topmost window by `z-index` close the dialog; focus goes to the first focusable element of the content (then the footer and header); with `breakpoints`, `@media` rules setting the window width are added to `head`.
- Showcase page `/dialog` with demos `basic`, `position`, `modal`, `withoutmodal`, `maximizable`, `responsive`, `longcontent`, `template`, `headless` (simplified); e2e with a real mouse and keyboard; benchmarks and budgets; `docs/dialog.md`.
- Styles: `dialog` is added to `theme.components`.
- **Out of scope:** `appendTo` (the window is always in `body`), `@primeuix/motion` and `motionOptions`/`maskMotionOptions`/`transitionOptions` (animations are CSS classes), `closeButtonProps` and `maximizeButtonProps` (buttons are `Button` with the same defaults), `OverlayService.emitParentDrag` (needed only by nested overlays), `rtl`, the `content` template (content is `children`), `pt` with context, the `overlaysinside` demo (`Select` has no `appendTo`: its panel would be clipped by the window bounds), the deprecated `closeIcon`/`maximizeIcon`/`minimizeIcon` as CSS class names.

## Capabilities

### New Capabilities
- `dialog`: a window over the page with a mask, dragging, resizing, maximizing, animation and a focus trap.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-dialog` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`, `windowmaximize`, `windowminimize`), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
