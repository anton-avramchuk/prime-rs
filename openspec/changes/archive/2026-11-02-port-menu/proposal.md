# Proposal

## Why

Menu — девятый компонент волны 3: список команд и ссылок, статичный или всплывающий у кнопки. Он нужен как основа для следующих меню (ContextMenu, TieredMenu, Menubar) и закрывает типовой сценарий «кнопка с троеточием → список действий». Всплывающий режим повторяет Popover (панель внутри корня, управляющий объект), список с группами и навигацией клавишами — новая часть.

## What Changes

- Новый крейт `prime-menu` (порт `p-menu` из PrimeNG 21.1.10).
- `MenuItem` (вместо `MenuItem` PrimeNG, без полей маршрутизатора Angular): `id`, `label`, `icon` (`ViewFn`), `url`, `target`, `title`, `disabled`, `visible` (да), `separator`, `items` (подпункты — группа), `command` (`Callback<MenuCommand>`), `style_class`, `style`, `link_class`, `icon_class`.
- `Menu`: корень `div.p-menu.p-component` (с `p-menu-overlay` и `position: absolute` во всплывающем режиме) > необязательные `start`/`end` > `ul.p-menu-list[role=menu]` с пунктами `li.p-menu-item[role=menuitem]` > `div.p-menu-item-content` > `a.p-menu-item-link` (значок и `span.p-menu-item-label`), заголовками групп `li.p-menu-submenu-label` и разделителями `li.p-menu-separator`.
- Свойства: `model`, `popup`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `aria_label`, `aria_labelledby`, `id`, `tabindex` (0), `pt`; представления `item` (вместо содержимого пункта), `submenu_header`, `start`, `end`; события `on_show`, `on_hide`, `on_focus`, `on_blur`.
- `MenuRef` (`Copy`): `toggle(event)`, `show(event)`, `hide()` — как `menu.toggle($event)` в PrimeNG; цель — `currentTarget` события.
- Поведение: клик по пункту вызывает `command` и закрывает всплывающее меню; отключённый пункт и пункт без `url` отменяют переход; клавиши ↓ ↑ Home End Enter Space Escape Tab (в popup Escape/Tab/Alt+↑ возвращают фокус на цель); фокус (`p-focus`, `aria-activedescendant`); клик вне, прокрутка предка и resize закрывают всплывающее меню; `z-index` группы `Menu`.
- Страница showcase `/menu` с демонстрациями `basic`, `group`, `command` (через `MessageService` и `Toast`), `popup`, `template`, `accessibility`-описание; e2e; бенчмарки и бюджеты; `docs/menu.md`.
- Стили: `menu` добавляется в `theme.components`.
- **Не входит:** `routerLink` и сопутствующие поля, `appendTo` (панель всегда внутри корня, как в Popover), `@primeuix/motion`/`motionOptions`/`show/hideTransitionOptions` (анимация — CSS-классы темы), `tooltipOptions` пунктов, `badge` пунктов, `escape=false` (HTML в подписи), `Ripple`, `pt` с контекстом, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `menu`: список команд и ссылок, статичный или всплывающий, с группами, разделителями и навигацией клавишами.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-menu` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
