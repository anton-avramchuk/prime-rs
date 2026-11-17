# Tasks

## 1. Foundation

- [x] 1.1 Add `cascadeselect` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and the `p-cascadeselect*` classes exist
- [x] 1.2 Check against the source `onOptionChange`, `onOptionClick`, `onArrow*Key`, `show/hide`, `onOptionMouseEnter`; `prime-icons` icons: chevrondown, angleright, times, spinner

## 2. Crate `prime-cascadeselect`

- [x] 2.1 `CascadeOption` model, pure logic `state.rs` (nodes, path, focus, keys, search) with table-driven tests
- [x] 2.2 `CascadeSelect` component, `CascadeSelectPt`: root, hidden input, label, clear, button, panel, level lists, SSR tests, rustdoc
- [x] 2.3 Opening and closing, cascade by click and hover, leaf selection, clearing, `breakpoint`
- [x] 2.4 Keyboard, search, focus, events
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/cascadeselect`, menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `cascadeselect.spec.ts`; `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test cascadeselect`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-cascadeselect`, budgets

## 5. Integration

- [x] 5.1 `docs/cascadeselect.md`; `prime-cascadeselect` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for wasm), tests, deny, machete, `openspec validate port-cascadeselect`; CI — once at the end of wave 5 (not run)

## Workflow follow-up

- Archive; no push and no CI wait until the end of wave 5.
- Next in the wave 5 plan: AutoComplete, InputMask.
