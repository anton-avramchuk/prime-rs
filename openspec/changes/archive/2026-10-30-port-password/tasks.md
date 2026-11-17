# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-password`: `state.rs` (`strength`, `meter_width`, `meter_class`, root classes, `data_p`) with table-driven tests
- [x] 1.2 Component `Password`: properties, markup on top of `InputText` (or our own field, see D4), clear and show icons, hint panel, templates `header`/`footer`/`content`/icons, SSR tests, rustdoc
- [x] 1.3 Behavior: focus/input/`Escape`/`blur`, panel phases and animation, z-index and `align_overlay`, closing on `resize`, locale; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `password` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change
- [x] 2.2 Page `/password` (demos from PrimeNG without Angular forms), menu, route, `stylesheet(&[..])`, crate in workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `password.spec.ts` (panel on focus, levels, `Escape`, show password, clearing); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test password`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-password` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/password.md` (differences from PrimeNG, what was not verified); `prime-password` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-password`; CI `dev` green

## Workflow follow-up

- Archived after CI.
- Next: there is no wave 2 plan left — choose candidates for the next wave.
