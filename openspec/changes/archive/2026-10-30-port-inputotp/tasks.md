# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-inputotp`: `state.rs` (`tokens_of`, `join`, `key_decision`, `paste_value`, `input_move`) with table-driven tests
- [x] 1.2 `InputOtp` component: markup, value (controlled and not), attributes, `Pt`, SSR tests for markup and value scenarios, rustdoc
- [x] 1.3 Behavior: input and auto-advance, keys, paste, selection on focus, `on_change`/`on_focus`/`on_blur`, `input` template with `InputOtpEvents`; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `inputotp` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change
- [x] 2.2 Page `/inputotp` (`basic`, `mask`, `integeronly`, `sizes`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `inputotp.spec.ts` (input, auto-advance, Backspace, arrows, paste, integer_only); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test inputotp`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-inputotp` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/inputotp.md` (differences from PrimeNG, value model, what was not verified); browser job in `ci.yml` (three places) if `tests/web.rs` was added
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features`, tests, deny, machete, `openspec validate port-inputotp`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: ScrollTop, Password.
