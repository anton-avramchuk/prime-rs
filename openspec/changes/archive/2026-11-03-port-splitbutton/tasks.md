# Tasks

## 1. Foundation

- [x] 1.1 Add `splitbutton` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-splitbutton*` classes exist

## 2. Crate

- [x] 2.1 `crates/prime-splitbutton`: `state.rs` (`root_classes`, `is_toggle_key`) with table-driven tests
- [x] 2.2 `SplitButton` component, `SplitButtonPt`: root, main button (`Button`), dropdown button (own markup), `content`, modifiers, SSR tests, rustdoc
- [x] 2.3 Menu: `Menu popup` with `MenuRef`, attachment to the root, toggling by click and ↓/↑, closing on a click of the main button, `aria-expanded`, show and hide events; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/splitbutton` (`basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `splitbutton.spec.ts` (markup, clicks, keys, panel position, item command); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test splitbutton`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-splitbutton` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/splitbutton.md` (differences from PrimeNG, what was not verified); `prime-splitbutton` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-splitbutton`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Dock.
