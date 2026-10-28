# CI

Workflow-файлы лежат в `.github/workflows/`. Их нельзя запустить локально целиком: логика выбора крейтов, бюджетов и сравнения
бенчмарков вынесена в `cargo xtask` и проверена тестами; сами YAML-файлы проверяются на GitHub.

## `ci.yml` — каждый PR и push в `dev`

| Job | Что делает |
|---|---|
| `changes` | `cargo xtask affected --base <база>`: список затронутых крейтов |
| `fmt` | `cargo fmt --all --check` |
| `deny` | `cargo deny check` (`deny.toml`) |
| `libs` | `clippy -D warnings`, `cargo nextest`, doc-тесты для затронутых библиотек, `--all-features` |
| `wasm` | `wasm-bindgen-test` для `prime-dom` в headless Chrome (при полном прогоне или изменении `prime-dom`) |
| `apps` | showcase (clippy для ssr и hydrate), бюджет размера wasm, бенчмарки, сравнение с базовой линией и комментарий в PR |

Правила выбора крейтов (`xtask/src/affected.rs`):
- изменён `prime-core` или `prime-dom`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `.github/`, `xtask/`, `deny.toml`, `rustfmt.toml`,
  `benches/budgets.toml` → проверяется весь workspace;
- иначе изменённые крейты и все зависящие от них (транзитивно);
- изменения только в документации и `openspec/` ничего не запускают, кроме `fmt` и `deny`.

e2e на PR не запускается. Регрессия бенчмарков больше 10 % сообщается комментарием и слияние не блокирует;
превышение бюджета размера wasm (`benches/budgets.toml`) проваливает job `apps`.

Кэш: `Swatinem/rust-cache`, сохраняется только в `dev`; перед завершением job `cargo sweep --file` удаляет артефакты, которые не
использовались в этом прогоне, а `CACHE_LIMIT_MB` (6000) — предел размера `target/`: при превышении job падает.
Базовая линия бенчмарков — артефакт `bench-baseline` (30 дней) последнего успешного прогона `dev`.

## `weekly.yml` — раз в неделю, вручную и на PR с меткой `run-e2e`

Клонирует PrimeNG на коммите из `upstream.toml`, собирает его showcase и наш, запускает Playwright (`e2e/`):
`smoke` (гидрация, переключатель схемы), `memory` (время и память для 1000 компонентов, цикл создания и удаления),
`visual` (сравнение страниц из `e2e/tests/pages.json` с оригиналом). Эталонные скриншоты на лету, в репозиторий они не попадают.
Job помечен `continue-on-error`: расхождения не блокируют PR. Артефакты (`e2e/results`) хранятся 7 дней.

Локально: поднять showcase (`cd showcase && cargo leptos serve`), затем `cd e2e && npm install && PW_CHANNEL=chrome npx playwright test`.
`UPSTREAM_URL` включает сравнение с PrimeNG.

## `machete.yml` — раз в неделю

`cargo machete`: поиск неиспользуемых зависимостей.

## Не проверено

Workflow-файлы синтаксически корректны (разобраны YAML-парсером), но ни разу не запускались на GitHub. Сборка Angular-showcase PrimeNG
(`pnpm --filter showcase build`) в `weekly.yml` написана по скриптам из `package.json` PrimeNG и не проверялась.
