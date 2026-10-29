# Design

## Context

Требования — в `specs/selectbutton`. Исходники PrimeNG 21.1.10: `packages/primeng/src/selectbutton/selectbutton.ts`, `selectbutton/style/selectbuttonstyle.ts`, демонстрации `apps/showcase/doc/selectbutton`. Образцы — `crates/prime-togglebutton` (`docs/togglebutton.md`) и группа `prime-radiobutton` (общий сигнал значения на всю группу). SelectButton в PrimeNG — это `p-togglebutton` на каждый вариант, у которого `[ngModel]="isSelected(option)"`, `onLabel = offLabel = label`, `allowEmpty = getAllowEmpty()`, `size`, `fluid`, `styleClass` и `pt = ptm('pcToggleButton')`; своего `tabindex` и стрелочной навигации в шаблоне нет.

## Decisions

### D1. Варианты и значение
`SelectButtonOption { label: String, value: String, disabled: bool }` (`Clone`, `Default` для `disabled`, конструктор `new(label, value)`), `options: Prop<Vec<SelectButtonOption>>`. Значения — строки, как у группы RadioButton. Поля произвольного объекта (`optionLabel`, `optionValue`, `optionDisabled`, `dataKey`) не переносятся: их заменяет типизированный вариант. Сравнение значений — по равенству строк (в PrimeNG `equals` из `@primeuix/utils`).

### D2. Модель значения
Два режима по `multiple: Prop<bool>`: одиночный — `value: Option<RwSignal<Option<String>>>`, множественный — `values: Option<RwSignal<Vec<String>>>`. Без сигнала компонент создаёт внутренний с `default_value` / `default_values`. Один сигнал на оба режима не берём: `Option<String>` и `Vec<String>` читаются проще, чем «список из ≤1 элемента». Читается только сигнал активного режима. `on_change` получает `SelectButtonChange { value: SelectButtonValue, original_event }`, где `SelectButtonValue` — `Single(Option<String>)` или `Multiple(Vec<String>)`.

### D3. Выбор
Обработчик варианта (как `onOptionSelect`): ничего не делаем при `disabled` у компонента или варианта. Одиночный режим: выбранный вариант при `allow_empty == Some(false)` не меняется, иначе значение становится `None`; невыбранный — значением варианта. Множественный: выбранный удаляется из списка, невыбранный добавляется в конец; `allow_empty = false` не даёт удалить единственное значение (`getAllowEmpty`). После изменения вызываются `on_change`, затем `on_option_click { option, index, original_event }`. Устаревший `unselectable` не переносится.

### D4. Связь с ToggleButton
`ToggleButton` ведёт `checked` сам и переключает свой сигнал до вызова `on_change`; чтобы он не расходился со значением SelectButton (например, при `allow_empty = false` или недоступном варианте), на каждый вариант создаётся `RwSignal<bool>`, который `Effect` синхронизирует с «выбран ли вариант», а обработчик после своей работы принудительно выравнивает его (`set(is_selected)`), т. к. при неизменном значении `Effect` не сработает. Ограничения `allow_empty` и `disabled` передаются кнопке теми же значениями (`allow_empty` кнопки — `getAllowEmpty()`), поэтому блокировка срабатывает уже в ней. `ToggleButton` не меняется.

Альтернатива — добавить в `ToggleButton` режим «только чтение»: `checked` как `Prop<bool>` и состояние только снаружи. Отвергнута: меняет принятый API и спеку `togglebutton` ради одного потребителя.

### D5. Разметка
Корень — `<div>`; классы `p-selectbutton p-component`, `p-invalid`, `p-selectbutton-fluid`, затем `pt.root`. Атрибуты `role="group"`, `aria-labelledby`, `data-p` (`invalid`). Дочерние `ToggleButton`: `on_label = off_label = label`, `disabled = disabled || option.disabled`, `size`, `fluid`, `style_class` (только кнопкам), `pt = pt.toggle_button`, `content_template` из `item_template`. `invalid` кнопкам не передаётся (как в PrimeNG). Список строится замыканием над `options`, при смене перерисовывается целиком; ключ по метке (как `track getOptionLabel`) не нужен.

### D6. CSS
`@primeuix/styles/selectbutton` и пресет — `selectbutton` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`. Блок `/* For PrimeNG */` содержит только правило `.ng-invalid.ng-dirty` Angular Forms, поэтому `primeng_extra` не расширяется. Стили вложенных `p-togglebutton` берутся из CSS `togglebutton`, он подключается на страницах вместе с `selectbutton`.

### D7. Демонстрации
`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`. В `template` вместо `pi-align-*` (таких иконок в `prime-icons` нет) шаблон выводит `<strong>` с меткой; демонстрации с формами пропускаются. Дополнительно `echo` значения для e2e.

### D8. Тесты
SSR: структура, метки и порядок, выбранный вариант, `disabled`, классы и `data-p` корня, `size`/`fluid`/`style_class` у кнопок, `item_template`, `pt`, `aria_labelledby`. Браузерные (`tests/web.rs`): выбор, снятие, `allow_empty`, множественный выбор и порядок, `disabled` у варианта и компонента, клавиши Enter и Space, один `on_change` на нажатие, синхронность `aria-pressed` после отклонённого клика, реакция на внешний сигнал и смену `options`. Крейт с `tests/web.rs` добавляется в браузерный job `ci.yml` (3 места).

## Risks / Trade-offs

- [Лишние сигналы и `Effect` на вариант повышают число аллокаций относительно ToggleButton] → бюджет по измерению; при превышении — упростить синхронизацию (см. альтернативу в D4).
- [Расхождение сигнала кнопки и значения при редких путях обновления] → браузерный тест на отклонённый клик и внешнюю смену значения.
- [Строковые значения менее гибки, чем произвольный объект PrimeNG] → осознанное отличие, фиксируется в `docs/selectbutton.md`; обобщение до `T` возможно отдельным change.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
