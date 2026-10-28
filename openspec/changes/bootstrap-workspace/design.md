# Design

## Context

Мотивация и состав изменения описаны в `proposal.md`, требования — в `specs/`. Что важно для реализации:

- Репозиторий пуст, кроме `openspec/`. Разработка на Windows 11 (MSVC), CI на ubuntu-latest. Установлены rustc/cargo 1.98, `trunk`, `cargo-leptos`, цель `wasm32-unknown-unknown`.
- Исходники PrimeNG лежат в `C:\Projects\primeng` (v21.1.10): `packages/primeng/src/<component>` (≈117 каталогов, из них ≈80–90 компонентов), `packages/primeng/src/icons` (55 каталогов, 53 иконки, каждая — один `<svg:path d=… fill="currentColor"/>`), `apps/showcase`.
- **Токены Aura и CSS компонентов не лежат в репозитории PrimeNG**: `packages/themes/src/presets/aura/*` лишь реэкспортирует `@primeuix/themes`, а `button/style/buttonstyle.ts` импортирует CSS из `@primeuix/styles/button`. `node_modules` в `C:\Projects\primeng` не установлен. Источник для `gen-theme` — npm-пакеты `@primeuix/themes` и `@primeuix/styles` нужных версий.
- Классы компонентов определены в TS (`classes = { root: ({instance}) => [...] }`), то есть их можно разбирать скриптом, но не импортировать как данные.

## Goals / Non-Goals

**Goals:**
- Волна 0: после изменения можно добавить первый компонент (Button) одним следующим change, не трогая инфраструктуру.
- Обычная сборка и тесты не требуют Node и PrimeNG; они нужны только генераторам `xtask` и еженедельному CI.
- Любая генерация детерминирована и повторяема.

**Non-Goals:**
- Портирование компонентов (включая Button) и пресетов кроме Aura.
- Интеграция с формами, публикация на crates.io, SSR-стриминг.
- Рантайм-генерация тем (`updatePreset`, смена темы на лету, кроме светлой/тёмной схемы).

## Decisions

### D1. Раскладка workspace
```
crates/prime-core  prime-dom  prime-theme  prime-icons  prime-<component>…
showcase/   xtask/   benches/   .github/workflows/
upstream.toml  rust-toolchain.toml  .cargo/config.toml  deny.toml  NOTICE
```
`resolver = "3"`, `members = ["crates/*", "benches/*", "showcase", "xtask"]` (в `benches/` два крейта: `render` и `wasm-size`). Базовые классы PrimeNG (`basecomponent`, `baseinput`, `baseeditableholder`) становятся модулями `prime-core`, а не крейтами: в Rust это трейты и хелперы без собственного рантайма. Утилитарные каталоги (`dom`, `utils`, `classnames`, `focustrap`, `ripple`) делятся между `prime-core` и `prime-dom`.
*Альтернатива*: крейт на каждый каталог PrimeNG — даёт ~117 крейтов, из которых треть пустые обёртки и лишние минуты на каждую проверку.

### D2. Один набор features
Каждый крейт с Leptos: `default = []`, features `ssr = ["leptos/ssr"]`, `hydrate = ["leptos/hydrate"]`, CSR по умолчанию. Код, обращающийся к `web_sys::window()`, прячется за `#[cfg(feature = "hydrate")]`/`cfg(target_arch = "wasm32")` либо выполняется внутри `Effect`, который не запускается на сервере.
*Альтернатива*: только CSR — проще, но SSR-бенчмарки (D8) и cargo-leptos showcase потеряли бы смысл, а добавить SSR позже означает переписать DOM-код всех компонентов.

### D3. CSS-классы
`ClassNames` в `prime-core` — построитель, пишущий в один `String` с заранее выделенной ёмкостью (`with_capacity`), без промежуточных `Vec<String>`; в реактивной части результат оборачивается в `Memo<String>`, чтобы атрибут `class` обновлялся только при изменении. Имена классов копируются из `classes` PrimeNG дословно (спека `core-runtime`).

### D4. Pass-through
`pt` — структура `Pt { class: Option<…>, attrs: … }` на корень и именованные части (например, `PtRoot`); применяется через `.attr()` и объединение классов. Полный набор `pt:*`-хуков PrimeNG и unstyled-режим не входят в волну 0 (см. Non-Goals), но тип `Pt` проектируется расширяемым.

### D5. Генерация темы
`xtask gen-theme`:
1. Устанавливает `@primeuix/themes`, `@primeuix/styles`, `@primeuix/styled`, `@primeuix/utils` закреплённых версий (`npm install --ignore-scripts`) в кэш **вне репозитория** (`~/.cache/prime-rs`, переопределяется `PRIME_RS_CACHE`) и запускает `xtask/theme/extract.mjs`.
2. Скрипт строит CSS тем же кодом, что и PrimeNG (`Theme.getCommon`/`getComponent` из `@primeuix/styled`, разрешение `dt()`), и печатает JSON `{base, components}`. Rust-часть записывает `css/base.css` и `css/components/<name>.css`, список компонентов и селектор тёмного режима берутся из `upstream.toml` (`[theme]`).
3. Результат **коммитится** в `crates/prime-theme/css/`. Обычная сборка и CI не требуют Node; регенерация нужна только при обновлении upstream.
Размер: токены Aura и CSS компонентов — десятки–сотни КБ текста; в репозитории это приемлемо, а скриншоты и бинарники не коммитятся.
*Альтернативы*: (a) генерация в `build.rs` — тянет Node в каждую сборку и ломается в офлайн-CI; (b) порт `@primeuix/styled` на Rust с рантайм-генерацией — соответствует PrimeNG, но противоречит цели нулевого рантайм-оверхеда (см. требования к памяти и производительности).

### D6. Иконки
`xtask gen-icons` читает `packages/primeng/src/icons/*/<name>.ts`, достаёт `d` и `viewBox` (регулярным разбором шаблона `<svg:path … d="…"`; при несоответствии — ошибка, а не пропуск) и пишет `crates/prime-icons/src/<name>.rs` + features в `Cargo.toml`. Общий компонент `Icon` (размер, `spin`, a11y) вынесен в `prime-icons`, чтобы данные иконки были константами `&'static str`.
*Альтернативы*: сторонний набор (Lucide) и шрифт отклонены пользователем; PrimeNG-иконок всего 53.

### D7. Overlay и DOM
`prime-dom` напрямую использует `web-sys` (минимальный набор features `Window`, `Document`, `Element`, `DomRect`, `HtmlElement`, `EventTarget`, `KeyboardEvent`), без `leptos-use`. Позиционирование — порт `absolutePosition`/`relativePosition` из `DomHandler` с теми же правилами переворота. Портал — `leptos::portal::Portal`. Анимации — классы CSS и `transition`, как `p-motion`.
*Альтернатива*: `leptos-use` — быстрее писать, но тащит большой граф зависимостей в wasm и в `target/`.

### D8. Бенчмарки
- `benches/render` — крейт с `divan` (`render`), `dhat` (`alloc`) и медианой времени в JSON (`time`); компонент рендерится в строку через `leptos::ssr` (одна и та же функция для 1 и 1000 экземпляров).
- Аллокации: `dhat` в тестовом режиме (`dhat::Profiler::builder().testing()`), результат пишется в JSON.
- Размер wasm: `xtask wasm-size` собирает минимальное приложение с одним компонентом, считает вклад как разницу размера сырого `.wasm` (release, без `wasm-opt`) с пустым приложением `benches/wasm-size` и сравнивает с `benches/budgets.toml`.
- Все отчёты — `target/bench/*.json` вида `{"metrics": {имя: число}}`. Базовая линия — артефакт `bench-baseline` последнего успешного прогона `dev`; `xtask bench-compare` сравнивает каталоги (порог 10 %, только комментарий в PR, `--fail-over` включает блокировку).
- Браузерные замеры памяти — Playwright в еженедельном job (CDP `Performance.getMetrics`: `JSHeapUsedSize`, `Nodes`): время создания 1000 компонентов и рост кучи и DOM после 20 циклов создания и удаления.

### D9. CI
Файлы `.github/workflows/`:
- `ci.yml` (PR и push): `fmt` → `clippy -D warnings` → `nextest` → `wasm-bindgen-test` (headless Chrome) → `cargo-deny` → `bench`. Определение затронутых крейтов — `xtask affected` (`cargo metadata` + `git diff`); изменения в `prime-core`, `prime-dom`, `Cargo.toml`, `Cargo.lock`, `xtask`, `.github/**` и др. включают полный прогон. Приложения (`showcase`, `wasm-size`, `render-bench`) проверяет отдельный job `apps`.
- `weekly.yml` (`schedule`, `workflow_dispatch`, PR с меткой `run-e2e`): клон PrimeNG на коммите из `upstream.toml`, сборка его showcase, сборка нашего showcase, Playwright (e2e + сравнение скриншотов + браузерные бенчмарки), артефакты `retention-days: 7`.
- Кэш — `Swatinem/rust-cache` с `cache-on-failure: false`, перед сохранением `cargo-sweep`; `cargo-machete` по расписанию.
Стратегия кэша выбрана так, чтобы размер кэша был ограничен; `sccache` не используется.

### D10. Профили сборки
```toml
[profile.dev]            debug = "line-tables-only"  incremental = true
[profile.dev.package."*"] debug = false opt-level = 1
[profile.release]        lto = "thin"  codegen-units = 1  panic = "abort"  strip = true  incremental = false
```
В `.cargo/config.toml`: `rust-lld` для `x86_64-pc-windows-msvc` и `x86_64-unknown-linux-gnu`; `CARGO_INCREMENTAL=0` в CI. Это снижает размер `target/` без потери отладки строк.

### D11. upstream.toml и заготовки спек
`upstream.toml` хранит `version`, `commit`, `path` (локальный клон; перекрывается `PRIMENG_PATH`) и версии `@primeuix/*`. `xtask upstream-diff` вызывает `git diff --name-only <pinned>..<target> -- packages/primeng/src/<component>` для портированных компонентов (список — крейты `crates/prime-<имя>` без инфраструктурных). `xtask gen-spec <name>` разбирает `<name>.ts` и `style/<name>style.ts` и заполняет шаблон спеки.

## Risks / Trade-offs

- [Парсинг TS регулярными выражениями хрупок] → генераторы падают с ошибкой при неожиданной форме исходника, на ключевых файлах есть snapshot-тесты (`insta`), версия upstream закреплена.
- [`@primeuix/*` — внешние npm-пакеты, не часть репозитория PrimeNG] → версии закреплены в `upstream.toml`, результат коммитится, Node нужен только для регенерации.
- [Рост репозитория из-за закоммиченного CSS и сгенерированных иконок] → только текст, повторная генерация детерминирована, дельты в git небольшие; бинарники не коммитятся.
- [Тесты SSR не ловят баги DOM] → `wasm-bindgen-test` на PR и Playwright в еженедельном job.
- [Еженедельный job зависит от сборки Angular showcase и может стать нестабильным] → его результат не блокирует PR, артефакты хранятся 7 дней.
- [Бюджеты wasm и порог 10 % выбраны без измерений] → первая версия бюджетов записывается после появления первого компонента и пересматривается.

## Migration Plan

Миграции нет: репозиторий создаётся с нуля. Откат — `git revert` соответствующего коммита. Ветка `dev` защищается проверкой `ci.yml` после первого зелёного прогона.

## Open Questions

- Нужно ли подключать `cargo-leptos` к CI на PR (сборка showcase) или достаточно сборки в еженедельном job? Решается после замера времени сборки; на спеки не влияет.
- Точные значения бюджета размера wasm и срока устаревания для `cargo-sweep` — определяются по первым измерениям.
