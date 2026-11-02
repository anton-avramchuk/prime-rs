# Design

## Context

Требования — в `specs/listbox`. Исходники PrimeNG 21.1.10: `listbox/listbox.ts`, `listbox/style/listboxstyle.ts`; демонстрации `apps/showcase/doc/listbox/*-doc.ts`. Образцы в проекте: `prime-select` (`SelectOption`/`SelectGroup`, плоский список `Entry`, фокус по индексу, поиск набором, фильтр с `p-iconfield`), `prime-checkbox` (флажки), `prime-menu` (`flatten`, чистая логика клавиш, правило `*_untracked` в замыканиях списка).

Наблюдения по исходнику:
- Один шаблон: шапка (флажок «выбрать все» и/или поле поиска) → `div.p-listbox-list-container` → `ul[role=listbox]` с группами и вариантами или `li.p-listbox-empty-message`.
- Идентификатор варианта `<id>_<индекс>` по плоскому списку вместе с заголовками групп; `focusedOptionIndex` — индекс в этом списке (−1 — нет).
- Выбор: `onOptionSelectSingle`/`Multiple` с `metaKeySelection`; касание (`optionTouched`) отключает мета-режим. Диапазон — `onOptionSelectRange` между `startRangeIndex` и фокусом; `Shift` без стрелок обновляет опору (`onShiftKey`).
- Фокус на списке: фокусный — прежний, иначе первый доступный (`autoOptionFocus`) или выбранный; блюр сбрасывает фокус, опору и накопленную строку поиска.
- Пользователь Angular получает значение через `ngModel`; у нас — сигналы.
- `p-listbox-header > .p-iconfield { flex-grow: 1 }` и `.p-listbox-list-container { height: 100% }` — в блоке `/* For PrimeNG */`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роли и клавиши (включая диапазоны и `Ctrl+A`), одиночный и множественный выбор, группы, поиск, флажки и отметка.

**Non-Goals:** виртуальная прокрутка, ленивая загрузка, `dragdrop`, формы Angular, `optionLabel` и родственные, скрытые границы фокуса, `aria-live`-сообщения, шаблоны `header`/`footer`/`filter`/`empty`/`loader`, внешнее `selectAll`.

## Decisions

### D1. Крейт и модель
`prime-listbox` с `Listbox`, `ListboxOption { label, value, disabled }`, `ListboxGroup { label, items }`, `ListboxPt`. Типы совпадают по форме с `SelectOption`/`SelectGroup`, но свои: зависимость от `prime-select` ради двух структур тянула бы весь Select в бинарник. Значения — строки. `options`/`groups` — `Prop<Vec<_>>`.

### D2. Значение
`value: RwSignal<Option<String>>` (одиночный режим) и `values: RwSignal<Vec<String>>` (`multiple`); оба необязательны, без них — внутреннее состояние (`default_value`/`default_values`), как в Select. Значение, которого нет среди вариантов, остаётся в модели, но не подсвечивается. Один компонент, два режима — общий выбор через внутренний `Selection` (чистые функции ниже).

### D3. Чистая логика (`logic.rs`)
- `visible_entries(options, groups, filter) -> Vec<Entry>` — по образцу Select (подстрока, без регистра, группы без совпадений опускаются).
- Индексы: `first_valid`, `last_valid`, `next_valid`, `prev_valid`, `find_by_prefix`, `nearest_selected(…)`.
- Выбор: `select_single(current, value, meta, meta_key) -> Option<Option<String>>` (None — без изменений), `select_multiple(current, value, meta, meta_key) -> Vec<String>`, `select_range(entries, start, end) -> Vec<String>`, `select_all(entries) -> Vec<String>`, `all_selected(entries, selected) -> bool`.
- `root_classes`, `option_classes`, `data_p`.
Табличные тесты на каждую; компонент вызывает только эти функции, DOM-проверок логики не нужно.

### D4. Состояние и разметка
`focused_index: RwSignal<i32>`, `start_range: RwSignal<i32>`, `focused: RwSignal<bool>`, `filter: RwSignal<String>`, `search: StoredValue<(String, Option<TimeoutHandle>)>`. Записи списка — `Memo<Vec<Entry>>` от вариантов, групп и строки поиска; `For` по ключу «индекс+значение». В замыканиях, строящих список, сигналы фокуса и выбора читать только `*_untracked` (урок Menu/SelectButton); `p-focus`, `aria-*`, `data-p-*` и класс выбранного — реактивные атрибуты каждого варианта.

### D5. Флажки и поле поиска
Флажок варианта и «выбрать все» — разметка `Checkbox` (классы и `data-p` из `prime_checkbox::{CheckboxState, write_classes, data_p}`) с состоянием из замыканий: компонент `Checkbox` потребовал бы сигнала на каждый вариант и `Effect`, не работающий в SSR. Клик по полю флажка отменяется (`preventDefault`) и обрабатывается списком; `tabindex=-1`. Поле поиска — разметка вручную `span.p-iconfield` > `input.p-inputtext.p-component.p-listbox-filter` + `span.p-inputicon` со значком `search` (как в Select), без `prime-iconfield`/`prime-inputtext`: меньше зависимостей, класс `p-listbox-filter` задаётся самим полем.

### D6. Клавиши
`keydown` списка и поля поиска — таблица `match event.code` как в PrimeNG; `Shift`-диапазоны, `Ctrl+A`, поиск набором (500 мс, накапливаемая строка в `StoredValue`, таймер снимается в `on_cleanup` и проверяется `is_disposed`). `PageUp`/`PageDown` — прокрутка контейнера на высоту видимой области через `scroll_top` (в PrimeNG — `scrollInView(0)`, различие в docs). Прокрутка к фокусному варианту — `scrollIntoView({block: nearest})` после вставки/смены фокуса, только на wasm (`Effect::new` под `cfg(wasm32)`).

### D7. Мышь и касание
`mousedown` — фокусный вариант, `mouseenter` — при `focus_on_hover && focused`, `click` — выбор и `on_click`, `dblclick` — `on_dbl_click`. `touchend` на варианте отключает мета-режим до следующего выбора (флаг `StoredValue`).

### D8. Представления
`option: Option<Callback<(ListboxOption, bool), AnyView>>` (вариант, выбран ли), `group: Option<Callback<String, AnyView>>`.

### D9. CSS
`listbox` добавляется в `theme.components` и в `primeng_extra` (блок `/* For PrimeNG */`); `iconfield` уже подключён Select. Проверить, что `p-listbox-*` есть в пресете.

### D10. Тесты
SSR: разметка, группы, пустое сообщение, выбранные (одиночный/множественный), флажки, отметка, поле поиска, `data-p`, `style_class`, `pt`, `aria-*`. Браузерные `tests/web.rs` (один последовательный прогон — слушатели и таймеры общие): клик и сброс, мета-режим, множественный, диапазон `Shift`, `Ctrl+A`, «выбрать все» с поиском, фильтр, клавиши и `aria-activedescendant`, поиск набором, `readonly`/`disabled`, `on_*`, отписка. e2e: настоящая мышь и клавиши. Крейт добавляется в три места `ci.yml`.

### D11. Бенчмарки
Сценарии `listbox-1`/`-1000` (пять вариантов) и `case-listbox`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке/номеру строки).

## Risks / Trade-offs

- [Фокус] Перестройка списка роняет фокус → только `*_untracked` в замыканиях списка.
- [Два режима значения] Двойной набор свойств (`value`/`values`) усложняет API → один внутренний `Selection`, документируем в rustdoc и docs.
- [Таймеры поиска] Замыкания могут пережить компонент → `is_disposed()` и снятие в `on_cleanup`.
- [Размер wasm] Клавиши, диапазоны и флажки тяжелее Menu → бюджет подбирается по замеру.

## Open Questions

- Нужен ли `scroll_height` как `Prop<String>` или достаточно статической строки: берём `Prop<String>` (как `style`).

## Migration Plan

Миграции нет. Откат — `git revert`.
