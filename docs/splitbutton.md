# SplitButton

Port of `p-splitbutton` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-splitbutton::SplitButton`, `SplitButtonPt` | `packages/primeng/src/splitbutton/splitbutton.ts`, `splitbutton/style/splitbuttonstyle.ts` |
| CSS | `@primeuix/styles/splitbutton` and the Aura preset — `cargo xtask gen-theme` (no `/* For PrimeNG */` block); buttons and menu take the `button` and `menu` styles |
| Demos | `apps/showcase/doc/splitbutton/*-doc.ts` (`basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`) |

Spec: `openspec/specs/splitbutton`.

## What matches PrimeNG

- Markup and classes: `div.p-splitbutton.p-component` (`p-splitbutton-raised`, `-rounded`, `-outlined`, `-text`, `-sm`/`-lg`, `data-p-severity`) > main button `button.p-button.p-splitbutton-button` > dropdown button `button.p-button.p-splitbutton-dropdown.p-button-icon-only` > popup menu.
- `severity`, `text`, `outlined`, `plain`, `size` are passed to both buttons; `raised` and `rounded` act via root classes.
- Click on the main button: `on_click`, then closing the menu. Click on the arrow: `on_dropdown_click`, then toggling the menu; ↓ and ↑ on the arrow do the same and cancel the default action.
- The menu is aligned to the whole component (in PrimeNG `currentTarget` is the host), not to the arrow; the arrow `aria-expanded` follows the menu show (`on_menu_show`/`on_menu_hide`), `aria-controls` points to the menu, `aria-haspopup="true"`.
- `content` replaces the content of the main button; `dropdown_icon` — the arrow; `disabled` disables both buttons, `button_disabled`/`menu_button_disabled` — one each.

## Differences

| Difference | Reason |
|---|---|
| The menu is a flat `prime_menu::Menu` with groups, not `TieredMenu`; the `nested` demo is not ported | `TieredMenu` is not ported yet (the last component of wave 4). When it appears, the inner `Menu` will be replaced without changing the `SplitButton` API. |
| The dropdown button is drawn by `SplitButton` itself, not by `Button` | `Button` does not accept reactive `aria-expanded`/`aria-controls`; classes and `data-p` are taken from `prime-button` functions so as not to diverge from it. Ripple on it is attached the same way as for `Button`. |
| The root is a `<div>`, not the `<p-splitbutton>` host | The host in Angular is a framework detail. |
| Icons are `ViewFn` (`icon`, `dropdown_icon`), not `pi pi-…` classes; `content` is a `ViewFn` | No PrimeIcons and `ng-template`. |
| No `tooltip`/`tooltipOptions`, `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` | A tooltip can be added from outside with the `use:tooltip` directive; the menu is always inside the root (like the other overlays of the project); animation uses theme classes. |
| `button_aria_label`/`expand_aria_label` instead of the fields `buttonProps.ariaLabel`/`menuButtonProps.ariaLabel`/`expandAriaLabel` | Flat string properties. |
| In the demos the icons `pi-check`, `pi-cog` are replaced with `CheckIcon`, `RefreshIcon`; the logo in `template` is text; `reversedkeys` is skipped | `prime-icons` has no `cog`; in PrimeNG `reversedkeys` is text about theme tokens, not about the component. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Menu position when there is not enough room below — same as `Menu`, details in `docs/menu.md`.

## Performance

See `benches/` (`splitbutton-1`, `splitbutton-1000`, `case-splitbutton`) and `benches/*budgets.toml`.
