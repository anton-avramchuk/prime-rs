# Design

## Context

Требования — в `specs/togglebutton`. Исходники PrimeNG 21.1.10: `packages/primeng/src/togglebutton/togglebutton.ts`, `togglebutton/style/togglebuttonstyle.ts`, демонстрации `apps/showcase/doc/togglebutton`. Образцы — `crates/prime-toggleswitch` (`docs/toggleswitch.md`: `checked: RwSignal<bool>`, `on_change`, `rx`-замыкания, `macro_rules!` разметки, браузерные тесты) и `crates/prime-button` (иконка — `ViewFn` в оболочке `span`).

## Decisions

### D1. Состояние
`checked: Option<RwSignal<bool>>`. Без него компонент создаёт внутренний `RwSignal` с `default_checked` — в PrimeNG состояние тоже всегда хранит сам компонент (`checked`), нативного `<input>` нет, поэтому DOM-состояние выравнивать не нужно. Статической ветви нет: любое нажатие меняет `class`, `aria-pressed`, `data-*` и метку, так что разметка всегда строится реактивными замыканиями `rx(props, pt, fn)`.

### D2. Переключение
Обработчик `toggle` как в PrimeNG: `if !disabled && !(allow_empty == Some(false) && checked)`. Вешается на `click` и на `keydown` (`Enter`, `Space` по `event.code`, с `preventDefault`). `allow_empty` — `Prop<Option<bool>>` в смысле PrimeNG: ограничение действует только при явном `false`. `on_change` получает `ToggleButtonChange { checked, original_event }`.

### D3. Разметка и атрибуты
Корень — `<div>` (вместо хоста `<p-togglebutton>`). Порядок классов как в `togglebuttonstyle.ts`: `p-togglebutton p-component`, `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` / `p-togglebutton-lg p-inputfield-lg`, `p-togglebutton-fluid`, затем `style_class` и `pt.root`. `tabindex`: заданное значение, иначе `0`, при `disabled` `-1`. `data-p` — `checked invalid <size>` (`DataP`). Пустую метку заменяет пробел (`' '`), как в шаблоне PrimeNG. Чистая логика (классы, `data-p`) — в `state.rs` с табличным тестом, как у ToggleSwitch.

### D4. Иконки и слоты
`on_icon`/`off_icon` — `Option<ViewFn>` в оболочке `span.p-togglebutton-icon` (+ `-left`/`-right`): шрифта PrimeIcons нет, оболочка нужна для CSS (`order`). `icon_template: Callback<bool, AnyView>` заменяет иконку по умолчанию, `content_template: Callback<bool, AnyView>` — всё содержимое после `span.p-togglebutton-content`. `IconPos` — собственный enum крейта (`Left`, `Right`): зависимость от `prime-button` ради одного перечисления не нужна.

### D5. CSS
В `togglebuttonstyle.ts` блок называется `/* For PrimeNG (iconPos) */`, а `xtask gen-theme` ищет точный маркер `/* For PrimeNG */`. Правим `primeng_extra_block`: принимать маркер `/* For PrimeNG` с необязательным пояснением до `*/`, и добавить `togglebutton` в `primeng_extra`. Правило `.p-togglebutton.ng-invalid.ng-dirty` относится к классам Angular Forms и безвредно (в Leptos их нет).

### D6. Демонстрации
`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`. В `prime-icons` нет `lock` и `lock-open`, поэтому в `customized` вместо них `CheckIcon` и `TimesIcon` (в исходнике `onIcon`/`offIcon` заданы дважды, действуют `pi-lock`/`pi-lock-open`). Демонстрации с формами пропускаются.

### D7. Тесты
SSR: структура, метки по умолчанию, иконки и позиция, слоты, классы и `data-p` для всех сочетаний, атрибуты, `tabindex`, `pt`. Браузерные (`tests/web.rs`, wasm-bindgen-test): клик, Enter и Space, `disabled`, `allow_empty`, один вызов `on_change` на нажатие, фокус и потеря фокуса. Новый крейт с `tests/web.rs` добавляется в браузерный job `ci.yml` (3 места).

## Risks / Trade-offs

- [Нет статической ветви, поэтому аллокаций больше, чем у статичных компонентов] → бюджет по измерению, как у ToggleSwitch.
- [Изменение `xtask gen-theme` затрагивает все компоненты] → маркер расширяется обратно совместимо; после правки `cargo xtask gen-theme` не должен менять CSS остальных компонентов (проверить `git diff`).

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
