# Design

## Context

Требования — в `specs/checkbox`. Исходники PrimeNG 21.1.10: `packages/primeng/src/checkbox/checkbox.ts`, `checkbox/style/checkboxstyle.ts`, демонстрации `apps/showcase/doc/checkbox`. Корень `<p-checkbox>` содержит скрытый `<input type="checkbox">` и `<div class="p-checkbox-box">` с иконкой. Модель значения и шаблон компонента — `docs/inputtext.md`, `docs/button.md`.

## Goals / Non-Goals

**Goals:** классы и `data-p` как в PrimeNG; два режима значения; статическая ветвь для случая без сигналов значения.
**Non-Goals:** формы (валидаторы, dirty/touched), произвольные `trueValue`/`falseValue`, `checkboxIcon` как CSS-класс.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-checkbox>` (как у остальных компонентов). Атрибуты `data-p*` ставятся на корень.

### D2. Два режима значения
- `binary=true`: `checked: Option<RwSignal<bool>>`.
- иначе: `value: String` и `model: Option<RwSignal<Vec<String>>>`; отмечен, если `model` содержит `value`; переключение добавляет или удаляет `value`.
Без сигнала состояние неконтролируемое: начальное значение из `default_checked`, дальше состояние живёт в DOM, классы корня не отслеживаются.
*Альтернатива*: обобщённый тип значения (`T: PartialEq`) — отвергнута ради простоты и размера wasm.

### D3. Вычисление состояния
`checked = !indeterminate && (binary ? checked_signal : model.contains(value))`. `indeterminate` — двусторонний `RwSignal<bool>` (с ним статическая ветвь не выбирается); при изменении пользователем он сбрасывается в `false`, `on_change` получает `CheckboxChange { checked, original_event }`; при `readonly` изменение игнорируется (нативное состояние `<input>` возвращается к сигналу).

### D4. Иконка
`check` при `checked`, `minus` при `indeterminate`; слот `icon_template` (`ViewFn`) заменяет иконку; иконки из `prime-icons`.

### D5. Ветви
Статическая — при `all_static!` и отсутствии сигналов значения; иначе реактивная. Разметка одна, в `macro_rules!`.

### D6. Тесты
SSR: структура, классы, `data-p*`, атрибуты, режимы, `pt`, паритет. Браузер: клик меняет сигнал, набор значений, `indeterminate`, `readonly`, `disabled`, события.

## Risks / Trade-offs

- [`<input>` скрыт, клик идёт по нему поверх бокса] → CSS темы `p-checkbox-input` накрывает бокс; проверяется e2e.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
