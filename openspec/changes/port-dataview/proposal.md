# Proposal

## Why

DataView — данные в виде списка или сетки с пагинатором, сортировкой и фильтром. Первый компонент волны 7 (DataView → Table → TreeTable): небольшой и опирается на готовый Paginator.

## What Changes

- Новый крейт `prime-dataview` (порт `p-dataview` из PrimeNG 21.1.10): обобщённый `DataView<T>`.
- Чистый модуль `data.rs`: `prepare` (фильтр и устойчивая сортировка по порядку 1/-1/0), `page`, `total`, `clamp_first`; табличные тесты.
- Разметка `div.p-dataview.p-component.p-dataview-list|grid` > `p-dataview-loading` (маска и значок), `p-dataview-header`, пагинатор `p-dataview-paginator-top|bottom`, `p-dataview-content` (шаблоны `list`/`grid` получают элементы страницы), `p-dataview-empty-message`, `p-dataview-footer`.
- Свойства: `value`, `list`, `grid`, `layout`, `paginator`, `first`, `rows`, `total_records`, настройки пагинатора, `paginator_position`, `lazy`, `lazy_load_on_init`, `filter` (предикат), `compare` и `sort_order`, `empty_message`, `loading`, `header`, `footer`, `empty`, `style`, `style_class`; события `on_page`, `on_lazy_load`.
- Страница `/dataview` (`basic`, `layout`, `pagination`, `sorting`, `loading`), e2e, бенчмарки и бюджеты, `docs/dataview.md`, `dataview` в `theme.components`.
- **Не входит:** `sortField`/`filterBy` по имени поля (замена — `compare` и `filter`), `trackBy`, `loadingIcon`, `onChangeLayout`/`onSort`, шаблоны Angular.

## Capabilities

### New Capabilities
- `dataview`: данные в виде списка или сетки с пагинатором, сортировкой и фильтром.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-dataview` (`publish = false`); зависимости: `prime-core`, `prime-paginator`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
