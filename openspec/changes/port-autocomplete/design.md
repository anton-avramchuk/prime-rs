# Design

## Context

Требования — в `specs/autocomplete`. Исходники PrimeNG 21.1.10: `autocomplete/autocomplete.ts`, `style/autocompletestyle.ts`; демонстрации `apps/showcase/doc/autocomplete/*-doc.ts`. Образцы — `prime-select`/`prime-treeselect`/`prime-cascadeselect` (панель внутри корня, окно-слушатели, `align_overlay`), `prime-inputtext` (классы поля), `prime-multiselect` (чипы).

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, клавиши, запрос и выбор.

**Non-Goals:** объекты как варианты, виртуальная прокрутка, формы Angular, `appendTo`, шаблоны `header`/`footer`/иконок.

## Decisions

### D1. Модель
`AutoCompleteOption { label, value: Option<String>, disabled, items }`. Плоский список для навигации: `flatten(options, group)` даёт строки `Row::Group(label)` / `Row::Option { index_in_flat, label, value, disabled }`; чистые функции `first/last/next/prev` по допустимым строкам (не отключённым и не заголовкам), `selected_index`, `matches(label, text)` (`force_selection`), `input_text(value, options)` (подпись значения), `split_separator` — табличные тесты.

### D2. Состояние
`Core`: `suggestions: Prop<Vec<AutoCompleteOption>>`, `value: RwSignal<String>`, `values: RwSignal<Vec<String>>`, `open`, `focused`, `loading`, `focus_index: RwSignal<Option<usize>>`, `focused_chip: RwSignal<Option<usize>>`, `dirty`, `search_timer` (wasm), ссылки на корень, поле, панель; эффекты открытия (z-index, `align_overlay`, слушатели клика вне и `resize`) — под `cfg(wasm32)`. Эффект на `suggestions` включает панель, когда идёт запрос.

### D3. Ввод
Текст поля — собственный `RwSignal<String>` `text` (одиночный режим синхронизируется со значением: значение → подпись, выбор → подпись). `on:input` обновляет `text`, при `typeahead` и длине ≥ `min_length` ставит таймер `search`. Значением становится `text` при `!force_selection`.

### D4. Чипы и клавиатура
Контейнер `ul` получает фокус при ← на пустом поле; `focused_chip` — индекс; удаление — `removeOption`.

### D5. Тесты
SSR: одиночный/множественный режим, классы, очистка, кнопка, спиннер, `pt`; браузерные `tests/web.rs` одним последовательным тестом: ввод и запрос (`delay`), выбор кликом и клавишей, `multiple`, чипы, кнопка-стрелка, очистка, `force_selection`, клик вне, группы; после клавиши `sleep`.

## Risks / Trade-offs

- [Две «значения» (текст и значение варианта)] Для строковых вариантов они совпадают; различие — только при `value` у варианта.
- [Таймер запроса] Сбрасывается при размонтировании (`StoredValue`), проверка `is_disposed`.

## Open Questions

- Нужно ли открывать панель повторно, если `suggestions` пришли без запроса (внешнее обновление) — как в PrimeNG, только пока `loading`.

## Migration Plan

Миграции нет. Откат — `git revert`.
