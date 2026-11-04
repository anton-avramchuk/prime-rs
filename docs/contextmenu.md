# ContextMenu

Порт `p-contextmenu` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-contextmenu::ContextMenu`, `ContextMenuRef`, `ContextMenuPt`, `ContextMenuItemContext`, `place`/`Viewport` | `packages/primeng/src/contextmenu/contextmenu.ts` (`ContextMenu` и `ContextMenuSub`), `contextmenu/style/contextmenustyle.ts` |
| Модель переходов (путь, фокус, поиск) | общая с `TieredMenu`: `prime_tieredmenu::{Tree, State, Node, …}` |
| CSS | `@primeuix/styles/contextmenu` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/contextmenu/*-doc.ts` (`basic`, `command`, `template`) и добавленная `nested` |

Спека: `openspec/specs/contextmenu`. Модель пунктов — `prime_menu::MenuItem` (поле `items` задаёт подменю), команды — `MenuCommand`.

```rust
view! {
    <div id="area">"Right click here"</div>
    <ContextMenu target="#area" model=items />
}

// или вручную
let menu = ContextMenuRef::new();
view! {
    <li on:contextmenu=move |e| menu.show(&e)>"Row"</li>
    <ContextMenu model=items popup_ref=menu />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-contextmenu.p-component` (`p-contextmenu-mobile`) > `ul.p-contextmenu-root-list[role=menu]` > `li.p-contextmenu-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-contextmenu-item-content` > `a.p-contextmenu-item-link`; вложенные `ul.p-contextmenu-submenu`; разделители. Панели в DOM нет, пока меню не показано.
- Показ: событие `contextmenu` (`trigger_event`) на цели или на документе (`global`); на iOS/Android — долгое касание (`press_delay`, 500 мс). Показ отменяет действие по умолчанию и всплытие, сбрасывает путь и фокус, вызывает `on_show`.
- Положение: (`pageX + 1`, `pageY + 1`), переворот влево/вверх, если панель не помещается, прижатие к левому/верхнему краю — чистая функция `place`.
- Закрытие: левый клик вне меню (правая кнопка и Ctrl-клик не закрывают), `resize`, лист, `Escape`, `Tab`; слой группы `Menu`; `on_hide`.
- Подменю, клавиатура (↓ ↑ → ← `Home` `End` `Enter` `Space`, поиск по началу подписи) и сторона раскрытия — как у `TieredMenu`.

## Отличия

| Отличие | Причина |
|---|---|
| `target` — CSS-селектор (разрешается при монтировании) или `global`; для произвольного элемента — `ContextMenuRef::show` из своего обработчика `contextmenu` | Нет `HTMLElement` как свойства компонента; `ContextMenuRef` покрывает остальное (в том числе строки таблицы и списков). |
| Нет `appendTo`: панель выводится в месте компонента и позиционируется абсолютно; координаты страницы пересчитываются в координаты ближайшего позиционированного предка | Так же устроены `Menu`, `TieredMenu`. Предок с `overflow: hidden` обрежет панель. |
| Подменю выравнивается в процентах (`left: 100%` / `-100%`), как у `TieredMenu`, а не в пикселях | Общая модель с `TieredMenu`; результат тот же. |
| Нет `motionOptions`, анимации входа и выхода | Тема не подключает классы анимации. |
| Нет `routerLink`, подсказок пунктов, `escape = false`, `Ripple`, `Bind`, `pTemplate`; значки — `ViewFn` | Нет маршрутизатора, `innerHTML` и PrimeIcons. |
| Нет интеграции с `Table` | Таблица не портирована. |
| Демонстрации `router` и `table` не перенесены; `basic` использует блок вместо картинки, добавлена `nested` | Нет маршрутизатора и сети; `nested` показывает подменю. |

## Что не проверялось

- Долгое касание (`touchstart`/`touchend` с таймером `press_delay`): ни автотестами, ни на реальном устройстве.
- Раскладка справа налево.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Позиционирование внутри прокручиваемых контейнеров и предков с `transform`.

## Производительность

См. `benches/` (`contextmenu-1`, `contextmenu-1000`, `case-contextmenu`) и `benches/*budgets.toml`.
