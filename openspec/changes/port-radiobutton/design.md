# Design

## Context

Требования — в `specs/radiobutton`. Исходники PrimeNG 21.1.10: `packages/primeng/src/radiobutton/radiobutton.ts`, `radiobutton/style/radiobuttonstyle.ts`, демонстрации `apps/showcase/doc/radiobutton`. Шаблон компонента и модель значения — `docs/checkbox.md`, `docs/inputtext.md`.

## Goals / Non-Goals

**Goals:** классы и `data-p*` как в PrimeNG; выбор одного значения группы через общий сигнал; статическая ветвь для случая без сигнала.
**Non-Goals:** формы, `RadioControlRegistry`, `binary`, `autofocus`.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-radiobutton>`; `data-p*` — на корне.

### D2. Группа через общий сигнал
`model: Option<RwSignal<Option<String>>>` и `value: OptProp<String>`: отмечен, если `model == Some(value)`. Выбор записывает `Some(value)` в `model`; остальные кнопки группы обновляются реактивно, реестр не нужен. Без `model` — неконтролируемая кнопка с `default_checked` (нативная группировка по `name`).

### D3. Атрибут `value`
В PrimeNG `[attr.value]="modelValue()"` выводит булево состояние. Мы выводим собственное `value` (иначе нативная форма теряет смысл); отличие фиксируется в `docs/radiobutton.md`.

### D4. События
`on_click` получает `RadioButtonClick { value, original_event }` после записи в `model`; при `disabled` выбор игнорируется (состояние возвращается к `model`).

### D5. Ветви
Статическая — при `all_static!` и `model.is_none()`; иначе реактивная. Разметка и помощник `rx(props, pt, fn)` — как у Checkbox.

### D6. Тесты
SSR: структура, классы, `data-p*`, атрибуты, группа, `pt`, паритет. Браузер: клик выбирает значение, группа снимает остальные, `disabled`, события.

## Risks / Trade-offs

- [Две кнопки с одним `name` без `model` ведут себя нативно] → документируется.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
