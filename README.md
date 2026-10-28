# prime-rs

Порт компонентов [PrimeNG](https://github.com/primefaces/primeng) на Rust и [Leptos](https://leptos.dev) 0.8.
Один компонент — один крейт (`crates/prime-<name>`). Порт синхронизирован с PrimeNG 21.1.10 (см. `upstream.toml`, `NOTICE`).

## Структура

```
crates/prime-core    конфиг, ClassNames, z-index, pass-through
crates/prime-dom     позиционирование, focus trap, портал, ripple
crates/prime-theme   статический CSS темы (Aura)
crates/prime-icons   SVG-иконки PrimeNG
crates/prime-<name>  компоненты
xtask/               генераторы и служебные команды (cargo xtask)
openspec/            спецификации и изменения
```

Сборка, диск и команды: [docs/build.md](docs/build.md).

## Процесс

Работа ведётся через [OpenSpec](https://github.com/Fission-AI/OpenSpec): один change на компонент, заготовка спеки создаётся командой `cargo xtask gen-spec <name>`.

## Лицензия

MIT, см. `LICENSE` и `NOTICE`.
