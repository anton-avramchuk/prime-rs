# Proposal

## Why

SpeedDial is the eighth component of wave 4: a floating action button that reveals a set of quick actions on press (in a line, a circle, a semicircle or a quarter circle). It uses the existing `Button`, `Tooltip` and item model, like `Dock`, but adds layout geometry and direction-based keyboard navigation.

## What Changes

- New crate `prime-speeddial` (port of `p-speeddial` from PrimeNG 21.1.10).
- `SpeedDialItem` (instead of PrimeNG's `MenuItem`, without a router): `id`, `label`, `icon` (`ViewFn`), `disabled`, `visible` (default `true`), `tabindex`, `command` (`Callback<SpeedDialCommand>`), `style_class`.
- `SpeedDial`: root `div.p-speeddial.p-component.p-speeddial-<type>` (`p-speeddial-direction-<direction>` except circle, `p-speeddial-open`, `p-disabled`; inline `flex-direction` and alignment by direction) > button `button.p-button.p-button-icon-only.p-speeddial-button.p-button-rounded` (`p-speeddial-rotate`, `aria-expanded`, `aria-haspopup`, `aria-controls`, `aria-label`, `aria-labelledby`) with a "plus" icon > list `ul.p-speeddial-list[role=menu]` (`id`, `tabindex=-1`, `aria-activedescendant`) > `li.p-speeddial-item[role=menuitem]` (`p-hidden`, `p-focus`, `data-p-active`, `transition-delay` and layout point) > `p-speeddial-action` button (secondary, round, small) with an icon; with `mask` and shown — `div.p-speeddial-mask.p-overlay-mask`.
- Layout: `type` (`Linear`, `Circle`, `SemiCircle`, `QuarterCircle`), `direction` (eight values, default `Up`), `radius` (default `20 × item count`), `transition_delay` (30 ms; when showing the delay grows toward the end of the list, when hiding — toward the start); `--item-diff-x/y` offsets from the difference between button and item sizes.
- Behavior: clicking the button toggles display (`on_click`, `on_visible_change`, `on_show`, `on_hide`); clicking an item runs `command` and hides; clicking outside the component hides (`hide_on_click_outside`); `Escape` hides and returns focus to the button; on the button the arrows open the list and move through items (↓/← forward, ↑/→ back), in the list the arrows follow the layout direction, `Home`/`End`, `Enter`/`Space` activate the focused item; disabled items are skipped.
- Properties: `model`, `visible` (controlled display), `id`, `style`, `style_class`, `direction`, `transition_delay`, `type`, `radius`, `mask`, `disabled`, `hide_on_click_outside`, `button_style`, `button_class`, `mask_style`, `mask_class`, `show_icon`, `hide_icon` (`ViewFn`), `rotate_animation`, `aria_label`, `aria_labelledby`, `tooltip_options` (`TooltipOptions` for item tooltips), `button_severity`, `pt`; views `button_content`, `item_content`, `icon_content`.
- Showcase page `/speeddial` with the demos `linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`; e2e; benchmarks and budgets; `docs/speeddial.md`.
- Styles: `speeddial` is added to `theme.components`.
- **Out of scope:** item `routerLink`, `buttonProps` as a whole (`button_severity` is kept), `Ripple`, `Bind`, `pTemplate`, `unstyled`, `pi pi-…` icon classes (icons are `ViewFn`).

## Capabilities

### New Capabilities
- `speeddial`: a floating button with an expanding set of actions and different layouts.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-speeddial` (`publish = false`); dependencies: `prime-core`, `prime-button`, `prime-tooltip`, `prime-icons` (plus), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places: `prime-speeddial`).
