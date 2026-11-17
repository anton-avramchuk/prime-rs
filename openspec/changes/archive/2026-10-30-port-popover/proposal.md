# Proposal

## Why

Popover is the fourth component of wave 3: a floating panel with arbitrary content bound to a trigger element. It is the second overlay of the project after `Select`; it exercises imperative showing (`toggle`/`show`/`hide` from an event and a target), which `Menu`, `Tooltip`, `Dialog` and `Drawer` will need later.

## What Changes

- New crate `prime-popover` (port of `p-popover` from PrimeNG 21.1.10): `Popover` — `div.p-popover.p-component[role=dialog]` > `div.p-popover-content` with `children` content. Shown and hidden through the control object `PopoverRef` (`Copy`): `toggle(&event)`, `toggle_at(&event, target)`, `show(&event)`, `show_at(&event, target)`, `hide()`, `visible()`; created with `PopoverRef::new()` and passed to `Popover` via the `popover_ref` property.
- `Popover` properties: `popover_ref`, `dismissable` (yes), `focus_on_show` (yes), `auto_z_index` (yes), `base_z_index` (0), `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `pt`, `on_show` / `on_hide` callbacks.
- Behaviour: the alignment target is the event's `currentTarget` or an explicit `target`; the panel is positioned by `prime_dom::align_overlay`, the arrow is shifted via the `--p-popover-arrow-left` property, when placed above the target `data-p-popover-flipped` and `p-popover-flipped` are set; a click outside the panel and target with `dismissable`, `Escape`, a window resize (not on touch devices) and a scroll of the target's ancestor hide the panel; a repeated `toggle` with a different target hides the panel and shows it at the new one; on show the first element with `autofocus` gets focus; `z-index` by `ZGroup::Overlay`; the trigger does not get `aria-expanded`/`aria-controls` (see "Out of scope").
- Showcase page `/popover` with demos `basic`, `target`, `selectdata`, `template` (closing via a `close` callback); `datatable` — not ported (DataTable is needed); e2e with a real mouse and keyboard; benchmarks and budgets; `docs/popover.md`.
- Styles: `popover` is added to `theme.components`.
- **Out of scope:** `appendTo` (the panel is always inside the root, as in `Select`), the `OverlayService` click bus between overlays, `@primeuix/motion` animations (the panel has no transition), `MotionOptions`, the `datatable` demo, automatic `aria-expanded`/`aria-controls` on the trigger (the trigger is an arbitrary user element; it sets the attributes itself from `visible()`), `pt` with context.

## Capabilities

### New Capabilities
- `popover`: a floating panel at a trigger element with imperative show and close.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-popover` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `web-sys` (`Element`, `HtmlElement`, `MouseEvent`, `KeyboardEvent`).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
