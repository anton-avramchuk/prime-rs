# Menu

Порт `p-menu` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-menu::Menu`, `MenuPt`, `MenuRef`, `MenuItem`, `MenuCommand` | `menu/menu.ts`, `menu/style/*` |
| CSS | `@primeuix/styles/menu` — `cargo xtask gen-theme` (блока `For PrimeNG` нет; `position` корня — инлайн-стиль, его записывает сам компонент) |
| Демонстрации | `apps/showcase/doc/menu/{basic,group,command,popup,template}-doc.ts` |

Спека: `openspec/specs/menu`.

## Как пользоваться

Статичное меню — только `model`:

```rust
view! { <Menu model=vec![MenuItem::new().label("New"), MenuItem::new().label("Search")] /> }
```

Всплывающее — `popup=true` и `MenuRef`, созданный заранее (аналог `menu.toggle($event)`):

```rust
let menu = MenuRef::new();
view! {
    <div style="position: relative">
        <Menu popup=true menu_ref=menu model=model />
        <Button label="Options" on_click=Callback::new(move |e: web_sys::MouseEvent| menu.toggle(&e)) />
    </div>
}
```

Пункт с `items` становится заголовком группы; `MenuItem::separator()` — разделитель; `command` получает `MenuCommand { event, item }`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-menu.p-component` (`p-menu-overlay` и `position: absolute` у всплывающего) > `p-menu-start` / `ul.p-menu-list[role=menu]` / `p-menu-end`; пункты `li.p-menu-item[role=menuitem]` > `p-menu-item-content` > `a.p-menu-item-link` со значком и подписью; `li.p-menu-submenu-label`, `li.p-menu-separator`; `p-focus`, `p-disabled`, `data-p-focused`, `data-p-disabled`, `aria-disabled`, `aria-activedescendant`.
- Идентификаторы пунктов (`<id меню>_<i>[_<j>]` либо `id` пункта); модель с группами определяется так же, как `hasSubMenu` (хотя бы у одного пункта есть подпункты).
- Клик: отмена перехода у пункта без `url` и у отключённого, `command`, закрытие всплывающего меню.
- Клавиши: ↓ ↑ (последний и первый остаются на месте), Home, End, Enter / NumpadEnter / Space (клик по ссылке), Escape / Tab и Alt+↑ в popup — скрыть и вернуть фокус на цель; отключённые пункты пропускаются.
- Popup: выравнивание по цели (`align_overlay`), `z-index` группы `Menu` (`auto_z_index`, `base_z_index`), фокус на список, закрытие кликом снаружи, `resize` (кроме сенсорных устройств) и прокруткой предка цели, `on_show` / `on_hide`.

## Отличия

| Отличие | Причина |
|---|---|
| `MenuItem` — свой тип без `routerLink`, `queryParams`, `badge`, `tooltipOptions`, `escape`, `automationId`, `labelClass`, `iconStyle`; значок — `ViewFn` (компонент из `prime-icons`), а не имя класса шрифта | Нет Angular-маршрутизатора; бейдж и подсказку можно положить в шаблон пункта. |
| `label` и `disabled` — значения, а не функции | Реактивность даёт `model` как сигнал. |
| Всплывающая панель выводится внутри корня, а не в `body` (`appendTo` нет); родитель — `position: relative` без `overflow: hidden` | Так же устроен `Popover`. |
| Нет `@primeuix/motion`, `motionOptions`, `show/hideTransitionOptions`: панель появляется и исчезает без анимации | Анимации не переносятся. |
| Свой вид — `item`, `submenu_header` (`Callback`), `start`, `end` (`ViewFn`), а не `pTemplate` | Аналог шаблонов. |
| Невидимая группа скрывается вместе с пунктами | У PrimeNG пункты скрытой группы с собственным `visible` остаются. |
| В `pt` нет контекста (`item`, `index`, `focused`, `disabled`), нет `Bind` и `Ripple` | `pt` — простые добавки классов, стиля и атрибутов. |
| Клавиши работают по модели, а не по DOM | Тот же порядок; отключённые пункты пропускаются. |
| Демо `router` не переносится; `command` показывает `Toast`; значки `cog`, `sign-out`, `ellipsis-v` заменены ближайшими из `prime-icons`, `template` — без логотипа и аватара | Нет маршрутизатора и этих иконок. |

## Стоимость

158 аллокаций на экземпляр при SSR (три пункта); вклад в wasm ≈ 383 КБ (бюджет 400 КБ в `benches/budgets.toml`).

## Не проверено

Касание, `rtl`, экранные считыватели, прокрутка предка цели на реальном вводе, закрытие по `resize` на сенсорном устройстве. Проверены в Chrome (браузерные тесты и e2e с настоящей мышью и клавишами): разметка, группы и разделители, невидимые и отключённые пункты, клик и `command`, фокус и все клавиши, показ popup у кнопки, закрытие кликом снаружи, Escape, Alt+↑, выбором пункта и `resize`, `z-index`, `on_show` / `on_hide`, шаблоны. Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось.
