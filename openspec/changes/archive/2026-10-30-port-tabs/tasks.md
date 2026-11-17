# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-tabs`: `state.rs` (identifiers, classes, `tabindex_for`, `key_action`, `wrap_target`, `should_render`, button scrolling) with table-driven tests
- [x] 1.2 Components `Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt`: context, markup, SSR tests, rustdoc
- [x] 1.3 Behaviour: click, focus, keys, roving `tabindex`, indicator, scroll buttons, `lazy`; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add the `tabs` module to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components has not changed
- [x] 2.2 The `/tabs` page (`basic`, `controlled`, `disabled`, `scrollable`, `dynamic`, `lazy`, `customtemplate`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `tabs.spec.ts` (click, keyboard, disabled, indicator, scrolling, lazy); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test tabs`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-tabs` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/tabs.md` (differences from PrimeNG, what was not verified); `prime-tabs` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-tabs`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Popover.
