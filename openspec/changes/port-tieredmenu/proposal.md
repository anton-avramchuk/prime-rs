# Proposal

## Why

TieredMenu — последний компонент волны 4: меню с вложенными подменю, раскрывающимися в соседних панелях (как в контекстном меню). Он закрывает отложенное в `SplitButton` (демонстрация `nested`) и открывает дорогу остальным меню (ContextMenu, Menubar, MegaMenu, PanelMenu), которые строятся на тех же идеях: дерево обработанных пунктов, путь активных пунктов, клавиатура по уровням.

## What Changes

- Новый крейт `prime-tieredmenu` (порт `p-tieredmenu` из PrimeNG 21.1.10). Модель пунктов — готовые `prime_menu::MenuItem` и `MenuCommand` (поле `items` у `MenuItem` задаёт вложенное подменю).
- `TieredMenu`: корень `div.p-tieredmenu.p-component` (`p-tieredmenu-overlay` в режиме `popup`, `p-tieredmenu-mobile` на узком окне; `id`, `style`, `style_class`) > `ul.p-tieredmenu-root-list[role=menu][aria-orientation=vertical]` (`tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`) > `li.p-tieredmenu-item[role=menuitem]` (`p-tieredmenu-item-active`, `p-focus`, `p-disabled`, `data-p-highlight|focused|disabled`, `aria-label`, `aria-disabled`, `aria-haspopup=menu`, `aria-expanded`, `aria-setsize`, `aria-posinset`) > `div.p-tieredmenu-item-content` > `a.p-tieredmenu-item-link` со значком `p-tieredmenu-item-icon`, подписью `p-tieredmenu-item-label` и стрелкой `p-tieredmenu-submenu-icon` (у групп) + вложенный `ul.p-tieredmenu-submenu[role=menu]` (показан у активного пункта); разделители `li.p-tieredmenu-separator[role=separator]`.
- Поведение: клик по группе раскрывает её, повторный — сворачивает; клик по листу вызывает `command`, закрывает все подменю (в `popup` — и меню) и возвращает фокус в корневой список; наведение раскрывает группы после первого раскрытия кликом (`auto_display`, по умолчанию да; на узком окне — не раскрывает); клавиатура: ↓/↑ по пунктам уровня, → раскрывает группу и уходит внутрь, ← закрывает подменю и возвращается к родителю, `Home`/`End`, `Enter`/`Space` активируют, `Escape` закрывает, `Tab` закрывает, `Alt+↑` закрывает всплывающее меню, печатаемые символы ищут пункт по началу подписи (сброс через 500 мс); `aria-activedescendant` указывает на пункт под фокусом.
- Режим `popup`: `TieredMenuRef` (`show`/`show_at`/`toggle`/`hide`/`visible`) — всплывающая панель у цели; слой группы `Menu`; выравнивание по цели; клик вне, `resize` (не на сенсорных) и прокрутка предка закрывают; `on_show`, `on_hide`.
- Положение подменю по `nestedPosition`: справа от пункта, а если справа нет места — слева.
- Свойства: `model`, `popup`, `style`, `style_class`, `breakpoint` (`960px`), `auto_z_index`, `base_z_index`, `auto_display`, `id`, `aria_label`, `aria_labelledby`, `disabled`, `tabindex`, `pt`; представления `item`, `submenu_icon`; события `on_show`, `on_hide`.
- Страница showcase `/tieredmenu` с демонстрациями `basic`, `popup`, `command`, `template`; демонстрация `nested` у `SplitButton` не меняется (см. «Не входит»); e2e; бенчмарки и бюджеты; `docs/tieredmenu.md`.
- Стили: `tieredmenu` добавляется в `theme.components`.
- **Не входит:** подключение `TieredMenu` внутрь `SplitButton` (отдельное изменение: меняется API и тесты `SplitButton`), `appendTo` (панель в собственном месте, как `Menu`), `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (анимация — классы темы), `routerLink` пунктов, `tooltip` пунктов, HTML-метки (`escape = false`), `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `tieredmenu`: меню с вложенными подменю, клавиатурной навигацией и режимом всплывающей панели.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-tieredmenu` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-menu`, `prime-icons` (angleright), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-tieredmenu`).
