# Design

## Context

Требования — в `specs/treetable`. Исходники PrimeNG 21.1.10: `treetable/treetable.ts` (`serializeNodes`, `serializePageNodes`, `sortNodes`, `toggleNodeWithCheckbox`, `propagateSelectionUp/Down`, `TreeTableToggler`, `TTCheckbox`), `style/treetablestyle.ts` (весь CSS — в блоке `For PrimeNG`; модуль `@primeuix/styles/treetable` даёт токены). Основа — Table (`prime_table::Column`, `prime_table::model`).

## Decisions

### D1. Узлы и ключи
`TreeNode<T>` с ключом; выбор и раскрытие — векторы ключей (как у Table). Начальное раскрытие — из флагов `expanded`, если `expanded_keys` не передан.

### D2. Чистая модель (`tree.rs`)
`sort_nodes` рекурсивно применяет `prime_table::model::sort_rows` к соседям; `page_roots` + `flatten` дают видимые строки; `checkbox_toggle` и `check_state` реализуют распространение выбора вниз/вверх и частичное состояние. Табличные тесты.

### D3. Компонент
Столбцы — `prime_table::Column<T>`; в столбце `toggler_column` перед содержимым выводятся кнопка раскрытия и (в режиме `checkbox`) флажок узла — `Checkbox` с `checked` и `indeterminate`, которые согласуются с `selection` эффектом на каждую строку под `cfg(wasm32)`. Заголовки, строки и подвал строятся в реактивных замыканиях; классы выбора и значки сортировки читают сигналы внутри ячеек.

### D4. Тема
`treetable` в `theme.components` и `primeng_extra`.

### D5. Тесты
Модель — таблицы. SSR: свёрнутое дерево, кнопки и отступы, сортировка, пагинатор по корням, выбор, флажки, пусто/загрузка/подвал. Браузерные: раскрытие/сворачивание, сортировка по уровням, выбор кликом и Ctrl, флажки с распространением, пагинатор. e2e: стили и клики.

## Risks / Trade-offs

- [Объём] Этап 1 — основа; фильтры, правка и ленивая загрузка добавляются новыми свойствами без смены модели.
- [Размер] Около 1,7 МБ wasm из-за Paginator, Checkbox, Badge и общей части Table.

## Migration Plan

Миграции нет. Откат — `git revert`.
