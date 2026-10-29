# Design

## Context

Требования — в `specs/{buttongroup,fluid,iconfield,floatlabel,iftalabel}`. Исходники PrimeNG 21.1.10: `buttongroup/buttongroup.ts`, `fluid/fluid.ts`, `iconfield/iconfield.ts`, `inputicon/inputicon.ts`, `floatlabel/floatlabel.ts`, `iftalabel/iftalabel.ts` и их `style/*style.ts`. Образцы — `crates/prime-inputgroup` (контейнер с слотом и `pt.root`, два компонента в одном крейте), `crates/prime-toolbar`/`prime-card` (контейнер с `style_class`), `prime_core::config` (контекст через `provide_context`).

Наблюдения по исходнику, влияющие на решения:
- Все шесть — «глупые» контейнеры: шаблон `<ng-content>`, классы корня и `pt`; логики нет, кроме выбора класса по `variant` и `iconPosition`.
- Положение значка, подъём метки, цвета и скругления целиком в CSS темы: `@primeuix/styles/{buttongroup,floatlabel,iftalabel,iconfield}` (в `iconfield` — и правила `.p-inputicon`). Для `fluid` модуля стилей нет (`FluidStyle` только задаёт класс `p-fluid`).
- Блоки `/* For PrimeNG */` у `buttongroup` нацелены на хост-элемент `p-button` (`.p-buttongroup p-button …`) и у `floatlabel`/`iftalabel` на `.ng-invalid.ng-dirty` — к нашей разметке они неприменимы. Базовый CSS тем уже содержит `.p-buttongroup .p-button …` и `:has(.p-invalid) label`, чего достаточно; `primeng_extra` не расширяется.
- `Fluid` ничего не стилизует: потомки (`Button`, `InputText`, `Textarea`, `InputNumber`, `Select`, а также `AutoComplete`, `DatePicker` и другие, которых у нас пока нет) читают `hasFluid = fluid() ?? !!pcFluid` через DI и сами добавляют свой класс растягивания. Значит, настоящая работа — потребители.
- `iconPosition` у `IconField` по умолчанию `left` (в документации сказано «right», но код ставит `left`); поскольку CSS позиционирует значок по `:first-child`/`:last-child`, класс `p-iconfield-left|right` информационный. Переносим поведение кода.

## Decisions

### D1. Крейты
`prime-buttongroup`, `prime-fluid`, `prime-iconfield` (`IconField`, `InputIcon`), `prime-floatlabel`, `prime-iftalabel`. Каждый зависит только от `prime-core` (`ClassNames`, `Pt`, `OptProp`/`Prop`); у каждого `ssr`/`hydrate` features, rustdoc с примером, SSR-тесты. Компоненты — по образцу `InputGroup`: `children: Children`, `style_class: OptProp<String>`, `style: OptProp<String>`, `pt: Option<…Pt>`, где `…Pt { root: Pt }`; одиночные свойства (`variant`, `icon_position`) — `Prop<…>`. Все пять без статической ветви: разметка из одного элемента с замыканиями только там, где свойство может быть сигналом (`class` корня).

### D2. Контекст Fluid (`prime-core::fluid`)
`FluidContext` (маркерный тип), `provide_fluid()` (`provide_context(FluidContext)`), `in_fluid() -> bool` (`use_context::<FluidContext>().is_some()`), `resolve_fluid(explicit: Option<bool>) -> bool` (`explicit.unwrap_or_else(in_fluid)`). `Fluid` вызывает `provide_fluid()` при создании и рисует `div.p-fluid`. Потребители читают контекст **один раз при создании** (`let inherited = in_fluid()`), потому что предок не меняется.

### D3. Потребители fluid
В `Button`, `InputText`, `Textarea`, `InputNumber` и `Select` свойство `fluid` меняется с `Prop<bool>` на `OptProp<bool>` (вызовы `fluid=true` и `fluid=signal_of_option` совместимы; `fluid=signal_of_bool` теперь нужно оборачивать в `Some`, но в showcase и тестах таких вызовов нет). Эффективное значение — `fluid.get().unwrap_or(inherited)`: в статической ветви вычисляется один раз (признак `all_static!` не нарушается — контекст статичен), в реактивной читается в замыкании. `ToggleButton` и `SelectButton` не меняются (у них в PrimeNG нет наследования).

### D4. FloatLabel и IftaLabel
`FloatLabelVariant { Over (по умолчанию), On, In }` и классы `p-floatlabel p-floatlabel-over|on|in` (чистая функция `write_classes` с табличным тестом); замыкание `class` позволяет менять вариант без пересоздания детей. `IftaLabel` — один постоянный класс. Метка `<label for>` и поле остаются в разметке пользователя (как в PrimeNG).

### D5. IconField и InputIcon
`IconPosition { Left (по умолчанию), Right }`; классы `p-iconfield p-iconfield-left|right`; `InputIcon` — `span.p-inputicon` со слотом, иконка передаётся как дочерний элемент (SVG из `prime-icons` или свой).

### D6. CSS
`buttongroup`, `floatlabel`, `iftalabel` добавляются в `theme.components` (`iconfield` уже там); `cargo xtask gen-theme`, затем проверка `git diff` на неизменность CSS остальных компонентов. Для `fluid` модуля нет — не добавляется.

### D7. Демонстрации
Страницы `/buttongroup` (`basic` — три кнопки; `split` с иконкой — по `button/buttongroup-doc.ts`), `/fluid` (`basic`), `/iconfield` (`basic`, `sizes`, `template`, `floatlabel`, `iftalabel`), `/floatlabel` (`basic`, `variants`, `invalid`), `/iftalabel` (`basic`, `invalid`). В `template` у `iconfield` вместо логотипа PrimeVue — простой SVG. На страницы InputText, InputNumber, Select и Textarea добавляются демонстрации `floatlabel` и `iftalabel` из их документации (если они там есть); e2e каждой страницы и обновлённых мест. Иконки — `SearchIcon`, `UserIcon`/`LockIcon` недоступны в `prime-icons`, поэтому в демонстрациях используются только имеющиеся (`SearchIcon`, `SpinnerIcon`, `TimesIcon`, `CheckIcon`); различие фиксируется в документации.

### D8. Тесты
SSR: структура, классы и порядок, `style`/`style_class`/`pt`, варианты и позиции, вложенность, пустое содержимое; для Fluid — все пять потребителей (включая приоритет явного `false`, реактивное `fluid`, вложенность). Браузерных тестов нет: компоненты статичны, а поведение проверяют e2e (позиционирование и подъём метки, слитность кнопок, растягивание по ширине).

### D9. Бенчмарки
Сценарии `buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel` (по `-1` и `-1000`) и `case-*` для wasm; бюджеты по измерению.

## Risks / Trade-offs

- [Смена типа `fluid` в пяти существующих крейтах] → API остаётся совместимым для `fluid=true` и без значения; поведение без `Fluid` не меняется (тесты существующих крейтов остаются зелёными без правок, кроме тех, что явно подают `Signal<bool>`).
- [CSS темы меняется только добавлением модулей] → проверка `git diff` после `gen-theme`.
- [Нет браузерных тестов] → компенсируется e2e с вычисленными стилями (расположение значка, подъём метки, радиусы кнопок).
- [Иконок `user`, `lock` в `prime-icons` нет, демонстрации упрощены] → фиксируется в `docs/field-wrappers.md`.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
