# Design

## Context

Требования — в `specs/stepper`. Исходники PrimeNG 21.1.10: `stepper/stepper.ts` (все части в одном файле), `stepper/style/*.ts`; стили `@primeuix/styles/stepper`; демонстрации `apps/showcase/doc/stepper/*-doc.ts`. Образцы в проекте: `prime-tabs` (составной компонент: `Tabs` + контекст, значения-строки, идентификаторы `<id>_tab_<value>`), `prime-accordion` (вертикальное раскрытие через `prime_core::collapsible::bind_region_with`, `NodeRef` только на клиенте).

Наблюдения по исходнику:
- `Stepper.value` — число (модель); `Step.value`/`StepPanel.value` — числа; внутри `StepItem` значение раздаётся вложенным `Step` и `StepPanel` эффектами.
- `Step.isStepDisabled = !active && (stepper.linear || step.disabled)`; клик → `stepper.updateValue(value)`; `activateCallback` шаблона `content` — `onStepClick` (для шага) и `updateValue(index)` (для панели).
- Разделитель после шага — если шаг не последний в `StepList.steps()`; разделитель панели в вертикальной раскладке — если элемент не последний среди `[data-pc-name=steppanel]` (число `StepItem`).
- `StepPanel` оборачивает содержимое в `p-motion` (`visible = active`, анимация только в вертикальном режиме): у неактивной горизонтальной панели содержимого нет вообще.
- Класс корня `p-readonly` при `linear` (в CSS темы стоит `p-stepper-readonly`; следуем классу PrimeNG).
- Клавиатурной навигации между шагами нет: шаги — обычные кнопки.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роли и `aria-*`; значение — двусторонний сигнал; свободный и линейный режимы; вертикальная раскладка с анимацией; `content` как аналог шаблонов.

**Non-Goals:** `transitionOptions`/`motionOptions`/`@primeuix/motion`, `Bind`, `pTemplate`, клавиатурная навигация сверх нативных кнопок.

## Decisions

### D1. Крейт и контекст
`prime-stepper` с `Stepper`, `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem`, `StepContext`, `StepperPt`. `Stepper` кладёт в контекст `StepperCtx { id, value: RwSignal<Option<u32>>, linear: Prop<bool>, steps: RwSignal<Vec<u32>>, items: RwSignal<Vec<u32>> }`; `StepItem` кладёт `ItemCtx { value }`. Значение шага/панели: собственное `value` или значение ближайшего `StepItem`; без обоих — паника с понятным сообщением (как `NO_TABS` в Tabs).

### D2. Чистая логика (`state.rs`)
`step_id(id, value)`, `panel_id(id, value)`, `is_active(value, current)`, `step_disabled(active, linear, disabled)`, `is_last(values, value)`, `root_classes(linear, extra)`, `step_classes(active, disabled)`, `item_classes(active)`, `panel_classes(vertical, active)`. Табличные тесты.

### D3. Реестр шагов для разделителя
`Step` при создании записывает своё значение в `steps` (порядок = порядок создания = порядок в DOM), `on_cleanup` убирает. Разделитель шага — реактивное выражение `!is_last(steps, value)`: компоненты вызываются при построении дерева до сериализации и монтирования, поэтому к моменту вывода реестр заполнен; динамически добавленные шаги пересчитывают разделители. Аналогично `StepItem` пишет значение в `items` для разделителя панели.

### D4. Вертикальная раскладка
Панель знает, что она вертикальная, по наличию `ItemCtx` в контексте. Вертикальная панель: корневой `div.p-steppanel` с `p-steppanel-active`; внутри `div.p-motion` с `bind_region_with(region, move || !active)` (только `cfg(wasm32)`, как у Accordion), в нём `div.p-steppanel-content-wrapper` > разделитель + `div.p-steppanel-content`; начальный стиль неактивной — `display: none` (`hidden_style`). Горизонтальная панель: `Show`-подобный вывод содержимого только при `active`.

### D5. `content`
`StepContext { value: u32, active: Signal<bool>, activate: Callback<u32> }` (`Copy`/`Clone`). `Step.content: Option<Callback<StepContext, AnyView>>` заменяет кнопку; `StepPanel.content` — содержимое `p-steppanel-content`. Дочерние элементы (`children`) — обычное содержимое; если заданы оба, побеждает `content`.

### D6. Идентификаторы
`Stepper.id` — свойство или `pn_id_<N>` через `prime_core::ids::next_unique`. Шаг: `<id>_step_<value>`, панель: `<id>_steppanel_<value>`.

### D7. CSS
`stepper` добавляется в `theme.components`. Проверить, есть ли в пресете `p-steppanel`, `p-stepitem`, `p-stepper-separator`; отдельный блок `/* For PrimeNG */` у `stepperstyle.ts` — `.p-steppanel .p-motion { display: grid; grid-template-rows: 1fr }`, поэтому `stepper` добавляется и в `primeng_extra` (`xtask gen-theme` читает блок); проверить, что правило попало в CSS.

### D8. Тесты
SSR: корень, `p-readonly`, шаги и их состояния, разделители (для двух и трёх шагов, вертикальный), панели (горизонтальная: контент только у активной; вертикальная: все в разметке), `StepItem`, `content`, идентификаторы и `aria-controls`, `pt`. Браузерные `tests/web.rs`: клик по шагу, линейный режим, `activate` из панели, внешний `value`, динамическое добавление шага (разделители), вертикальный режим (скрытие и показ), отключённый шаг. e2e: настоящая мышь и Tab/Enter по шагам, скриншотные сравнения. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `stepper-1`/`-1000` (три шага, три панели, активный первый) и `case-stepper`; сначала `cargo bench -p render-bench --bench alloc`; для wasm — отдельный файл бюджетов `--budgets`.

## Risks / Trade-offs

- [Реестр и порядок] Разделитель зависит от порядка создания компонентов; динамическая вставка шага в середину нарушит порядок реестра → вставка в середину не поддерживается (как порядок `contentChildren` в Angular зависит от DOM); фиксируем в docs.
- [Анимация вертикальной панели] Высота анимируется через `grid-template-rows`; корректность проверяется в браузере (e2e) и визуально, как у Accordion.
- [Снимок `active` в `content`] В PrimeNG `active` — значение момента; у нас сигнал.

## Open Questions

- Нужен ли `StepperHandle` (программное управление без своего сигнала): нет, достаточно `value` и `StepContext::activate`.

## Migration Plan

Миграции нет. Откат — `git revert`.
