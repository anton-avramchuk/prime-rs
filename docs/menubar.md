# Menubar

Порт `p-menubar` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-menubar::Menubar`, `MenubarPt`, `MenubarItemContext` | `packages/primeng/src/menubar/menubar.ts` (`Menubar` и `MenubarSub`), `menubar/style/menubarstyle.ts` |
| Модель переходов (путь, фокус, поиск) | общая с `TieredMenu`: `prime_tieredmenu::{Tree, State, Node, …}`; клавиши горизонтального корня — `Tree::bar_down/bar_up/bar_left/bar_right` |
| CSS | `@primeuix/styles/menubar` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/menubar/*-doc.ts` (`basic`, `command`, `template`) |

Спека: `openspec/specs/menubar`. Модель пунктов — `prime_menu::MenuItem` (поле `items` задаёт подменю), команды — `MenuCommand`.

```rust
view! {
    <Menubar
        model=items
        start=ViewFn::from(|| view! { <b>"Logo"</b> })
        end=ViewFn::from(|| view! { <input placeholder="Search" /> })
    />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-menubar.p-component` (`p-menubar-mobile`, `p-menubar-mobile-active`) > `div.p-menubar-start`, `a.p-menubar-button[role=button]`, `ul.p-menubar-root-list[role=menubar]` > `li.p-menubar-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-menubar-item-content` > `a.p-menubar-item-link`; вложенные `ul.p-menubar-submenu` (инлайн `display: flex`, пока группа раскрыта); разделители; `div.p-menubar-end`. Стрелка группы: вниз у корня, вправо у вложенных.
- Клавиатура: ← → по корню, ↓ на группе корня раскрывает её и входит, ↑ на группе корня — на последний пункт, ↑ на первом пункте подменю — на родителя; внутри подменю ↓ ↑ → ←; `Home` `End` `Enter` `Space` `Escape` `Tab`; поиск по началу подписи с буфером на 500 мс. Фокус снаружи сразу ставит первый пункт.
- Наведение раскрывает группы только после клика/нажатия мыши по меню (`dirty`) и при `auto_display`; на узком окне путь не меняется. Повторный клик по раскрытому пункту сворачивает его (после наведения, которое уже раскрыло пункт, клик, как в PrimeNG, его закроет).
- Клик вне и `resize` (не на сенсорных) закрывают подменю; клик вне при открытой мобильной панели закрывает её.
- Мобильный режим по `matchMedia` с `breakpoint` (по умолчанию `960px`): кнопка, `aria-expanded`, слой группы `Menu` у открытого списка, возврат фокуса на кнопку; смена размера окна через порог закрывает панель.

## Отличия

| Отличие | Причина |
|---|---|
| `auto_hide`: возврат курсора в меню до истечения `auto_hide_delay` отменяет закрытие | В PrimeNG закрытие идёт через `debounce` без отмены и срабатывает, даже если курсор вернулся. |
| Подменю не отзеркаливаются у правого края окна | В PrimeNG у меню-бара этого тоже нет: положение задаёт CSS темы (`left: 100%`). |
| ↑ ↓ → ← всегда отменяют действие по умолчанию | В PrimeNG ↓ на листе корня и → на листе подменю оставляют его; прокрутка страницы от стрелок в меню не нужна. |
| `aria-label` кнопки берётся из `locale` (`aria.navigation`) | Так же в PrimeNG (`config.translation.aria.navigation`). |
| Нет `routerLink`, `Tooltip`/`Badge` пунктов, `escape = false`, `Ripple`, `Bind`, `pTemplate`; значки — `ViewFn` | Нет маршрутизатора, `innerHTML` и PrimeIcons. |
| Нет `appendTo` и анимаций | Подменю позиционирует CSS темы. |
| Демонстрация `router` не перенесена; значки без аналога в `prime-icons` заменены близкими | Нет маршрутизатора и PrimeIcons. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод (на касании наведение раскрывает группы сразу, как в PrimeNG, но реальных касаний в тестах нет).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Подменю у самого правого края окна (вылезают за окно, как и в PrimeNG).

## Производительность

См. `benches/` (`menubar-1`, `menubar-1000`, `case-menubar`) и `benches/*budgets.toml`.
