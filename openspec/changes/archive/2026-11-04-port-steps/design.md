# Design

## Context

Requirements are in `specs/steps`. PrimeNG 21.1.10 sources: `steps/steps.ts` (about 300 lines), `steps/style/stepsstyle.ts`; styles `@primeuix/styles/steps`. There are no demos in the local copy (the `apps/showcase/doc/steps` directory is absent in this commit, the `pages/steps/index.ts` page references them), so the demos are composed from the meaning of the list (`basic`, `controlled`, `interactive`) and from the component code. References in the project: `prime-stepper` (the same CSS style and a "steps — panels" pair), `prime-dock` (its own item model, a command `Callback`), `prime-tabs` (roving `tabindex`).

Observations about the source:
- Item activity: without `routerLink` — `index === activeIndex`; with `routerLink` — a match with the router address (not ported).
- Item class: `p-steps-item`, `p-steps-item-active` (active), `p-disabled` (`item.disabled || (readonly && !active)`). `aria-current="step"` is on the active one.
- Link: `tabindex` — `-1` on a disabled one, `item.tabindex || '0'` on the active one, `item.tabindex ?? '-1'` on the rest; `aria-expanded = (i === activeIndex)`; `aria-disabled = item.disabled || (readonly && i !== activeIndex)`.
- Click: with `readonly` or `disabled` — `preventDefault`, exit; otherwise `activeIndexChange.emit(i)`, `preventDefault` without `url`/`routerLink`, then `command({ originalEvent, item, index })`. The `activeIndex` value is NOT changed inside the component (the owner drives it) — a two-way binding.
- Keys: ←/→ — a neighbouring item, `Home`/`End` — the extreme ones (focus is moved by calling `focus()` and a roving `tabindex`), `Tab` — returns `tabindex="0"` to the active item, `Enter`/`Space` — a click; `preventDefault` on the handled ones.
- A bug in the source: `track item.label` in `@for` — items with the same label get merged; in the port the key is the index.

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, click and key behaviour.

**Non-Goals:** router, item tooltips, HTML labels, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and model
`prime-steps` with `Steps`, `StepsItem` (a builder), `StepsCommand { event, item, index }`, `StepsPt`. Its own item type rather than `MenuItem`, because `command` receives the index and `tabindex` is needed.

### D2. Pure logic (`state.rs`)
`root_classes(readonly, extra)`, `item_classes(active, disabled, extra)`, `is_item_disabled(item, index, active, readonly)`, `link_tabindex(item, index, active)`, `aria_disabled(item, index, active, readonly)`, `click_outcome(item, readonly) -> Click { Ignore, Activate { prevent_default } }`, `key_target(code, index, count) -> Option<usize>` (on the extreme items ←/→ stay in place), `is_activation_key(code)`. Table-driven tests.

### D3. State
`active_index: Prop<usize>` for reading and an optional `RwSignal<usize>` (`active_index_signal`) for two-way binding: if it is set, a click sets the value; the `on_active_index_change` event is always called. Roving `tabindex` — `focus_index: RwSignal<Option<usize>>`: on ←/→/Home/End the link is updated and focused; reset to the active item on `Tab` and when `active_index` changes.

### D4. Markup
One `li` per item, the index as the key; the item state is reactive attributes. The `a` link with `href` from `url` (in `readonly` too, as in PrimeNG).

### D5. Tests
SSR: markup, classes by `readonly`/`disabled`/active, `aria-*`, `tabindex`, `href`/`target`, `pt`. Browser `tests/web.rs`: a click in `readonly` (nothing), a click in interactive mode (event, command with the index, signal), a disabled item, a link with `url` is not cancelled, keys and roving `tabindex`, `Enter`/`Space`, `Tab`. e2e: theme styles (numbers in circles, a line between steps), click/keys in the interactive demo.

### D6. Demos and benchmarks
`basic` (readonly), `controlled` ("Back/Next" buttons change `active_index`), `interactive` (a click on items, `Toast` via `command`). Bench `steps-1`/`-1000` and `case-steps`.

## Risks / Trade-offs

- [No reference demos] The demos are derived from the code and the list; the result is checked by classes and theme CSS, not by reference screenshots.
- [Two ways to drive `active_index`] A value and a signal; documented in `docs/steps.md`.

## Open Questions

- Whether `exact` is needed: in PrimeNG it affects only `ariaCurrentWhenActive` of router links — we keep the property for compatibility, it has no behaviour.

## Migration Plan

No migration. Rollback — `git revert`.
