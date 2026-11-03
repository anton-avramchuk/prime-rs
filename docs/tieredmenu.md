# TieredMenu

Порт `p-tieredmenu` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-tieredmenu::TieredMenu`, `TieredMenuRef`, `TieredMenuPt`, `TieredMenuItemContext`, чистая модель (`Tree`, `State`, `Node`) | `packages/primeng/src/tieredmenu/tieredmenu.ts` (`TieredMenu` и `TieredMenuSub`), `tieredmenu/style/tieredmenustyle.ts` |
| CSS | `@primeuix/styles/tieredmenu` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/tieredmenu/*-doc.ts` (`basic`, `popup`, `command`, `template`) |

Спека: `openspec/specs/tieredmenu`. Модель пунктов — `prime_menu::MenuItem` (поле `items` задаёт подменю), команды — `MenuCommand`.

```rust
let menu = TieredMenuRef::new();
view! {
    <button on:click=move |e| menu.toggle(&e)>"Toggle"</button>
    <TieredMenu model=items popup=true popup_ref=menu />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-tieredmenu.p-component` (`p-tieredmenu-overlay`, `p-tieredmenu-mobile`) > `ul.p-tieredmenu-root-list[role=menu]` > `li.p-tieredmenu-item` (`p-tieredmenu-item-active`, `p-focus`, `p-disabled`, `data-p-highlight|focused|disabled`, `aria-haspopup`, `aria-expanded`, `aria-setsize`, `aria-posinset`) > `div.p-tieredmenu-item-content` > `a.p-tieredmenu-item-link` со значком, подписью и стрелкой группы; вложенное `ul.p-tieredmenu-submenu`; разделители `p-tieredmenu-separator`. Подменю выводится, только пока группа раскрыта.
- Состояние переходов — копия исходника и вынесена в чистую модель: дерево ключей (`0`, `0_1`, …), путь раскрытых групп, фокус по уровню, признак «раскрывали кликом» (`dirty`), клик (раскрыть / свернуть ветку / лист закрывает всё и ведёт фокус на корневой пункт ветки), смена пути, наведение (после клика; на касании сразу; на узком окне путь не меняется), `auto_display`.
- Клавиатура: ↓ ↑ → ← `Home` `End` `Enter` `Space` `Escape` `Tab`, `Alt+↑` в `popup`, поиск по началу подписи без учёта регистра по кругу с буфером, сбрасываемым через 500 мс; `aria-activedescendant`.
- Положение подменю (`nestedPosition`): справа от пункта (`left: 100%`, `top: 0`), при нехватке места справа — слева (`-100%`), с учётом полосы прокрутки и очень узких окон.
- Режим `popup`: выравнивание по цели (`AlignMode::Relative`, минимальная ширина по цели), слой группы `Menu`, клик вне / `resize` (не на сенсорных) / прокрутка предка закрывают, `Escape` возвращает фокус на цель, `on_show`/`on_hide`.
- Мобильный режим по `matchMedia` с `breakpoint` (по умолчанию `960px`).

## Отличия

| Отличие | Причина |
|---|---|
| Клик по ветке сворачивает её по границам ключей: ключ `"1"` не считается предком `"10"` | В PrimeNG проверка `key.startsWith(p.key)` по строке, поэтому при десяти и более соседях сворачивание затрагивало чужие ветки. |
| У подменю нет `tabindex` и `id` повторяются не по всей странице: `<id меню>_<ключ>_list` | В PrimeNG у всех подменю одинаковый `id` `<id меню>_list` и `tabindex="0"`; это ошибка разметки. |
| Нет `appendTo`: всплывающее меню выводится в собственном месте разметки (нужен предок с `position: relative`) | Так же устроены `Menu`, `Popover`. |
| Нет `showTransitionOptions`, `hideTransitionOptions`, `motionOptions`: меню и подменю появляются без анимации входа | Тема не подключает классы анимации; PrimeNG ставит `p-anchored-overlay`. |
| Нет `routerLink`, подсказок пунктов, `escape = false` (HTML в подписи), `Ripple`, `Bind`, `pTemplate`; значки — `ViewFn` | Нет маршрутизатора, PrimeIcons и `innerHTML`; остальное не переносится. |
| Демонстрация `router` не перенесена; значки PrimeIcons без аналога в `prime-icons` в демонстрациях опущены | Нет маршрутизатора и PrimeIcons. |
| `SplitButton` по-прежнему показывает плоское `Menu`: подключение `TieredMenu` — отдельное изменение | Меняется API и тесты `SplitButton`. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод (на касании наведение раскрывает группы сразу, как в PrimeNG, но реальных касаний в тестах нет).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Подменю глубже трёх уровней у самого края окна.

## Производительность

См. `benches/` (`tieredmenu-1`, `tieredmenu-1000`, `case-tieredmenu`) и `benches/*budgets.toml`.
