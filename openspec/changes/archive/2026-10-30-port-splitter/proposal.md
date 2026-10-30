# Proposal

## Why

Splitter — второй компонент волны 3: набор панелей, размеры которых меняются перетаскиванием разделителей или стрелками. Он не использует оверлеев, опирается на подписки на `window` (по образцу Slider) и вводит в проект сохранение состояния в `localStorage`/`sessionStorage`.

## What Changes

- Новый крейт `prime-splitter` с компонентом `Splitter` (порт `p-splitter` из PrimeNG 21.1.10): корень `div.p-splitter.p-component.p-splitter-<layout>` (`display: flex`, колонкой при `vertical`), панели `div.p-splitterpanel`, между панелями разделитель `div.p-splitter-gutter[role=separator]` с ручкой `div.p-splitter-gutter-handle[tabindex=0][aria-orientation][aria-valuenow]`.
- Свойства: `panels: Vec<ViewFn>` (содержимое панелей по порядку, вместо `ng-template #panel`), `layout` (`Horizontal` по умолчанию | `Vertical`), `panel_sizes: Vec<f64>` (проценты; недостающие и нулевые — поровну), `min_sizes: Vec<f64>`, `gutter_size` (4), `step` (5, пикселей), `state_key` и `state_storage` (`Session` по умолчанию | `Local`), `nested`, `style`, `style_class`, `panel_style`, `panel_style_class`, `pt`; `on_resize_start`, `on_resize_end` с событием `SplitterResizeEvent { sizes }`.
- Размер панели: `flex-basis: calc(<доля>% - <(n-1)*gutter_size>px)`. Перетаскивание мышью (и касанием) и стрелки на ручке (по оси `layout`) меняют две соседние панели; `min_sizes` ограничивают изменение; состояние сохраняется по `state_key` и восстанавливается при монтировании.
- Страница showcase `/splitter` с демонстрациями `horizontal`, `vertical`, `size`, `nested`; e2e с настоящим перетаскиванием; бенчмарки и бюджеты; `docs/splitter.md`.
- Стили: `splitter` добавляется в `theme.components`.
- **Не входит:** шаблоны через `ng-template` и `ContentChildren` (вместо них `panels`), автоопределение вложенности (вместо него `nested`), `originalEvent` в событиях, `Bind`/`pt` с контекстом, повтор клавиши по таймеру (`setTimer` 40 мс): шаг применяется на каждый `keydown` (в том числе автоповтор клавиши), завершение — на `keyup`.

## Capabilities

### New Capabilities
- `splitter`: панели с изменяемыми размерами и разделителями.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-splitter` (`publish = false`); зависимости: `prime-core`, `web-sys` (`Window`, `Storage`, `MouseEvent`, `TouchEvent`, `KeyboardEvent`, `Element`, `DomRect`).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
