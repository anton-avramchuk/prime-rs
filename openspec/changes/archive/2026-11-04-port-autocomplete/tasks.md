# Tasks

## 1. Foundation

- [x] 1.1 Add `autocomplete` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components has not changed and the `p-autocomplete*` classes are present
- [x] 1.2 Check against the source's `onInput`, `onKeyDown`, `onOptionSelect`, `search`, `updateInputWithForceSelection`, `handleSuggestionsChange`; `prime-icons` icons: chevrondown, times, timescircle, spinner

## 2. Crate `prime-autocomplete`

- [x] 2.1 The `AutoCompleteOption` model, pure logic in `state.rs` (flat list, navigation, value label, `force_selection`, separator) with table-driven tests
- [x] 2.2 The `AutoComplete` component, `AutoCompletePt`: root, field/chips, clear, spinner, button, panel, options, groups, empty message, SSR tests, rustdoc
- [x] 2.3 Input, delay and `on_complete`, `suggestions` update, selection, `multiple`, chips, arrow button, clear, `force_selection`, `complete_on_focus`
- [x] 2.4 Keyboard of the field and chip container, `typeahead = false` (adding on `Enter`/separator/`Tab`/blur/paste)
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 The `/autocomplete` page, menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `autocomplete.spec.ts`; `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test autocomplete`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-autocomplete`, budgets

## 5. Integration

- [x] 5.1 `docs/autocomplete.md`; `prime-autocomplete` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for wasm), tests, deny, machete, `openspec validate port-autocomplete`; CI once at the end of wave 5 (not run)

## Workflow follow-up

- Archive; no push and no waiting for CI until the end of wave 5.
- Next in the wave 5 plan: InputMask.
