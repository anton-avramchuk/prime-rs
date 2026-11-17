# Design

## Context

The requirements are in `specs/imagecompare`. PrimeNG 21.1.10 sources: `imagecompare/imagecompare.ts` (about 150 lines), `imagecompare/style/imagecomparestyle.ts`; styles `@primeuix/styles/imagecompare`; demos `apps/showcase/doc/imagecompare/{basic,responsive}-doc.ts`. Template in the project: `prime-slider`/`prime-knob` (the `input` handlers and `cfg(wasm32)` for DOM calls), `prime-divider` (simple markup).

Observations on the source:
- Template: `left`, `right`, `input[type=range][min=0][max=100][value=50]`; the `onSlide` handler reads `event.target.value`, finds the `previousElementSibling` of the slider and sets its `style.clipPath`.
- Theme: `.p-imagecompare img + img { clip-path: polygon(0 0, X 0, X 100%, 0 100%) }` with `X = 50%` by default; so the right image must be the second `img` right after the left one. Therefore the views are rendered without wrappers.
- The direction is determined at creation and by a `MutationObserver` watching `dir` on `<html>`: `isRTL = !!el.closest('[dir=rtl]')`; it is applied only in `onSlide`, so computing it at the moment of input gives the same result.
- `tabindex`, `aria-labelledby`, `aria-label` — on the host (the root).

## Goals / Non-Goals

**Goals:** the same DOM and classes; the same `clip-path` on input, including `rtl`.

**Non-Goals:** `MutationObserver`, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and API
`prime-imagecompare` with `ImageCompare`, `ImageComparePt { root, slider }`. Properties: `left: ViewFn`, `right: ViewFn` (required), `tabindex: OptProp<i32>`, `aria_label`, `aria_labelledby`, `style`, `style_class`, `pt`.

### D2. Pure logic (`state.rs`)
`clip_path(value: f64, rtl: bool) -> String` — a `polygon(...)` string with percentages formatted without trailing zeros (integers without a decimal part; `30` → `30%`, `33.5` → `33.5%`); values outside 0–100 are clamped. Table tests, including `rtl`.

### D3. Input handler
The `on:input` of the slider (browser only; in SSR the handler is not called): reads `HtmlInputElement::value`, finds `previous_element_sibling`, determines `rtl` by `closest("[dir=rtl]")` of the slider, sets `style.clip-path` via `HtmlElement::style().set_property`. The absence of a sibling or a non-numeric value — a silent exit.

### D4. CSS
`imagecompare` is added to `theme.components`. Check `p-imagecompare`, `p-imagecompare-slider`, the handle variables; there is no `For PrimeNG` block in `imagecomparestyle.ts`.

### D5. Tests
SSR: order and composition, the slider attributes, `tabindex`/`aria-*`, `style`/`style_class`/`pt`, the views without wrappers. Browser `tests/web.rs`: input 30 → `clip-path` on the previous element, `rtl` in an ancestor, repeated input, input without a sibling. e2e: a real drag of the handle (a mouse over the range), `clip-path` from the value, a screenshot pair.

### D6. Demos
The images from the CDN are replaced by two built-in SVGs (`data:` URIs) of different colours with a caption — independence from the network. `responsive` — a container of fixed width.

### D7. Benchmarks
Scenarios `imagecompare-1`/`-1000` (two `<img>`), `case-imagecompare`; first `cargo bench -p render-bench --bench alloc`; for wasm — a separate budgets file `--budgets`.

## Risks / Trade-offs

- [The `img + img` structure] If a user passes something other than `img`, the theme rule will not apply; as in PrimeNG, recorded in the docs.
- [Input before hydration] The handler is attached on hydration; early input is lost, as for the other components.

## Open Questions

- Whether a two-way value (`value` as a signal) is needed: PrimeNG has none, so we do not add it.

## Migration Plan

No migration. Rollback — `git revert`.
