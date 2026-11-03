# Dock

Порт `p-dock` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-dock::Dock`, `DockItem`, `DockCommand`, `DockPosition`, `DockPt` | `packages/primeng/src/dock/dock.ts`, `dock/style/dockstyle.ts` |
| CSS | `@primeuix/styles/dock` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/dock/*-doc.ts` (`basic`; `advanced` с увеличением значков и `menu` вынесены в отличия) |

Спека: `openspec/specs/dock`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-dock.p-component.p-dock-<положение>` > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` > `li.p-dock-item[role=menuitem]` > `div.p-dock-item-content` > `a.p-dock-item-link[aria-hidden]`; `p-dock-mobile` при ширине окна не больше `breakpoint` (по умолчанию `960px`, через `matchMedia`).
- Положения `bottom` (по умолчанию), `top`, `left`, `right`; `aria-orientation` следует за ними.
- Клавиатура на списке: стрелки вдоль дока, `Home`/`End`, `Enter`/`Space` — клик по ссылке пункта. Поперечные стрелки перехватываются без действия. Отключённые пункты пропускаются, на границе фокус остаётся.
- Фокус показан классом `p-focus`, `aria-activedescendant` списка указывает на `id` пункта; `on_focus`/`on_blur`.
- Пункт: `label`, `icon`, `url`, `target`, `title`, `disabled`, `badge`, `command`, `style_class`, `style`; идентификатор — свой `id` или `<id>_<индекс>`.
- `item` заменяет содержимое пункта (значок и бейдж).

## Отличия

| Отличие | Причина |
|---|---|
| Увеличение значков при наведении (`advanced`) не входит в компонент | В PrimeNG это CSS демонстрации, а не стиль компонента. Шаблон `item` позволяет повторить его снаружи. |
| Значки — `ViewFn`, а не классы `pi pi-…` | Нет PrimeIcons. |
| Нет `tooltip`-обёртки, `dir`, `routerLink` | Подсказка добавляется снаружи директивой `use:tooltip` (см. демонстрацию); маршрутизация — через `url`/`command`. |
| Корень — `<div>`, а не хост `<p-dock>` | Хост в Angular — деталь фреймворка. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`dock-1`, `dock-1000`, `case-dock`) и `benches/*budgets.toml`.
