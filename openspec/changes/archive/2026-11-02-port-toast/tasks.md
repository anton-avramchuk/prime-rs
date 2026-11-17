# Tasks

## 1. Foundation

- [x] 1.1 Add `toast` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-toast-message-*` classes, icons and `@keyframes` for `p-toast-message-enter-active` / `-leave-active` exist (otherwise — resolve the question from design.md)

## 2. Crate

- [x] 2.1 `crates/prime-toast`: `state.rs` (`ToastSeverity`, `ToastPosition`, `message_classes`, `root_classes`, `position_style`, `matches_key`, `contains_message`, `can_add`, `lifetime`, `breakpoints_css`) with table-driven tests
- [x] 2.2 `ToastMessage` (builder) and `MessageService` (`provide_message_service`, `use_message_service`, `add`, `add_all`, `clear`, `clear_key`, subscribe/unsubscribe) with browser-free tests
- [x] 2.3 `Toast` component and `ToastPt`: container, list items, icons, close button, `message`/`headless`, SSR tests, rustdoc
- [x] 2.4 Behavior: adding and keys, timers (`life`, `sticky`, hover), closing and exit animation, `clear`/`clear_key`, duplicates, layer, `breakpoints`, `on_close`, unsubscribe; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/toast` (`basic`, `severity`, `multiple`, `position`, `sticky`, `life`, `clear`, `target`, `responsive`, `template`, `headless`), message service in the shell, menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `toast.spec.ts` (display, severities, closing via the button, auto-close, sticky, hover, clear, keys, positions); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test toast`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-toast` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background)

## 5. Integration

- [x] 5.1 `docs/toast.md` (differences from PrimeNG, what was not verified); `prime-toast` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-toast`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Menu.
