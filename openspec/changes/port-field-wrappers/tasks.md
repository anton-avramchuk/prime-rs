# Tasks

## 1. Контекст Fluid и потребители

- [x] 1.1 `prime-core::fluid`: `FluidContext`, `provide_fluid`, `in_fluid`, `resolve_fluid`; тесты (вне контекста, внутри, вложенность, явное значение главнее)
- [x] 1.2 `Button`, `InputText`, `Textarea`, `InputNumber`, `Select`: `fluid: OptProp<bool>` и разрешение через `resolve_fluid` (статическая и реактивная ветви); существующие тесты остаются зелёными, новые SSR-тесты на наследование в каждом крейте

## 2. Новые крейты

- [x] 2.1 `crates/prime-buttongroup` (`ButtonGroup`, `ButtonGroupPt`): крейт, компонент, SSR-тесты, rustdoc
- [x] 2.2 `crates/prime-fluid` (`Fluid`, `FluidPt`): крейт, компонент с `provide_fluid`, SSR-тесты со всеми потребителями, rustdoc
- [x] 2.3 `crates/prime-iconfield` (`IconField`, `InputIcon`, `IconPosition`, `IconFieldPt`, `InputIconPt`): крейт, компоненты, SSR-тесты, rustdoc
- [x] 2.4 `crates/prime-floatlabel` (`FloatLabel`, `FloatLabelVariant`, `FloatLabelPt`, `write_classes` с табличным тестом): крейт, компонент, SSR-тесты, rustdoc
- [x] 2.5 `crates/prime-iftalabel` (`IftaLabel`, `IftaLabelPt`): крейт, компонент, SSR-тесты, rustdoc

## 3. Тема, showcase, e2e

- [x] 3.1 Добавить `buttongroup`, `floatlabel`, `iftalabel` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 3.2 Страницы `/buttongroup`, `/fluid`, `/iconfield`, `/floatlabel`, `/iftalabel`, меню, маршруты, `stylesheet(&[..])`
- [x] 3.3 Демонстрации `floatlabel` и `iftalabel` на страницах InputText, InputNumber, Select, Textarea (по их документации PrimeNG)
- [x] 3.4 `e2e/tests/pages.json` и spec-файлы (`buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel`) плюс проверки новых демонстраций на существующих страницах; ждать `networkidle`, перезапустить сервер showcase перед прогоном; `npx playwright test`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-*` и бюджеты для `buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel` (сначала `cargo bench -p render-bench --bench alloc`)

## 5. Интеграция

- [ ] 5.1 `docs/field-wrappers.md` (отличия от PrimeNG: контекст Fluid, демонстрации без недостающих иконок, правила `/* For PrimeNG */`); clippy с `--all-features` без исключений, тесты, deny, machete, `openspec validate port-field-wrappers`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
