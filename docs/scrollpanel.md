# ScrollPanel

Port of `p-scrollpanel` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-scrollpanel::ScrollPanel` | `packages/primeng/src/scrollpanel/scrollpanel.ts`, `scrollpanel/style/scrollpanelstyle.ts` |
| CSS | `@primeuix/styles/scrollpanel` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/scrollpanel/*-doc.ts` (`basic`, `custom`) |

Spec: `openspec/specs/scrollpanel`.

## What matches PrimeNG

- Markup and classes: `p-scrollpanel p-component` → `p-scrollpanel-content-container` → `p-scrollpanel-content`, then the bars `p-scrollpanel-bar p-scrollpanel-bar-x` and `p-scrollpanel-bar-y` with `role="scrollbar"`, `tabindex="0"`, `aria-orientation`, `aria-valuenow`, `aria-controls`, `data-pc-group-section="bar"`.
- Bar size and position formulas (length not less than 10%, offset proportional to scroll), the `data-p-scrollpanel-hidden` attribute and the `p-scrollpanel-hidden` class, root height calculation with `max-height`.
- Recalculation on `scroll`, `mouseenter` of the content and window `resize`; bar dragging with `p-scrollpanel-grabbed` on the bar and `body`.
- Keyboard: arrows on a focused bar scroll by `step` after 40 ms, `keyup` cancels the timer, so the key must be held down (in Playwright, `keyboard.down`, not `press`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-scrollpanel>` host; the `.p-scrollpanel { display: block }` rule from `scrollpanelstyle.ts` is not needed | A `div` is already block-level; the host in Angular is a framework detail. |
| Content is `children` or the `content_template` slot, mutually exclusive | Instead of `<ng-content>` and the `#content` template; in Angular with a template `<ng-content>` is hidden by the same `@if`. |
| The methods `refresh()` and `scroll_top(value)` go through the `ScrollPanelRef` handle (the `panel_ref` property) | No `@ViewChild`. |
| When dragging the horizontal bar `data-p-scrollpanel-grabbed="true"` | In PrimeNG it is `"false"` there (a typo in `onXBarMouseDown`); the `p-scrollpanel-grabbed` class is set in both cases. |
| `aria-valuenow` is updated directly in the DOM from the `scroll` handler | No change detection; the value is the same. |
| The bar `style` is driven by the client recalculation; `pt.bar_*.style` applies only on the first render and is then overwritten | Otherwise Leptos and `moveBar` would overwrite each other. Set the bar appearance with classes. |
| The root height with `max-height` is fixed once after mounting; a reactive `style` with `max-height` will overwrite it | As in PrimeNG, the calculation is one-off; a signal that changes `style` returns the value from `style`. |
| The `step` property is read when the key is pressed, not at render | Allows a signal without re-rendering. |
| No `BlockableUI`, template synonyms, `appendTo` | Angular interface and legacy names. |
| The drag position is rounded to whole pixels | `scrollTop` in `web-sys` without `unstable_apis` is `i32`. |

## Performance

SSR: 26 allocations per instance with one paragraph inside (budget 29); contribution to raw wasm +184 KB (budget 200 KB). A noticeable part is client code: `f64` formatting for the bar `style`, `web-sys` listeners and `getComputedStyle`; there is none on the server.
The markup template is the same as Toolbar's (`docs/toolbar.md`), the client part is `crates/prime-scrollpanel/src/client.rs`.

## Tests

SSR is `tests/ssr.rs`; browser tests are `tests/web.rs` (`cargo test -p prime-scrollpanel --target wasm32-unknown-unknown --test web`, require `wasm-bindgen-cli` and `chromedriver`); e2e is `e2e/tests/scrollpanel.spec.ts`. The browser tests do not include the theme CSS: the minimum rules needed for scrolling are set in the test.
