# Proposal

## Why

ContextMenu — первое из остальных меню волны 5 (после `Steps`). Оно показывает то же дерево пунктов, что и `TieredMenu`, но открывается правой кнопкой мыши (или долгим касанием) у курсора. Вся логика путей, клавиатуры и поиска уже есть в `prime-tieredmenu`; новое — привязка к цели/документу, положение по координатам курсора и отсутствие `popup`-режима.

## What Changes

- Новый крейт `prime-contextmenu` (порт `p-contextmenu` из PrimeNG 21.1.10). Модель пунктов — `prime_menu::MenuItem`; чистая логика (`model`) берётся из `prime-tieredmenu`.
- `ContextMenu`: корень `div.p-contextmenu.p-component` (`p-contextmenu-mobile` на узком окне; `id`, `style`, `style_class`) > `ul.p-contextmenu-root-list[role=menu]` > `li.p-contextmenu-item` > `div.p-contextmenu-item-content` > `a.p-contextmenu-item-link` (значок `p-contextmenu-item-icon`, подпись `p-contextmenu-item-label`, стрелка `p-contextmenu-submenu-icon`) + вложенный `ul.p-contextmenu-submenu`; разделители `li.p-contextmenu-separator`. Атрибуты `aria-*`, `data-p-*` и клавиатура — как у `TieredMenu`.
- Открытие: событие `contextmenu` (свойство `trigger_event`) на `target` (CSS-селектор) или на всём документе (`global`); на сенсорных устройствах — долгое касание (`press_delay`, 500 мс). Меню показывается у курсора (`pageX + 1`, `pageY + 1`) с переворотом у правого/нижнего края окна и прижатием к левому/верхнему; событие отменяется (`preventDefault`, `stopPropagation`).
- `ContextMenuRef` (`show(event)`, `hide`, `toggle(event)`, `visible`); `on_show`, `on_hide`.
- Закрытие: клик вне меню (кроме правой кнопки и Ctrl-клика), `resize`, выбор листа, `Escape`, `Tab`. Слой группы `Menu` (`auto_z_index`, `base_z_index`), фокус корневого списка после показа.
- Подменю раскрываются вправо, а при нехватке места — влево (как у `TieredMenu`).
- Свойства: `model`, `target`, `global`, `trigger_event`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `id`, `breakpoint` (`960px`), `aria_label`, `aria_labelledby`, `press_delay`, `pt`; представления `item`, `submenu_icon`; события `on_show`, `on_hide`.
- Страница showcase `/contextmenu` с демонстрациями `basic`, `command`, `template` (интеграция с `Table` и `router` не переносятся); e2e; бенчмарки и бюджеты; `docs/contextmenu.md`.
- Стили: `contextmenu` добавляется в `theme.components`.
- **Не входит:** интеграция с `Table`, `appendTo`, анимационные опции (`motionOptions`), `routerLink`, `tooltip` и HTML-метки пунктов, `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `contextmenu`: контекстное меню с вложенными подменю, открываемое у курсора.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-contextmenu` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-menu`, `prime-tieredmenu`, `prime-icons` (angleright), `web-sys`.
- Возможно расширяется публичный экспорт `prime-tieredmenu` (`pub use model::…`) — без изменения поведения.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
