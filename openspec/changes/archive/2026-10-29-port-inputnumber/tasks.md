# Tasks

## 1. Crate, format and text editing

- [x] 1.1 Create the crate `crates/prime-inputnumber` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core`, `prime-inputtext`, `prime-icons` with `angleup`, `angledown`, `times`, `web-sys` with `InputEvent`, `HtmlInputElement`, `ClipboardEvent`/`DataTransfer`, wasm dev-dependencies as in `prime-slider`), add it to the workspace; verify `cargo check -p prime-inputnumber --all-features`
- [x] 1.2 `locale.rs` and `format.rs`: the table of locales and currencies, `resolve_locale`, `format_number`, rounding on the shortest decimal representation, grouping (including 3-2-2), the currency and its position, `prefix`/`suffix`; table tests over all locales and currencies
- [x] 1.3 Parsing `parse_text` (prefix, suffix, currency, groups, minus, decimal separator, the "only a minus" state) and `parse(format(x)) == x` round-trip tests
- [x] 1.4 `edit.rs`: `insert`, `delete_back`, `delete_forward`, `delete_range`, `paste`, `commit`, `clamp_caret`, keeping the typed fraction "tail" and the caret by the number of significant characters; table tests (typing, minus, separator, fraction limit, Backspace/Delete over separators and the fraction, ranges, paste, prefix and suffix)
- [x] 1.5 `state.rs`: root, button classes and `data-p` per `inputnumberstyle.ts` and `InputNumber.dataP`; table tests

## 2. Component

- [x] 2.1 Implement `InputNumber`, `InputNumberPt`, `InputNumberInput` (markup, button layouts, clear icon, attributes, `display` and formatting, external signal); SSR tests (formats, structure, classes and `data-p`, layouts, clearing, attributes, `pt`, branch parity)
- [x] 2.2 Input events: `beforeinput` (typing, deletion, paste), `keydown` (arrows, `Enter`, `Tab`, `Home`, `End`), `focus`/`blur`, `click` (`clamp_caret`), value commit, `on_input`, `on_key_down`
- [x] 2.3 Buttons: step, repeat while held (500 ms, then 40 ms), stop on `mouseup`/`mouseleave`/`keyup` and in `on_cleanup`, `readonly`/`disabled`, `max_length`; clearing and `on_clear`
- [x] 2.4 Browser tests `tests/web.rs`: typing and caret, Backspace/Delete, paste, commit, bounds, arrows, buttons and repeat, `readonly`/`disabled`, clearing, external signal and `locale` change, keeping "1." while typing
- [x] 2.5 rustdoc with an example; `cargo test -p prime-inputnumber --all-features`

## 3. Theme, showcase, e2e

- [x] 3.1 Add `inputnumber` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, link the CSS
- [x] 3.2 The `/inputnumber` page (`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid`), menu, route, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` and `e2e/tests/inputnumber.spec.ts` (keyboard typing, arrows, buttons; wait for `networkidle`; `scrollIntoViewIfNeeded` before coordinates); restart the showcase server before the run; `npx playwright test inputnumber`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `inputnumber-1`, `inputnumber-1000`, `case-inputnumber`, budgets by measurement (first `cargo bench -p render-bench --bench alloc`)

## 5. Integration

- [x] 5.1 Add `prime-inputnumber` to the browser job of `ci.yml` (3 places); `docs/inputnumber.md` (differences from PrimeNG, including locales, `beforeinput`, the caret); clippy, tests, deny, machete, `openspec validate port-inputnumber`; CI `dev` green

## Workflow follow-up

- Archive after CI.
