# Proposal

## Why

Drawer - the fifth component of wave 3: a panel at the screen edge with a mask, scroll lock and a focus trap. This is the project's first modal overlay: it is the first user of `Portal` and `FocusTrap` from `prime-dom` (ready but without consumers), of the mask and of the `body` scroll lock, which `Dialog` will need next.

## What Changes

- New crate `prime-drawer` (port of `p-drawer` from PrimeNG 21.1.10): `Drawer` - root `div.p-drawer.p-component[role=complementary]` (classes `p-drawer-<position>`, `p-drawer-full`, `p-drawer-open`) with `div.p-drawer-header` (title `div.p-drawer-title`, close button `p-drawer-close-button`), `div.p-drawer-content` and an optional `div.p-drawer-footer`; with `modal` a mask `div.p-drawer-mask.p-overlay-mask` is rendered behind the panel. The panel and mask are moved into `body` via `Portal`.
- Properties: `visible` (two-way `RwSignal<bool>`), `position` (`DrawerPosition`: `Left` by default, `Right`, `Top`, `Bottom`, `Full`), `full_screen`, `modal` (yes), `dismissible` (yes), `close_on_escape` (yes), `closable` (yes), `block_scroll` (no), `auto_z_index` (yes), `base_z_index` (0), `header`, `aria_close_label`, `mask_style`, `style`, `style_class`, `pt`; view parts `header_content`, `footer`, `close_icon`, `headless` (`ViewFn`); callbacks `on_show`, `on_hide`.
- Behaviour: with `visible` the panel and mask appear with the enter animation (`p-drawer-enter-<position>`, the mask `p-overlay-mask-enter-active`); on hiding the leave animations play and only after `animationend` are they removed from the DOM; a click on the mask with `dismissible`, the close button and `Escape` on the topmost window by `z-index` close the panel; `z-index` via `ZGroup::Modal`, the mask one below the panel; with `block_scroll` `p-overflow-hidden` is set on `body` while shown; `Tab` does not leave the panel (`FocusTrap`).
- Showcase page `/drawer` with demos `basic`, `position`, `size`, `fullscreen`, `template`, `headless` (simplified); e2e with a real mouse and keyboard; benchmarks and budgets; `docs/drawer.md`.
- Styles: `drawer` is added to `theme.components` and `primeng_extra`; `xtask gen-theme` learns to recognize the `/** For PrimeNG **/` marker that delimits Drawer's block.
- `prime-dom` gets `block_body_scroll()` / `unblock_body_scroll()` (a nested-call counter, a port of `blockBodyScroll`).
- **Out of scope:** `appendTo` (the panel is always in `body`), `@primeuix/motion` and `motionOptions`/`transitionOptions` (animations are CSS classes), the deprecated `showCloseIcon`, `closeButtonProps` (the close button is a `Button` with the same defaults: secondary, text, rounded), the `content` template (content is `children`), `pt` with context, the full `headless` demo (a profile menu with `Ripple` and `PanelMenu`).

## Capabilities

### New Capabilities
- `drawer`: a modal or inline panel at the screen edge with animation, mask, scroll lock and a focus trap.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-drawer` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`), `web-sys`.
- Changed: `prime-dom` (scroll lock), `xtask` (marker), workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
