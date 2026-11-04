# Proposal

## Why

PanelMenu — третье меню волны 5: гибрид Accordion и Tree, вертикальный список панелей, каждая раскрывается на месте (без всплывающих подменю). Состояние раскрытия, коллапс-анимация и разметка заголовка близки `Accordion` (`prime-core::collapsible`), а модель пунктов и клавиатура дерева (↑ ↓ ← → `Home` `End` `Enter`) — к `prime-tieredmenu`, но с другим понятием «видимый пункт» (раскрытые ветки, а не всплывающие уровни).

## What Changes

- Новый крейт `prime-panelmenu` (порт `p-panelmenu` из PrimeNG 21.1.10). Модель пунктов — `prime_menu::MenuItem` (`key`, `expanded`, `disabled`, `visible`, `separator`, `items`, `command`).
- `PanelMenu`: корень `div.p-panelmenu.p-component` (`id`, `style`, `style_class`) > на каждый корневой пункт `div.p-panelmenu-panel` > `div.p-panelmenu-header` (`p-panelmenu-header-active`, `p-disabled`; `aria-expanded`, `aria-controls`, `aria-disabled`, `data-p-highlight`) > `div.p-panelmenu-header-content` > `a.p-panelmenu-header-link` (`p-panelmenu-header-icon`, `p-panelmenu-header-label`, `p-panelmenu-submenu-icon` — шеврон вправо/вниз) + `div.p-panelmenu-content-container` (`p-panelmenu-expanded`) > `div.p-panelmenu-content-wrapper` > `div.p-panelmenu-content` > `ul.p-panelmenu-root-list[role=tree]` > `li.p-panelmenu-item[role=treeitem]` (`aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded`, `p-focus`, `p-disabled`) > `div.p-panelmenu-item-content` > `a.p-panelmenu-item-link` (`-item-icon`, `-item-label`, `-submenu-icon`) + вложенный `ul.p-panelmenu-submenu`; разделители `p-menuitem-separator`.
- Поведение: клик по заголовку/группе переключает раскрытие; при `multiple=false` раскрыта одна корневая панель (раскрытие другой сворачивает прежнюю), при `multiple=true` — любое число; лист вызывает `command`; начальное состояние берётся из `expanded` пунктов; управляемое раскрытие — через `PanelMenuRef` (`expand_all`/`collapse_all`/`toggle_all`) и/или сигнал набора раскрытых ключей (по `key`, как в демо `controlled`). Раскрытие/сворачивание — анимация `prime-core::collapsible`.
- Клавиатура (фокус в корне, `aria-activedescendant`): ↓ ↑ по видимым пунктам и заголовкам, → раскрывает ветку (или идёт вниз), ← сворачивает (или идёт к родителю), `Home`/`End`, `Enter`/`Space` — активация (переключение/команда). `Tab` уходит из компонента; недоступные пункты пропускаются.
- Свойства: `model`, `style`, `style_class`, `multiple` (по умолчанию `false`), `id`, `tabindex` (0); представления `item` (замена содержимого `-item-content`) и `submenu_icon`; события `on_focus`/`on_blur` не входят — их нет в исходнике.
- Страница `/panelmenu` (`basic`, `multiple`, `controlled`, `command`, `template`), e2e, бенчмарки и бюджеты, `docs/panelmenu.md`, `panelmenu` в `theme.components` (и в `primeng_extra`: в исходнике стилей есть блок `/*For PrimeNG*/`).
- **Не входит:** `routerLink` и демонстрация `router`, `Tooltip`/`Badge` пунктов, HTML-метки (`escape: false`), `Ripple`, `Bind`, `pTemplate`, `transitionOptions` (анимация единая), `accessibility`-демо как отдельный код.

## Capabilities

### New Capabilities
- `panelmenu`: вертикальное меню-аккордеон с вложенными ветками, клавиатурой и управляемым раскрытием.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-panelmenu` (`publish = false`); зависимости: `prime-core` (`collapsible`, `ids`), `prime-dom`, `prime-menu`, `prime-icons` (chevronright, chevrondown; для демо — несколько значков), при необходимости чистая модель из `prime-tieredmenu` (только то, что подходит; иначе собственная плоская модель в крейте).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места). Поведение существующих крейтов не меняется.
