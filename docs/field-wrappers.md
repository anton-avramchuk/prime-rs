# Обёртки полей: ButtonGroup, Fluid, IconField, FloatLabel, IftaLabel

Порт `p-buttongroup`, `p-fluid`, `p-iconfield` с `p-inputicon`, `p-floatlabel` и `p-iftalabel` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-buttongroup::ButtonGroup` | `buttongroup/buttongroup.ts`, `buttongroup/style/buttongroupstyle.ts` |
| `prime-fluid::Fluid` | `fluid/fluid.ts` (стилей нет: `FluidStyle` лишь задаёт класс `p-fluid`) |
| `prime-iconfield::IconField`, `InputIcon` | `iconfield/iconfield.ts`, `inputicon/inputicon.ts`, их `style/*style.ts` |
| `prime-floatlabel::FloatLabel` | `floatlabel/floatlabel.ts`, `floatlabel/style/floatlabelstyle.ts` |
| `prime-iftalabel::IftaLabel` | `iftalabel/iftalabel.ts`, `iftalabel/style/iftalabelstyle.ts` |
| CSS | `@primeuix/styles/{buttongroup,floatlabel,iftalabel,iconfield}` — `cargo xtask gen-theme`; для `fluid` и `inputicon` модулей нет (правила `.p-inputicon` входят в `iconfield`) |
| Демонстрации | `apps/showcase/doc/{buttongroup,fluid,iconfield,floatlabel,iftalabel}/*-doc.ts`, а также `floatlabel` и `iftalabel` на страницах InputText, InputNumber, Select, Textarea |

Спеки: `openspec/specs/{buttongroup,fluid,iconfield,floatlabel,iftalabel}`.

Все пять — контейнеры без логики: `children`, `style`, `style_class` и `pt.root`. Поле и `<label for>` остаются в разметке пользователя, как в PrimeNG.

## Что совпадает с PrimeNG

- Классы корня и их порядок: `p-buttongroup p-component` (`role="group"`), `p-fluid`, `p-iconfield p-iconfield-left|right`, `p-inputicon`, `p-floatlabel p-floatlabel-over|on|in`, `p-iftalabel`; затем `style_class`, затем класс из `pt.root`.
- Положение значка, подъём и размер метки, слитные кнопки, цвета и скругления — целиком в CSS темы; компоненты только ставят классы.
- `iconPosition` по умолчанию `left` (в документации PrimeNG сказано «right», но код ставит `left`); поскольку CSS позиционирует значок по `:first-child`/`:last-child`, класс `p-iconfield-left|right` информационный.
- `Fluid` без явного `fluid` у потомка заставляет `Button`, `InputText`, `Textarea`, `InputNumber` и `Select` вести себя как при `fluid = true`; явное значение (в том числе `false`) главнее — `hasFluid = fluid() ?? !!pcFluid`.

## Отличия

| Отличие | Причина |
|---|---|
| Корни — `<span>`/`<div>`, а не хосты `<p-…>` | Хост в Angular — деталь фреймворка. |
| Контекст `Fluid` — `prime_core::fluid` (`provide_fluid`, `in_fluid`, `resolve_fluid`, `fluid_prop`); потребители читают его один раз при создании | Предок не меняется за время жизни потомка. Вместо `inject(Fluid)` в PrimeNG. |
| `Fluid` кладёт контекст в дочернего владельца и держит его до очистки родителя | `provide_context` в компоненте иначе протёк бы на соседей, идущих после `Fluid` в том же владельце. |
| `fluid` у `Button`, `InputText`, `Textarea`, `InputNumber`, `Select` — `OptProp<bool>`, а не `Prop<bool>` | Нужно отличать «не задано» от `false`. Вызовы `fluid=true` и `fluid=сигнал_Option` совместимы; сигнал `bool` теперь оборачивается в `Some`. `ToggleButton` и `SelectButton` не меняются: у них в PrimeNG нет наследования. |
| Блоки `/* For PrimeNG */` не переносятся | Они нацелены на хосты `p-button` (`.p-buttongroup p-button …`) и класс Angular Forms `.ng-invalid.ng-dirty`; базовый CSS тем уже содержит `.p-buttongroup .p-button …` и `:has(.p-invalid) label`. |
| `FloatLabelVariant` — перечисление, `variant` может быть сигналом | Класс корня заменяется без пересоздания детей; `write_classes` — чистая функция с табличным тестом. |
| В демонстрациях нет иконок `user`, `lock`, `bookmark`, `heart`, `time`, `trash` (из имеющихся — `search`, `eye`, `check`, `times`, `plus`, `pencil`, `trash`, `spinner`); в `template` у IconField вместо логотипа PrimeVue — простой SVG | В `prime-icons` их нет или нужен CDN. |
| Демонстрации с Angular Forms не переносятся; `invalid` в `floatlabel` и `iftalabel` следует за пустотой поля | Формы вне объёма. |
| В демонстрации Fluid вместо `AutoComplete`, `DatePicker`, `Password` — `InputNumber`, `Select`, `Textarea`; рядом показан тот же набор без `Fluid` | Этих компонентов в проекте пока нет. |
| В `fluid` нет ни CSS, ни темы | Как в PrimeNG: класс `p-fluid` ничего не стилизует, ширину задают сами потомки. |

Другие компоненты, читающие `Fluid` в PrimeNG (`AutoComplete`, `DatePicker`, `Password`, `InputMask`, `MultiSelect` и другие), получат тот же контекст после появления: достаточно `fluid_prop(fluid)` в начале компонента.

## Производительность

SSR, аллокаций на экземпляр (бюджет `benches/alloc-budgets.toml`): ButtonGroup с тремя кнопками-заглушками — 45 (50), Fluid с `InputText` и `Button` — 31 (34), IconField со значком и полем — 21 (23), FloatLabel — 9 (10), IftaLabel — 9 (10). Вклад в сырой wasm — в таблице ниже (бюджеты `benches/budgets.toml`).

| Компонент | Вклад в сырой wasm, Б | Бюджет, Б |
|---|---|---|
| Buttongroup | 73687 | 81000 |
| Fluid | 75238 | 83000 |
| Iconfield | 112697 | 124000 |
| Floatlabel | 91149 | 100000 |
| Iftalabel | 71920 | 79000 |

Тесты: SSR-тесты крейтов (структура, классы, порядок, `style`/`style_class`/`pt`, паритет статической и реактивной ветвей, варианты и позиции, наследование `Fluid` всеми пятью потребителями, явное `fluid = false`, вложенность и соседи вне `Fluid`), SSR-тесты наследования в каждом из пяти потребителей, e2e Playwright (расположение значка, подъём метки, слитные кнопки, растягивание по ширине).
