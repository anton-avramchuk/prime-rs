# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-select` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core`, `prime-dom`, `prime-icons` with `chevrondown`, `search`, `times`, `check`, `spinner`, `web-sys` with the needed events and geometry, wasm dev-dependencies as in `prime-inputnumber`), add to the workspace; check `cargo check -p prime-select --all-features`
- [x] 1.2 `options.rs`: `SelectOption`, `SelectGroup`, `Entry`, `visible_entries` (filter and groups), indices (`first_valid`, `last_valid`, `next_valid`, `prev_valid`, `selected_index`, `first_focused`), `search_match`, `set_size`, `pos_in_set`; table-driven tests
- [x] 1.3 `state.rs`: root, label and option classes, root and label `data-p` per `selectstyle.ts` and `Select.dataP`; table-driven tests
- [x] 1.4 Implement `Select`, `SelectPt`, `SelectChange` (label and placeholder, clear icon, button and icons, panel markup with options, groups, empty message, search field and hidden boundaries, `aria-*`, templates); SSR tests of the closed and open (via `open`) states
- [x] 1.5 Panel behaviour: open and close, positioning and z-index (`prime-dom`, `ZGroup::Overlay`), outside click and `resize`, scrolling the selected option into view, focus return, `on_show`/`on_hide`, `disabled`/`readonly`/`loading`
- [x] 1.6 Selection and keyboard: click and `focus_on_hover`, selection and `on_change`, arrows, `Alt+ArrowUp`, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab` (including the focus trap), `Delete`, type-ahead search, `select_on_focus`, `auto_option_focus`, `aria-activedescendant`
- [x] 1.7 Search and groups: search field, filtering, `on_filter`, `reset_filter_on_hide`, `autofocus_filter`, empty list and empty result messages, groups and their template; clear and `on_clear`
- [x] 1.8 Browser tests `tests/web.rs`: open and close in every way, selection by mouse and key, skipping disabled, type-ahead, filter, groups, clear, `disabled`/`readonly`/`loading`, focus and label stability, external `value` and `open`, reactive options
- [x] 1.9 rustdoc with an example; `cargo test -p prime-select --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `select`, `iconfield` to `theme.components` and `select` to `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components has not changed, wire up the CSS
- [x] 2.2 The `/select` page (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/select.spec.ts` (keyboard, mouse, filter, panel position; wait for `networkidle`; `scrollIntoViewIfNeeded` before coordinates); restart the showcase server before the run; `npx playwright test select`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `select-1`, `select-1000`, `case-select`, budgets from measurement (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 Add `prime-select` to the browser job in `ci.yml` (3 places); `docs/select.md` (differences from PrimeNG: scope, overlay, filter, string values); clippy, tests, deny, machete, `openspec validate port-select`; CI `dev` green

## Workflow follow-up

- Archive after CI.
