# Proposal

## Why

Dock — четвёртый компонент волны 4: панель быстрого запуска из значков, закреплённая у края контейнера (как док в macOS). Он простой по логике (список пунктов, фокус с клавиатуры, адаптация по ширине экрана) и не зависит от тяжёлых компонентов.

## What Changes

- Новый крейт `prime-dock` (порт `p-dock` из PrimeNG 21.1.10).
- `DockItem` (вместо `MenuItem` PrimeNG, без полей маршрутизатора): `id`, `label`, `icon` (`ViewFn`), `url`, `target`, `title`, `disabled`, `badge`, `command` (`Callback<DockCommand>`), `style_class`, `style`, `link_class`, `icon_class`.
- `Dock`: корень `div.p-dock.p-component.p-dock-{position}` (`p-dock-mobile` при ширине окна не больше `breakpoint`) > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` (`aria-orientation`, `tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`) > пункты `li.p-dock-item[role=menuitem]` (`p-focus`, `p-disabled`, `data-p-focused`, `data-p-disabled`) > `div.p-dock-item-content` > `a.p-dock-item-link` (`aria-hidden`, `href`, `target`, `title`) со значком `span.p-dock-item-icon`, представлением `item` и бейджем.
- Свойства: `model`, `position` (`DockPosition`: `Bottom` по умолчанию, `Top`, `Left`, `Right`), `breakpoint` (`960px`), `id`, `aria_label`, `aria_labelledby`, `style_class`, `style`, `pt`; представление `item` (вместо значка пункта); события `on_focus`, `on_blur`.
- Поведение: клик по пункту вызывает `command` (не у отключённого); фокус на списке ставит фокус на первый доступный пункт; стрелки по ориентации (→/← для верхнего и нижнего дока, ↓/↑ для бокового), `Home`, `End`, `Enter`/`Space` кликают по ссылке пункта; отключённые пункты пропускаются; `media query` с `breakpoint` включает режим `p-dock-mobile`.
- Страница showcase `/dock` с демонстрацией `basic` (выбор положения); e2e; бенчмарки и бюджеты; `docs/dock.md`.
- Стили: `dock` добавляется в `theme.components`.
- **Не входит:** `routerLink` и сопутствующие поля, `tooltipOptions` (подсказку можно добавить директивой `use:tooltip` в шаблоне `item`), демонстрация `advanced` (нужны Terminal, Galleria и Dialog внутри дока), `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `dock`: панель значков у края контейнера с клавиатурной навигацией и адаптацией по ширине окна.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-dock` (`publish = false`); зависимости: `prime-core`, `prime-badge`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
