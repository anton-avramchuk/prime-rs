# Иконки

Крейт `prime-icons` содержит 53 SVG-иконки PrimeNG (`packages/primeng/src/icons`, без `baseicon`) плюс `BlankIcon`.
Каждая иконка включается своим feature-флагом (имя каталога PrimeNG: `check`, `chevrondown`, …), `all-icons` включает все.

```toml
prime-icons = { path = "crates/prime-icons", features = ["check", "chevrondown"] }
```

```rust
view! { <CheckIcon /> }                         // декоративная: aria-hidden="true"
view! { <CheckIcon aria_label="Готово" /> }     // role="img" + aria-label
view! { <SpinnerIcon spin=true size=20u32 class="custom" /> }
```

Код создаётся командой `cargo xtask gen-icons` из исходников PrimeNG (путь — `upstream.toml`). Генерация детерминирована:
повторный запуск не меняет файлы. Тест `tests/upstream.rs` сверяет `d` и `viewBox` каждой иконки с исходником.

Отличие от PrimeNG: обёртка `<g clip-path>` + `clipPath` на весь `viewBox` не воспроизводится, она ничего не обрезает.

## Вклад в размер wasm

Сырой `.wasm` (release, без `wasm-opt`, `cargo build -p wasm-size --release --target wasm32-unknown-unknown --features case-…`):

| Случай | Размер, байт | Разница с baseline |
|---|---|---|
| `case-baseline` (пустой `div`) | 1 235 486 | — |
| `case-one-icon` (`CheckIcon`) | 1 243 662 | +8 176 |
| `case-all-icons` (54 иконки) | 1 362 853 | +127 367 (≈ 2,4 КБ на иконку) |

Не включённые иконки в бинарник не попадают: `case-one-icon` отличается от `case-all-icons` на ≈ 119 КБ.
