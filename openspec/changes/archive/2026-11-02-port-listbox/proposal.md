# Proposal

## Why

Listbox — десятый компонент волны 3: список вариантов, из которого выбирают одно или несколько значений, с поиском, группами и флажками. Он нужен как основа для MultiSelect (общий выбор из списка, `Ctrl`/`Shift`, «выбрать все») и закрывает типовые формы «список справа от формы» без панели-оверлея. Разметка и клавиши во многом повторяют список внутри Select, но выбор нескольких значений и диапазонов — новая часть.

## What Changes

- Новый крейт `prime-listbox` (порт `p-listbox` из PrimeNG 21.1.10).
- `ListboxOption` (`label`, `value`, `disabled`) и `ListboxGroup` (`label`, `items`); значения — строки, как у Select и SelectButton.
- `Listbox`: корень `div.p-listbox.p-component` > необязательная шапка `div.p-listbox-header` (флажок «выбрать все» при `checkbox` и `multiple`, поле поиска `p-listbox-filter` в `p-iconfield`) > `div.p-listbox-list-container` (`scroll_height`, по умолчанию `14rem`) > `ul.p-listbox-list[role=listbox]` с `li.p-listbox-option[role=option]`, заголовками групп `li.p-listbox-option-group` и сообщением пустого списка `div.p-listbox-empty-message`.
- Свойства: `options`, `groups`, `value` (одно значение — `RwSignal<Option<String>>`) и `values` (несколько — `RwSignal<Vec<String>>`) при `multiple`, `multiple`, `checkbox`, `filter`, `checkmark`, `striped`, `highlight_on_select` (да), `meta_key_selection`, `select_on_focus`, `auto_option_focus` (да), `focus_on_hover` (да), `show_toggle_all` (да), `readonly`, `disabled`, `invalid`, `fluid` (`OptProp<bool>`), `scroll_height`, `tabindex` (0), `style`, `style_class`, `list_style`, `list_style_class`, `filter_placeholder`, `empty_message`, `empty_filter_message`, `aria_label`, `aria_filter_label`, `id`, `pt`; представления `option` (вместо содержимого варианта) и `group`; события `on_change`, `on_click`, `on_dbl_click`, `on_filter`, `on_focus`, `on_blur`, `on_select_all_change`.
- Поведение: клик выбирает/снимает вариант (одиночный и множественный режимы, `Ctrl`/`⌘` при `meta_key_selection`); клавиши ↓ ↑ Home End PageUp PageDown Enter Space, `Shift`+стрелки и `Ctrl+Shift+Home/End` выбирают диапазон, `Ctrl+A` — все, печатные символы — поиск набором; фокус (`p-focus`, `aria-activedescendant`); двойной клик; «выбрать все» с учётом фильтра; поиск — подстрока метки без учёта регистра.
- Страница showcase `/listbox` с демонстрациями `basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`; e2e; бенчмарки и бюджеты; `docs/listbox.md`.
- Стили: `listbox` добавляется в `theme.components`; блок `/* For PrimeNG */` (шапка, поле поиска, высота контейнера) — через `primeng_extra`.
- **Не входит:** виртуальная прокрутка и ленивая загрузка (`virtualScroll*`, `lazy`, `onLazyLoad`), `dragdrop` и `dropListData` (CDK), интеграция с формами Angular, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroup*`/`dataKey`/`filterBy`/`filterFields`/`filterMatchMode`/`filterLocale`/`searchLocale` (заменены типизированными вариантами), скрытые границы фокуса `p-hidden-focusable`, шаблоны `header`, `footer`, `filter`, `empty`, `loader`, скрытые `aria-live`-сообщения (`searchMessage`, `selectionMessage` и т. п.), управляемое снаружи «выбрать все» (`selectAll`), `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `listbox`: список вариантов с одиночным и множественным выбором, группами, поиском, флажками и навигацией клавишами.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-listbox` (`publish = false`); зависимости: `prime-core`, `prime-icons`, `prime-checkbox` (только `CheckboxState`, классы и `data-p` флажка), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
