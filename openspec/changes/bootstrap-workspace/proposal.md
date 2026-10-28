# Proposal

## Why

Проект prime-rs — порт компонентов PrimeNG (21.1.10, коммит `fe82b291d6`) на Rust/Leptos 0.8 по принципу «один компонент — один крейт». Репозиторий пока пуст (только `openspec/`). Прежде чем портировать первый компонент, нужна инфраструктура («волна 0»): структура workspace, общее ядро, тема, иконки, CI, бенчмарки и защита от разрастания `target/`. Без неё каждый компонент будет изобретать это заново, а ограничения по диску, памяти и производительности придётся навязывать задним числом.

## What Changes

- Cargo workspace: `crates/`, `showcase/`, `xtask/`, `benches/`, общий `[workspace.dependencies]`, `rust-toolchain.toml` (stable, wasm32).
- Крейт `prime-core`: конфиг (`PrimeConfig` через Leptos context), `ClassNames`, z-index менеджер, locale, базовые пропсы (`disabled`, `invalid`, `variant`, `size`), общий хелпер `pt` (pass-through) для корневого элемента.
- Крейт `prime-dom`: порт нужной части `DomHandler` (позиционирование, фокус, scroll), `focustrap`, `ripple`, портал; только `web-sys`, без `leptos-use`.
- Крейт `prime-theme` и `xtask gen-theme`: генерация статического CSS из токенов пресета Aura и стилей компонентов (npm-пакеты `@primeuix/themes` и `@primeuix/styles`, которые использует PrimeNG) командой `xtask`; результат коммитится.
- Крейт `prime-icons` и `xtask gen-icons`: 53 SVG-иконки PrimeNG, по компоненту на иконку за feature-флагом.
- `upstream.toml` и `xtask upstream-diff`: привязка к версии PrimeNG и поиск изменений в исходниках.
- `xtask gen-spec`: заготовка OpenSpec-спеки компонента из TS-исходников PrimeNG.
- Режим CSR и SSR/hydrate через features `ssr`/`hydrate` во всех крейтах; `showcase` на cargo-leptos.
- Меры против роста диска: `debug = "line-tables-only"`, `rust-lld`, инкрементальная сборка только в dev, `cargo-sweep`, `xtask clean`, минимум зависимостей.
- CI (GitHub Actions): на PR — fmt, clippy, nextest, wasm-bindgen-test, deny, бенчмарки (divan/dhat), бюджет размера wasm, только затронутые крейты; по расписанию раз в неделю и вручную — e2e (Playwright), сравнение скриншотов с PrimeNG, браузерные бенчмарки памяти.
- Лицензирование: MIT, `NOTICE` с копирайтом PrimeNG.
- Публикация на crates.io **не входит** в изменение.

## Capabilities

### New Capabilities
- `workspace-layout`: структура Cargo workspace, именование крейтов `prime-<name>`, feature-флаги `ssr`/`hydrate`, тулчейн, лицензия и `NOTICE`.
- `core-runtime`: `prime-core` и `prime-dom` — конфиг, контекст, ClassNames, z-index, pass-through, DOM-хелперы.
- `theme-css`: генерация статического CSS из токенов Aura и его подключение.
- `icon-set`: крейт `prime-icons` и генерация иконок из PrimeNG.
- `upstream-tracking`: закрепление версии PrimeNG, `upstream-diff`, генерация заготовок спек.
- `disk-footprint`: ограничения на размер `target/` и профили сборки.
- `ci-pipeline`: набор проверок на PR и еженедельные проверки (e2e, скриншоты, браузерные бенчмарки).
- `benchmarking`: бенчмарки SSR-рендера и аллокаций, бюджеты размера wasm, порог регрессий.

### Modified Capabilities
<!-- Нет: существующих спеков в openspec/specs/ нет. -->

## Impact

- Весь код репозитория создаётся с нуля; затрагиваются `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/workflows/`, `.gitignore` (добавить `nul`), `NOTICE`, `LICENSE`, `upstream.toml`.
- Зависимости: `leptos` 0.8, `web-sys`, `wasm-bindgen`; dev: `divan`, `dhat`, `wasm-bindgen-test`. На стороне CI: `cargo-nextest`, `cargo-deny`, `cargo-sweep`, Playwright (только в еженедельном job).
- Читается (только чтение) `C:\Projects\primeng`; в репозиторий он не копируется.
- Компоненты (Button и далее) в это изменение не входят: каждый идёт отдельным change.
