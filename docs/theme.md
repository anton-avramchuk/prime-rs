# Тема

`prime-theme` содержит статический CSS темы Aura. Он не генерируется в рантайме и не требует Node при сборке:
файлы лежат в `crates/prime-theme/css/` и подключаются через `include_str!`.

```rust
let css = prime_theme::stylesheet(&["button", "badge"]); // базовый CSS + CSS выбранных компонентов
```

- `BASE` — переменные `--p-*` (палитры, семантические токены, светлая и тёмная схема), глобальные стили и `.p-hidden-accessible`.
- Тёмная схема включается селектором `DARK_MODE_SELECTOR` (`.p-dark`) на корневом элементе.
- CSS компонента доступен, если включён его feature (`button`, `badge`, …, `all-components`); остальные имена пропускаются.

## Регенерация

```sh
cargo xtask gen-theme
```

Нужны Node и npm. Команда ставит `@primeuix/themes`, `@primeuix/styles`, `@primeuix/styled` и `@primeuix/utils` версий из `upstream.toml`
в кэш вне репозитория (`~/.cache/prime-rs`, переопределяется `PRIME_RS_CACHE`) без запуска скриптов пакетов, затем `xtask/theme/extract.mjs`
строит CSS тем же кодом, что и PrimeNG (`Theme.getCommon`, `Theme.getComponent`, разрешение `dt()`). Результат детерминирован.

Список компонентов задаётся в `upstream.toml` (`[theme] components`): новый компонент добавляется в список вместе со своим портом.
Размер: `base.css` ≈ 19 КБ, CSS компонента — от 3 до 35 КБ (минифицированный).
