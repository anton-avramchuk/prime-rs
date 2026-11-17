# Design

## Context

Requirements are in `specs/speeddial`. PrimeNG 21.1.10 sources: `speeddial/speeddial.ts` (about 580 lines), `speeddial/style/speeddialstyle.ts`; styles `@primeuix/styles/speeddial`; demos `apps/showcase/doc/speeddial/{linear,circle,semicircle,quartercircle,mask,tooltip,template}-doc.ts`. Reference implementations in the project: `prime-dock` (own item model, focus via `aria-activedescendant`, pure key logic), `prime-button`, `prime-tooltip` (directive), `prime-popover` (window listeners).

Observations on the source:
- Layout by `type`: `linear` — only `flex-direction`; `circle` — points on a circle of radius `radius || length*20` with step `2π/length`; `semi-circle` — step `π/(length-1)`, side offsets depend on `direction`; `quarter-circle` — step `π/(2(length-1))` and diagonal directions. Each point is `left/right/top/bottom: calc(<px> + var(--item-diff-x|y, 0px))`; `--item-diff-*` is half the difference between the button and first item sizes (computed after showing, in the browser).
- `transition-delay`: `index * transitionDelay` when showing, `(length - index - 1) * transitionDelay` when hiding.
- Key direction: `ArrowUp`/`ArrowDown` depend on `up`/`down`, `ArrowLeft`/`ArrowRight` — on `left`, `up-right`, `down-left` and `right`, `up-left`, `down-right`; the other values have a fallback meaning.
- Item focus is stored by the identifier `<id>_<index>` among items without a `p-disabled` link; `Home`/`End` reset the index and go to the first/last; `Enter`/`Space` click the item, then focus returns to the button.
- Outside click: a `click` listener on the document only while shown; ignored if the click is inside the container or there was just a click on a button/item (`isItemClicked`).
- PrimeNG has logic quirks: `findNextOptionIndex(-1)` crashes on an empty list; the index by `id` is looked up after filtering out disabled ones. In the port, indices are counted over the list of available items, without exceptions.

## Goals / Non-Goals

**Goals:** the same DOM, classes, layout, keys and closing.

**Non-Goals:** `routerLink`, `buttonProps` as a whole, `Ripple`, `Bind`, `pTemplate`, `unstyled`.

## Decisions

### D1. Crate and model
`prime-speeddial` with `SpeedDial`, `SpeedDialItem` (builder, like `DockItem`), `SpeedDialCommand { event, item }`, `SpeedDialType`, `SpeedDialDirection`, `SpeedDialPt`, `SpeedDialItemContext { item, index, toggle }`. Own model rather than `MenuItem`: no dependency on `prime-menu`.

### D2. Pure logic (`state.rs`)
`root_classes(type, direction, open, disabled, extra)`, `root_flex_style(direction)`, `list_flex_style(direction)`, `item_classes(hidden, focused)`, `point_style(type, direction, index, length, radius) -> Vec<(&str, String)>`, `transition_delay(visible, index, length, step)`, `arrow_step(direction, key) -> Step {Next, Prev}`, `step_index(count, current, step)`, `button_icon_choice(visible, has_show, has_hide)`, `button_classes(rotate_with_hide_icon...)`. Table-driven tests, including all types and directions.

### D3. State
`visible`: `RwSignal<bool>` (if the `visible: Option<RwSignal<bool>>` property is set — that one), `focused_index: RwSignal<Option<usize>>` (position among available items), `focused: RwSignal<bool>` (`aria-activedescendant` only while the list has focus), `item_clicked: StoredValue<bool>`. `show`/`hide` call `on_visible_change`, `on_show`/`on_hide`.

### D4. Markup
All items are built once on mount (hidden ones stay in the DOM with class `p-hidden` when `visible=false`; hiding the whole list is done by the theme). Item states (`p-focus`, `data-p-active`, action `tabindex`: none when `disabled` or the list is hidden, otherwise the item's `tabindex` or `0`) are reactive attributes. `--item-diff-x/y` are set after mount in the browser for all types except `linear`.

### D5. Button
The toggle button is a `Button` from `prime-button` with classes `p-speeddial-button p-button-rounded` (and `p-speeddial-rotate` if there is no `hide_icon`), `button_icon_only`; `aria-*` are reactive. Since `Button` does not accept reactive `aria-expanded`/`aria-controls`, if needed the button is rendered by `SpeedDial` itself, following `SplitButton` (classes and `data-p` from `prime-button` functions). The decision is made once it is built in the task application.

### D6. Outside click and `Escape`
The `click` listener on the window is attached for the time it is shown, on the next tick (the click that opened the list is still bubbling); a click inside the container, on the button or an item (`item_clicked`) does not hide it. `Escape` on the button and in the list hides it and returns focus to the button.

### D7. Tooltips
`tooltip_options: Option<TooltipOptions>` — a template: for each item the `tooltip` directive gets a clone with the `label` text (`disabled` until the options are set); the position is taken from the template.

### D8. Tests
SSR: root markup and classes by types/directions, items, `p-hidden`, button (`aria-*`), `pt`, the mask is not rendered on the server. Browser `tests/web.rs` as one sequential test (shared window listeners): toggling with the button, events, item command and hiding, a disabled item, `hide_on_click_outside`, `Escape`, button and list keys by direction (all eight), `Home`/`End`/`Enter`/`Space`, skipping disabled items, `mask`, controlled `visible`, `disabled`, layout (checking `left/top/bottom/right` of circle items), `transition-delay`. e2e: theme styles, expansion by direction, circle and semicircle, mask, tooltip, templates.

### D9. Demos and benchmarks
All seven demos; icons — `prime-icons` (PlusIcon, PencilIcon?, RefreshIcon, TrashIcon?, UploadIcon?, ExternalLinkIcon? — pick existing ones). Benchmarks `speeddial-1`/`-1000` and `case-speeddial`.

## Risks / Trade-offs

- [Layout geometry] Without the theme CSS the item positions cannot be checked in wasm tests; we check style strings and positions in e2e.
- [Icon set] Not all PrimeIcons icons exist in `prime-icons`; missing ones are replaced by close ones.
- [Source quirks] Focus indices are counted over available items; the difference is documented.

## Open Questions

- Whether the toggle button should be a `Button` or a custom one is decided during implementation (D5).

## Migration Plan

No migration. Rollback — `git revert`.
