# Design

## Context

Требования — в `specs/steps`. Исходники PrimeNG 21.1.10: `steps/steps.ts` (около 300 строк), `steps/style/stepsstyle.ts`; стили `@primeuix/styles/steps`. Демонстраций в локальной копии нет (каталога `apps/showcase/doc/steps` в этом коммите нет, страница `pages/steps/index.ts` на них ссылается), поэтому демонстрации составлены по смыслу списка (`basic`, `controlled`, `interactive`) и по коду компонента. Образцы в проекте: `prime-stepper` (тот же CSS-стиль и пара «шаги — панели»), `prime-dock` (собственная модель пунктов, `Callback` команды), `prime-tabs` (roving `tabindex`).

Наблюдения по исходнику:
- Активность пункта: без `routerLink` — `index === activeIndex`; с `routerLink` — совпадение с адресом маршрутизатора (не переносится).
- Класс пункта: `p-steps-item`, `p-steps-item-active` (активен), `p-disabled` (`item.disabled || (readonly && !active)`). `aria-current="step"` — у активного.
- Ссылка: `tabindex` — `-1` у отключённого, у активного `item.tabindex || '0'`, у остальных `item.tabindex ?? '-1'`; `aria-expanded = (i === activeIndex)`; `aria-disabled = item.disabled || (readonly && i !== activeIndex)`.
- Клик: при `readonly` или `disabled` — `preventDefault`, выход; иначе `activeIndexChange.emit(i)`, `preventDefault` без `url`/`routerLink`, затем `command({ originalEvent, item, index })`. Значение `activeIndex` внутри компонента НЕ меняется (его ведёт владелец) — двусторонняя привязка.
- Клавиши: ←/→ — соседний пункт, `Home`/`End` — крайние (фокус переносится вызовом `focus()` и roving `tabindex`), `Tab` — возвращает `tabindex="0"` активному пункту, `Enter`/`Space` — клик; `preventDefault` у обработанных.
- Ошибка исходника: `track item.label` в `@for` — пункты с одинаковой подписью склеиваются; в порте ключ — индекс.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, поведение клика и клавиш.

**Non-Goals:** маршрутизатор, подсказки пунктов, HTML-метки, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и модель
`prime-steps` с `Steps`, `StepsItem` (строитель), `StepsCommand { event, item, index }`, `StepsPt`. Свой тип пункта, а не `MenuItem`, потому что `command` получает индекс и нужен `tabindex`.

### D2. Чистая логика (`state.rs`)
`root_classes(readonly, extra)`, `item_classes(active, disabled, extra)`, `is_item_disabled(item, index, active, readonly)`, `link_tabindex(item, index, active)`, `aria_disabled(item, index, active, readonly)`, `click_outcome(item, readonly) -> Click { Ignore, Activate { prevent_default } }`, `key_target(code, index, count) -> Option<usize>` (в крайних пунктах ←/→ остаются на месте), `is_activation_key(code)`. Табличные тесты.

### D3. Состояние
`active_index: Prop<usize>` для чтения и необязательный `RwSignal<usize>` (`active_index_signal`) для двусторонней связи: если он задан, клик устанавливает значение; событие `on_active_index_change` вызывается всегда. Roving `tabindex` — `focus_index: RwSignal<Option<usize>>`: по ←/→/Home/End обновляется и фокусируется ссылка; сброс на активный пункт при `Tab` и при смене `active_index`.

### D4. Разметка
Один `li` на пункт, индекс как ключ; состояние пункта — реактивные атрибуты. Ссылка `a` с `href` из `url` (в `readonly` — тоже, как в PrimeNG).

### D5. Тесты
SSR: разметка, классы по `readonly`/`disabled`/активному, `aria-*`, `tabindex`, `href`/`target`, `pt`. Браузерные `tests/web.rs`: клик в `readonly` (ничего), клик в интерактивном режиме (событие, команда с индексом, сигнал), отключённый пункт, ссылка с `url` не отменяется, клавиши и roving `tabindex`, `Enter`/`Space`, `Tab`. e2e: стили темы (номера в кружках, линия между шагами), клик/клавиши в интерактивной демонстрации.

### D6. Демонстрации и бенчмарки
`basic` (только чтение), `controlled` (кнопки «Назад/Далее» меняют `active_index`), `interactive` (клик по пунктам, `Toast` по `command`). Бенч `steps-1`/`-1000` и `case-steps`.

## Risks / Trade-offs

- [Нет эталонных демонстраций] Демонстрации выведены по коду и списку; результат проверяем по классам и CSS темы, а не по эталонным скриншотам.
- [Два способа вести `active_index`] Значение и сигнал; документируем в `docs/steps.md`.

## Open Questions

- Нужен ли `exact`: в PrimeNG он влияет лишь на `ariaCurrentWhenActive` ссылок маршрутизатора — сохраняем свойство для совместимости, поведения у него нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
