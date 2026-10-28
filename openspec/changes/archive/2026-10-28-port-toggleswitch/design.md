# Design

## Context

Требования — в `specs/toggleswitch`. Исходники PrimeNG 21.1.10: `packages/primeng/src/toggleswitch/toggleswitch.ts`, `toggleswitch/style/toggleswitchstyle.ts`, демонстрации `apps/showcase/doc/toggleswitch`. В PrimeNG клик обрабатывается на хосте (`@HostListener('click')`): переключается `modelValue`, эмитится `onChange`, фокус возвращается на `<input>`. Шаблон компонента и модель значения — `docs/checkbox.md`.

## Goals / Non-Goals

**Goals:** классы, `data-p*` и inline-стиль корня как в PrimeNG; клик по любой части корня; слот handle; статическая ветвь без сигнала.
**Non-Goals:** формы, `trueValue`/`falseValue`, `autofocus`, `size`.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-toggleswitch>`; `style="position: relative"` — как `inlineStyles.root` PrimeNG, к нему присоединяются `style` и `pt.root`.

### D2. Состояние
`checked: Option<RwSignal<bool>>`; без него — `default_checked`, а дальше состояние живёт в DOM, классы корня не отслеживаются.

### D3. Клик
Обработчик `on:click` на корне: при `!disabled && !readonly` переключает сигнал, вызывает `on_change(ToggleSwitchChange { checked, original_event })`, возвращает фокус на `<input>`. Нативное состояние `<input>` после обработки выравнивается по итоговому значению (при `readonly`/`disabled` возвращается к прежнему). Отдельный `change` на `<input>` не обрабатывается, чтобы не вызвать событие дважды.

### D4. Слот
`handle_template: Option<Callback<bool, AnyView>>` внутри `div.p-toggleswitch-handle`, получает `checked`.

### D5. Ветви
Статическая — при `all_static!` и `checked.is_none()`; иначе реактивная. Разметка и помощник `rx(props, pt, fn)` — как у Checkbox.

### D6. Тесты
SSR: структура, стиль корня, классы, `data-p*`, `aria-checked`, `pt`, слот, паритет. Браузер: клик по корню и по `<input>` переключает, `readonly`/`disabled`, события.

## Risks / Trade-offs

- [Клик по `<input>` всплывает на корень и обрабатывается один раз] → проверяется браузерным тестом.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
