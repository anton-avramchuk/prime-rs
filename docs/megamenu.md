# MegaMenu

Порт `p-megamenu` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-megamenu::MegaMenu`, `MegaMenuPt`, `MegaMenuItemContext`, `MegaMenuItem`, `MegaMenuSubmenu`, `MegaMenuOrientation` | `packages/primeng/src/megamenu/megamenu.ts` (`MegaMenu` и `MegaMenuSub`), `megamenu/style/megamenustyle.ts` |
| Модель переходов (путь, фокус, поиск) | `prime-megamenu::state` — своя: `build`, `Menu`, `State` |
| CSS | `@primeuix/styles/megamenu` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/megamenu/*-doc.ts` (`basic`, `vertical`, `command`, `template`) |

Спека: `openspec/specs/megamenu`. Корневой пункт — `MegaMenuItem` (обычный `prime_menu::MenuItem` плюс `columns`), колонка — список `MegaMenuSubmenu` (заголовок и плоские `MenuItem`). Ключи узлов: корень `"0"`, подменю `"0_<колонка>_<индекс>"`, пункт `"0_1_0_2"`; они же в `id` элементов `li` (`<id меню>_<ключ>`).

```rust
let model = vec![MegaMenuItem::new().label("Furniture").columns(vec![
    vec![MegaMenuSubmenu::new("Living Room").items(vec![MenuItem::new().label("Armchair")])],
    vec![MegaMenuSubmenu::new("Kitchen").items(vec![MenuItem::new().label("Chair")])],
])];
view! { <MegaMenu model=model orientation=MegaMenuOrientation::Vertical /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-megamenu.p-component` (`-horizontal|-vertical`, `-mobile`, `-mobile-active`) > `div.p-megamenu-start`, `a.p-megamenu-button`, `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`) > `li.p-megamenu-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-megamenu-item-content` > `a.p-megamenu-item-link` и `div.p-megamenu-overlay` > `div.p-megamenu-grid` > `div.p-megamenu-col-N` > `ul.p-megamenu-submenu[role=menu]` > `li.p-megamenu-submenu-label` и пункты; разделители; `div.p-megamenu-end`. Класс колонки: 2 → `col-6`, 3 → `col-4`, 4 → `col-3`, 6 → `col-2`, иначе и на узком окне — `col-12`. Стрелка: вниз в горизонтальной ориентации и на мобильном, вправо в вертикальной.
- Клик по корню раскрывает/закрывает наложение, лист вызывает команду и закрывает всё; наведение работает только после клика/нажатия мыши по меню (`dirty`) и не на узком окне; клик вне и `resize` (не на сенсорных) закрывают.
- Клавиатура по `onArrow*Key`: в горизонтальной ориентации ← → по корню, ↓ раскрывает группу и входит в первую колонку, ↓ на обычном корне идёт к следующему; внутри наложения ↓ ↑ по плоскому списку пунктов, → ← между колонками (первый допустимый пункт колонки), `Alt+↑` возвращает на корень; в вертикальной — ↓ ↑ по корню, → раскрывает и входит, ← из первой колонки возвращает на корень; `Home` `End` `Enter` `Space` `Escape` `Tab`; поиск по началу подписи с буфером на 500 мс. Фокус снаружи сразу ставит первый пункт.
- Мобильный режим по `matchMedia` с `breakpoint` (по умолчанию `960px`): кнопка, `aria-expanded`, слой группы `Menu` у открытого списка, возврат фокуса на кнопку; смена размера окна через порог закрывает панель.

## Отличия

| Отличие | Причина |
|---|---|
| Переходы по корню при открытом наложении (← → ↓ ↑) идут по корневым пунктам, а наложение следует за фокусом (у обычного корня закрывается) | В PrimeNG при открытом наложении `visibleItems` — список пунктов подменю, и индекс корня указывает на посторонний пункт. |
| Наведение на обычный корневой пункт закрывает открытое наложение | В PrimeNG оно остаётся открытым. |
| Невидимые и отключённые пункты, а также скрытые подменю пропускаются при навигации и поиске | В PrimeNG `isValidItem` не проверяет `visible`. |
| Наложение выводится только у раскрытого пункта | Показ задаёт CSS темы (`.p-megamenu-item-active > .p-megamenu-overlay`); так дешевле по DOM. |
| Инлайн-стиль корневого списка (`max-height`, `overflow: auto`) задаётся только при `scroll_height` | В PrimeNG `overflow: auto` ставится всегда; у горизонтального меню это обрезало бы наложение. |
| `aria-label` кнопки берётся из `locale` (`aria.navigation`) | Так же в PrimeNG (`config.translation.aria.navigation`). |
| Нет `routerLink`, `Tooltip`/`Badge` пунктов, `escape = false`, `Ripple`, `Bind`, `pTemplate`, замены кнопки целиком (`#button`); значки — `ViewFn` | Нет маршрутизатора, `innerHTML` и PrimeIcons. |
| Демонстрации `router` и `accessibility` не перенесены; значки без аналога в `prime-icons` заменены близкими | Нет маршрутизатора и PrimeIcons. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод (на касании наведение раскрывает сразу, как в PrimeNG, реальных касаний в тестах нет).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Наложение у самого правого края окна (вылезает за окно, положение задаёт CSS темы).

## Производительность

См. `benches/` (`megamenu-1`, `megamenu-1000`, `case-megamenu`) и `benches/*budgets.toml`.
