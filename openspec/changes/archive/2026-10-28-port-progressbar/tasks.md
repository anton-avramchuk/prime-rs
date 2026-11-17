# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-progressbar` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`), add it to the workspace; check `cargo check -p prime-progressbar --all-features`
- [x] 1.2 Implement `ProgressBarMode`, `ProgressBarState`, `write_classes`, `data_p` after `progressbarstyle.ts` and `ProgressBar.dataP`; verify with a table of unit tests

## 2. Component

- [x] 2.1 Implement `ProgressBar` (ARIA, both modes, label, `color`, template, `pt`, two branches); verify with SSR tests and parity
- [x] 2.2 Add rustdoc with an example; check `cargo test -p prime-progressbar --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/progressbar` page (`basic`, `dynamic`, `indeterminate`, `template`), menu, `progressbar` CSS
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/progressbar.spec.ts`; check `npx playwright test progressbar`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `progressbar-1`, `progressbar-1000`, `case-progressbar`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/progressbar.md`; clippy, tests, deny, machete, `openspec validate port-progressbar`; CI `dev` green

## Workflow follow-up

- Archive after CI; wave 1 is complete (Button, Badge, Tag, Chip, Avatar, Divider, Card, InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar).
