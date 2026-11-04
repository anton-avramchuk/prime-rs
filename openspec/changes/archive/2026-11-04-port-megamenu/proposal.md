# Proposal

## Why

MegaMenu — четвёртое меню волны 5: корневая панель (горизонтальная или вертикальная), у каждого пункта которой раскрывается широкое наложение с колонками подменю, а в них — пункты. Состояние путей, клик вне, `resize`, мобильная кнопка и автоскрытие уже отработаны в `prime-menubar`; новое — двухуровневая модель «пункт → колонки → подменю → пункты», ориентация, раскладка колонок и своя навигация клавиатурой (по колонкам).

## What Changes

- Новый крейт `prime-megamenu` (порт `p-megamenu` из PrimeNG 21.1.10).
- Модель: `MegaMenuItem` — корневой пункт (поля как у `prime_menu::MenuItem`: `label`, `icon`, `url`, `target`, `title`, `disabled`, `visible`, `separator`, `command`, классы) с полем `columns: Vec<Vec<MegaMenuSubmenu>>`; `MegaMenuSubmenu` — заголовок колонки (`label`, `visible`, классы) с `items: Vec<MenuItem>` (плоские пункты без вложенных подменю, как в PrimeNG).
- `MegaMenu`: корень `div.p-megamenu.p-component` (`p-megamenu-horizontal|vertical`, `p-megamenu-mobile`, `p-megamenu-mobile-active`; `id`, `style`, `style_class`) > `div.p-megamenu-start`, `a.p-megamenu-button` (при непустой модели), `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`) > `li.p-megamenu-item` > `div.p-megamenu-item-content` > `a.p-megamenu-item-link` (значок, подпись, стрелка вниз/вправо) + `div.p-megamenu-overlay` > `div.p-megamenu-grid` > `div.p-megamenu-col-N` (N по числу колонок: 12/6/4/3/2, на узком окне 12) > `ul.p-megamenu-submenu[role=menu]` > `li.p-megamenu-submenu-label[role=presentation]` и `li.p-megamenu-item`; разделители `p-megamenu-separator`; `div.p-megamenu-end`.
- Поведение: клик по корневому пункту с колонками раскрывает наложение, повторный — закрывает; лист вызывает `command` и закрывает; наведение раскрывает при `dirty` (после клика), на узком окне путь не меняется; клик вне и `resize` закрывают; мобильная кнопка и слой `Menu` как у Menubar. `auto_hide` в MegaMenu нет.
- Клавиатура по исходнику: ↓ ↑ → ← по корню и внутри наложения (с учётом ориентации), `Home` `End` `Enter` `Space` `Escape` `Tab`, поиск по началу подписи.
- Свойства: `model`, `orientation` (`Horizontal` по умолчанию, `Vertical`), `style`, `style_class`, `breakpoint` (`960px`), `scroll_height`, `disabled`, `tabindex`, `id`, `aria_label`, `aria_labelledby`, `auto_z_index`, `base_z_index`; представления `start`, `end`, `item`, `submenu_icon`, `menu_icon`.
- Страница `/megamenu` (`basic`, `vertical`, `command`, `template`), e2e, бенчмарки и бюджеты, `docs/megamenu.md`, `megamenu` в `theme.components` (блока `For PrimeNG` в исходнике стилей нет).
- **Не входит:** `routerLink` и демонстрация `router`, `Tooltip`/`Badge`, HTML-метки, `Ripple`, `Bind`, `pTemplate`, `button` как полная замена кнопки (`#button`), `accessibility`-демо.

## Capabilities

### New Capabilities
- `megamenu`: меню с широким наложением из колонок подменю, двумя ориентациями, клавиатурой и мобильным режимом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-megamenu` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-menu`, `prime-icons` (angledown, angleright, bars), по итогам D1 — возможно `prime-tieredmenu` (только общие вспомогательные функции).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места). Поведение существующих крейтов не меняется.
