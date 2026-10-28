# Tasks

## 1. Ядро

- [x] 1.1 `ClassNames`: стековый буфер, признак «динамический», `finish_cow()` с интернированием (лимит 4096); юнит-тесты (одинаковый результат, повторный вызов даёт тот же `&'static str`, динамические строки не интернируются, переполнение буфера)
- [x] 1.2 `DataP::finish_cow()`; замер на пустом сценарии: повторный вызов не аллоцирует (тест с счётчиком аллокаций в `render-bench`)

## 2. Компоненты

- [x] 2.1 Перевести `class`/`data-p` на `Cow<'static, str>` в `prime-avatargroup` и `prime-progressbar`, убрать лишние `Arc` в статической ветви; сверить число аллокаций и HTML
- [x] 2.2 То же в `prime-badge`, `prime-tag`, `prime-chip`, `prime-avatar`, `prime-divider`, `prime-card`
- [x] 2.3 То же в `prime-button`, `prime-inputtext`, `prime-checkbox`, `prime-radiobutton`, `prime-toggleswitch`, `prime-message`

## 3. Бюджеты и интеграция

- [x] 3.1 Пересчитать `benches/alloc-budgets.toml` по измерениям (запас ≤ 15 %), дополнить `docs/benchmarks.md` таблицей до/после
- [x] 3.2 `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --exclude showcase --exclude wasm-size --all-features`, браузерные тесты, `cargo deny check`, `cargo machete`, `openspec validate optimize-allocs`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
