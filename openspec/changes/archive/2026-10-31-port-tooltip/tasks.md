# Tasks

## 1. Foundation

- [x] 1.1 Spike `use:` on the root of project components (`Button`, `InputText`) and on a plain element: is it accepted, is it called on build, how many times on `rebuild`; record the result in design.md (D1) and choose a fallback if needed
- [x] 1.2 Add `tooltip` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the tooltip arrow and position are in the CSS

## 2. Crate

- [x] 2.1 `crates/prime-tooltip`: `state.rs` (`TooltipPosition`, `TooltipEvent`, `priority`, `place`, `arrow_style`, `out_of_bounds`, `root_classes`, `activation_delay`, `has_ellipsis`, `leave_keeps_open`) with table-driven tests
- [x] 2.2 `TooltipOptions` (builder, `TooltipContent`) and the `tooltip` directive: per-element state table, `hover`/`focus`/`both` listeners, SSR tests (markup does not change), rustdoc
- [x] 2.3 Show and hide: creating the tooltip in `body`, position and fallback by priority, arrow, delays and `life`, `auto_hide`, Escape/resize/scroll, layer, content (text, HTML, `View`), reactive text and `disabled`, `show_on_ellipsis`, cleanup; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/tooltip` page (`position`, `event`, `delay`, `autohide`, `options`, `custom`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `tooltip.spec.ts` (mouse hover, keyboard focus, four sides, delay, auto-hide, Escape); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test tooltip`

## 4. Benchmarks and budgets

- [x] 4.1 `-1` and `-1000` scenarios, `case-tooltip` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 5. Integration

- [x] 5.1 `docs/tooltip.md` (differences from PrimeNG, what was not verified); `prime-tooltip` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-tooltip`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Toast.
