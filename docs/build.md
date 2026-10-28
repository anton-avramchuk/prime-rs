# Сборка

Тулчейн закреплён в `rust-toolchain.toml` (Rust 1.98.0, таргет `wasm32-unknown-unknown`, rustfmt и clippy): rustup установит всё сам при первой сборке.

```sh
cargo check --workspace
cargo check -p prime-core --target wasm32-unknown-unknown --features hydrate   # клиент
cargo check -p prime-core --features ssr                                       # сервер
cargo deny check
cargo xtask --help
```

Режимы Leptos включаются features `ssr` и `hydrate` в каждом крейте; по умолчанию — CSR.

## Линкер

`.cargo/config.toml` использует `rust-lld` для `x86_64-pc-windows-msvc` и `lld` для Linux (пакет `lld` должен быть установлен).

## Размер `target/`

Профили (см. корневой `Cargo.toml`): в dev `debug = "line-tables-only"`, для зависимостей `debug = false`; в release `lto = "thin"`, `strip`, без инкрементальной сборки. В CI выставляйте `CARGO_INCREMENTAL=0`.

Чтобы несколько клонов и проектов не плодили свои `target/`, вынесите каталог сборки в общее место:

PowerShell (постоянно для пользователя):
```powershell
[Environment]::SetEnvironmentVariable("CARGO_TARGET_DIR", "D:\cargo-target", "User")
```

Linux и macOS (`~/.profile`):
```sh
export CARGO_TARGET_DIR="$HOME/.cache/cargo-target"
```

Размер `target/` после `cargo check --workspace` (только Leptos, без компонентов) на момент создания каркаса: 542 МБ.

## Команды xtask

```sh
cargo xtask upstream-diff [--to <rev>]   # какие портированные компоненты изменились в PrimeNG
cargo xtask gen-spec <имя> [--out файл]  # заготовка спеки из исходников PrimeNG
cargo xtask clean [--days N]             # cargo-sweep: удалить артефакты target/ старше N дней (по умолчанию 7)
```

`upstream-diff` и `gen-spec` читают `upstream.toml`; путь к клону PrimeNG можно переопределить `PRIMENG_PATH`.
Для `clean` нужен `cargo install cargo-sweep`. Список «портированных» компонентов — это `crates/prime-<имя>` без `core`, `dom`, `theme`, `icons`.

## Браузерные тесты (wasm-bindgen-test)

```sh
cargo install wasm-bindgen-cli --version 0.2.129 --locked   # версия должна совпадать с wasm-bindgen в Cargo.lock
# chromedriver под установленный Chrome (https://googlechromelabs.github.io/chrome-for-testing/)
export CHROMEDRIVER=/путь/к/chromedriver
cargo test -p prime-dom --target wasm32-unknown-unknown --test web
```

Раннер `wasm-bindgen-test-runner` прописан в `.cargo/config.toml`. Без `CHROMEDRIVER` раннер ищет `chromedriver` в `PATH`.
Тесты помечены `#![cfg(target_arch = "wasm32")]`, поэтому обычный `cargo test` на хосте их пропускает.

## Showcase

```sh
cd showcase
cargo leptos serve    # http://127.0.0.1:3000, SSR + гидрация; cargo leptos build — только сборка
```

Страница подключает тему встроенным `<style>` (`prime_theme::stylesheet`), показывает все иконки и переключатель светлой и тёмной схемы (класс `p-dark` на `<html>`).
`cargo-leptos` собирает клиентскую часть в отдельный каталог `target/front`, поэтому `target/` примерно вдвое больше, чем у обычной сборки: чистите его `cargo xtask clean`.
