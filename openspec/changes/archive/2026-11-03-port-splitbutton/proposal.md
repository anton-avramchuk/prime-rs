# Proposal

## Why

SplitButton is the third component of wave 4: a button with a default action and an adjacent arrow button that opens a list of additional commands. It is assembled from the ready-made `Button` and a popup `Menu` and covers the typical "Save ▾ → Save as…" scenario.

## What Changes

- New crate `prime-splitbutton` (port of `p-splitbutton` from PrimeNG 21.1.10).
- `SplitButton`: root `div.p-splitbutton.p-component` (modifiers `p-splitbutton-raised`, `-rounded`, `-outlined`, `-text`, `-sm`/`-lg`; `data-p-severity`) > default button `p-splitbutton-button` (`Button`) > dropdown button `p-splitbutton-dropdown p-button-icon-only` with an arrow (`aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) > popup menu (`Menu` with `popup`, attached to the root).
- Properties: `model` (`Vec<MenuItem>`), `severity`, `raised`, `rounded`, `text`, `outlined`, `plain`, `size`, `icon`, `icon_pos`, `label`, `button_aria_label`, `expand_aria_label`, `dropdown_icon`, `disabled`, `button_disabled`, `menu_button_disabled`, `tabindex`, `autofocus`, `style`, `style_class`, `menu_style`, `menu_style_class`, `pt`; view `content` (instead of the button label); events `on_click`, `on_dropdown_click`, `on_menu_show`, `on_menu_hide`.
- Behavior: a click on the main button calls `on_click` and closes the menu; a click on the dropdown button calls `on_dropdown_click` and toggles the menu at the root; ↓/↑ on the dropdown button open the menu (without scrolling the page); show and hide change `aria-expanded` and call `on_menu_show`/`on_menu_hide`.
- Showcase page `/splitbutton` with demos `basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`; e2e; benchmarks and budgets; `docs/splitbutton.md`.
- Styles: `splitbutton` is added to `theme.components`.
- **Out of scope:** nested submenus (`nested`): in PrimeNG the menu is a `TieredMenu`, while the project has only a flat `Menu` with groups for now; nesting will arrive together with `TieredMenu` (the last component of wave 4), then the inner `Menu` will be replaced without changing the API. Also out of scope are `tooltip`/`tooltipOptions` (the `use:tooltip` directive is available from outside), `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `Ripple`, `@primeuix/motion`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `splitbutton`: a button with a default action and a dropdown list of commands.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-splitbutton` (`publish = false`); dependencies: `prime-core`, `prime-button`, `prime-menu`, `prime-icons`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
