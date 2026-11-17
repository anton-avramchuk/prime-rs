# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-inplace`: `state.rs` (`display_classes`, `next_state`) with table tests
- [x] 1.2 The `Inplace` component: properties, markup, the close button, `InplacePt`, SSR tests, rustdoc
- [x] 1.3 Behaviour: click, `Enter`, `prevent_click`, `disabled`, closing, the external signal, the callbacks; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `inplace` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, check that the CSS of the other components did not change
- [x] 2.2 The `/inplace` page (`basic`, `input`, `image`, `lazy`), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 2.3 `e2e/tests/pages.json` and `inplace.spec.ts`; wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test inplace`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-inplace` and the budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/inplace.md` (differences from PrimeNG, what was not verified); `prime-inplace` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features`, tests, deny, machete, `openspec validate port-inplace`; CI `dev` is green

## Workflow follow-up

- Archived after CI.
- Next: Splitter.
