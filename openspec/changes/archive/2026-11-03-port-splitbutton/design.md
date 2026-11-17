# Design

## Context

Requirements are in `specs/splitbutton`. PrimeNG 21.1.10 sources: `splitbutton/splitbutton.ts` (about 300 lines), `splitbutton/style/splitbuttonstyle.ts`; styles `@primeuix/styles/splitbutton`; demos `apps/showcase/doc/splitbutton/*-doc.ts`. Reference implementations in the project: `prime-button` (`Button`, `ButtonState`, class functions), `prime-menu` (`Menu` with `popup`, `MenuRef::show_at`/`toggle`/`hide`, `on_show`/`on_hide`), `prime-panel` (a toggle button with its own markup, because `ButtonPt` does not support reactive attributes).

Observations about the source:
- Template: the main button (`pButton`, `label`/`icon`/`iconPos`, or the `content` template), the dropdown button (`pButton`, the `ChevronDown` icon or `dropdownIcon`), `p-tieredmenu` in `popup` mode.
- Root classes: `p-splitbutton p-component` + modifiers by `raised`, `rounded`, `outlined`, `text`, `size` (`small → sm`, otherwise `lg`); the host has `data-p-severity`.
- `onDefaultButtonClick`: `onClick.emit(event)`, then `menu.hide()`. `onDropdownButtonClick`: `onDropdownClick.emit(event)`, then `menu.toggle({ currentTarget: hostElement, relativeAlign: appendTo == 'self' })` — the menu is aligned to the host, not to the arrow. `onDropdownButtonKeydown`: ↓/↑ → the same as a click, and `preventDefault`.
- `aria-expanded` of the dropdown button is `isExpanded` (changed by the menu `onShow`/`onHide`), `aria-controls` is the menu `ariaId`.
- `raised` and `rounded` reach the buttons only via the root CSS (`p-splitbutton-raised` / `-rounded`), while `severity`, `text`, `outlined`, `size` are passed to `pButton` itself.
- The menu is a `TieredMenu`; the project has no `TieredMenu` yet (the last component of wave 4), only a flat `Menu` with groups.

## Goals / Non-Goals

**Goals:** the same DOM, classes and `aria-*`; the same click, key and menu-closing behavior; the menu is attached to the root.

**Non-Goals:** nested submenus (until `TieredMenu`), `tooltip`/`tooltipOptions`, `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `Ripple`, `@primeuix/motion`, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and API
`prime-splitbutton` with `SplitButton`, `SplitButtonPt { root, button, dropdown, menu }`. The model is `Prop<Vec<MenuItem>>` from `prime-menu`. `severity: OptProp<ButtonSeverity>`, `size: OptProp<Size>`, `icon: Option<ViewFn>`, `icon_pos: Prop<IconPos>`, `content: Option<ViewFn>`.

### D2. Main button
`Button` with `style_class="p-splitbutton-button"` and the shared `severity`/`text`/`outlined`/`plain`/`size`/`icon`/`icon_pos`/`label`/`disabled`/`tabindex`/`autofocus`/`aria_label`; with `content` — the button `children` instead of the label (label and icon are not rendered). `disabled = disabled || button_disabled`. The `on_click` wrapper: the user handler, then `menu.hide()`.

### D3. Dropdown button
Own markup `<button type="button">` with classes and `data-p*` from `prime_button::{ButtonState, root_classes, root_data_p}` (an icon without a label gives `p-button-icon-only`) plus `p-splitbutton-dropdown`; the icon is `ChevronDownIcon` with class `p-button-icon` or `dropdown_icon`. Reactive `aria-expanded`/`aria-controls` are set via closure attributes. The reason for own markup is that `Button` does not accept reactive arbitrary attributes.

### D4. Menu
`Menu popup=true menu_ref=… model=… style=menu_style style_class=menu_style_class id=menu_id on_show on_hide` inside the root. `MenuRef` is created in `SplitButton`; opening is `menu_ref.show_at(root_element)` (the root is obtained via `NodeRef`), closing is `menu_ref.hide()`. The menu `on_show`/`on_hide` change `expanded: RwSignal<bool>` and call `on_menu_show`/`on_menu_hide`.

### D5. Pure logic (`state.rs`)
`root_classes(raised, rounded, outlined, text, size)` — root classes; `is_toggle_key(code)` (`ArrowDown`/`ArrowUp`). Table-driven tests.

### D6. CSS
`splitbutton` is added to `theme.components`. The `.p-splitbutton*` rules and the shared button styles are already in the theme; check that `p-splitbutton-raised`, `-rounded`, `-button`, `-dropdown` exist.

### D7. Tests
SSR: markup, modifiers, `severity` on both buttons, `content`, `disabled`/`button_disabled`/`menu_button_disabled`, `aria-*` of the dropdown button, menu closed. Browser `tests/web.rs`: click on the main button (event, menu closing), click on the arrow (event, menu open, `aria-expanded`), ↓/↑, closing by an outside click, panel position under the root, item command, show/hide events. e2e: a real mouse and keys, panel position, screenshot pairs.

### D8. Demos and benchmarks
Demos `basic` (with `Toast`/`MessageService` for the `command` items), `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`. PrimeIcons icons (`pi-check`, `pi-cog`) are replaced from `prime-icons`. Bench `splitbutton-1`/`-1000` and `case-splitbutton`.

## Risks / Trade-offs

- [Menu without nesting] `nested` is not ported until `TieredMenu`; recorded in docs.
- [Own dropdown button] Classes and `data-p` are taken from `prime-button` functions so as not to diverge from `Button` when it changes.
- [Panel position] `Menu` aligns relative to an ancestor with `position: relative`; the `.p-splitbutton` root gets it from the theme — checked in e2e.

## Open Questions

- Whether `appendTo` is needed: no, the menu is inside the root, like the other overlays of the project.

## Migration Plan

No migration. Rollback — `git revert`.
