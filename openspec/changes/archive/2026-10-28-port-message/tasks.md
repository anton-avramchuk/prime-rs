# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-message` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core`, `prime-icons` (`times`), `web-sys`), add to the workspace; check `cargo check -p prime-message --all-features`
- [x] 1.2 Implement `MessageSeverity`, `MessageVariant`, `MessageState`, `write_classes`, `data_p` per `messagestyle.ts` and `Message.dataP`; verify with a table of unit tests

## 2. Component

- [x] 2.1 Implement `Message` (markup, classes, text and children, slots, `pt`, two branches); verify with SSR tests and parity
- [x] 2.2 Implement closing, `on_close` and `life`; verify with `wasm-bindgen-test`
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-message --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/message` page (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `icon`, `life`), menu, `message` CSS
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/message.spec.ts`; check `npx playwright test message`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `message-1`, `message-1000`, `case-message`, budgets from measurement

## 5. Integration

- [x] 5.1 `docs/message.md`; clippy, tests, deny, machete, `openspec validate port-message`; `prime-message` in the CI browser job; CI `dev` green

## Workflow follow-up

- Archive after CI; next is ProgressBar.
