## ADDED Requirements

### Requirement: Один компонент — один крейт
Каждый компонент PrimeNG SHALL портироваться в отдельный крейт `crates/prime-<name>`, где `<name>` совпадает с именем каталога компонента в PrimeNG. Общий код SHALL жить в `prime-core`, `prime-dom`, `prime-theme` и `prime-icons`, а не в крейтах компонентов.

#### Scenario: Именование крейта компонента
- **WHEN** добавляется порт компонента `button`
- **THEN** создаётся крейт `crates/prime-button`, и он указан в workspace как участник

#### Scenario: Компонент не дублирует общий код
- **WHEN** компоненту нужны ClassNames, z-index или позиционирование
- **THEN** он зависит от `prime-core` или `prime-dom`, а не содержит собственную копию

### Requirement: Единый workspace и версии зависимостей
Репозиторий SHALL быть одним Cargo workspace, в котором версии внешних зависимостей объявлены один раз в `[workspace.dependencies]`, а крейты на них ссылаются.

#### Scenario: Сборка workspace
- **WHEN** выполняется `cargo check --workspace`
- **THEN** сборка завершается без ошибок, и в `Cargo.lock` нет нескольких версий `leptos`

### Requirement: Режимы CSR и SSR/hydrate
Каждый крейт, зависящий от Leptos, SHALL собираться в режимах CSR, `ssr` и `hydrate` через одноимённые feature-флаги. Код, обращающийся к `window` или `document`, SHALL выполняться только на клиенте.

#### Scenario: SSR-сборка
- **WHEN** крейт собирается с `--features ssr` для нативной цели
- **THEN** сборка проходит, а рендер компонента в строку не обращается к DOM API

#### Scenario: Клиентская сборка
- **WHEN** крейт собирается для `wasm32-unknown-unknown` с `--features hydrate`
- **THEN** сборка проходит без ошибок

### Requirement: Зафиксированный тулчейн
Репозиторий SHALL закреплять версию Rust в `rust-toolchain.toml` (stable) вместе с таргетом `wasm32-unknown-unknown` и компонентами `rustfmt`, `clippy`.

#### Scenario: Свежий клон
- **WHEN** разработчик клонирует репозиторий и запускает `cargo build`
- **THEN** используется закреплённый тулчейн, а нужный таргет устанавливается автоматически

### Requirement: Лицензия и атрибуция
Репозиторий SHALL распространяться под MIT, а файл `NOTICE` SHALL содержать копирайт PrimeNG и закреплённую версию, из которой выполнен порт.

#### Scenario: Проверка NOTICE
- **WHEN** просматривается корень репозитория
- **THEN** присутствуют `LICENSE` и `NOTICE` с упоминанием PrimeNG

### Requirement: Служебные файлы Windows
В `.gitignore` SHALL быть исключены файлы `nul`, чтобы они не попадали в коммиты.

#### Scenario: Случайный файл nul
- **WHEN** в рабочей копии появляется файл `nul`
- **THEN** `git status` его не показывает
