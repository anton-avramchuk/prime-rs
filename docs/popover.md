# Popover

Порт `p-popover` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-popover::Popover`, `PopoverRef`, `PopoverPt` | `popover/popover.ts`, `popover/style/*` |
| CSS | `@primeuix/styles/popover` — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/popover/{basic,target,selectdata,template}-doc.ts` |

Спека: `openspec/specs/popover`.

## Как пользоваться

Панелью управляет `PopoverRef` (`Copy`), аналог `op.toggle($event)` из PrimeNG. Его создают заранее, передают в `Popover` и вызывают из обработчиков триггера:

```rust
let op = PopoverRef::new();
view! {
    <button on:click=move |e| op.toggle(&e)>"Share"</button>
    <Popover popover_ref=op>"Content"</Popover>
}
```

Цель выравнивания — `currentTarget` события; другую цель передают в `show_at(&el)` / `toggle_at(&el)`. `hide()` скрывает панель, `visible()` читает состояние (его можно использовать для `aria-expanded` триггера).

## Что совпадает с PrimeNG

- Разметка: `div.p-popover.p-component[role=dialog][aria-modal=true]` > `div.p-popover-content`; `aria-label`, `aria-labelledby`, `id`, `style`, `style_class`, `pt`.
- Выравнивание по цели (`align_overlay`, как у `Select`), сдвиг стрелки `--p-popover-arrow-left`, `p-popover-flipped` и `data-p-popover-flipped` при размещении над целью, `z-index` группы оверлеев (`auto_z_index`, `base_z_index`).
- Закрытие: клик вне панели и вне цели при `dismissable`, `Escape`, `resize` окна (не на сенсорных устройствах), прокрутка предка цели; повторный `toggle` с другой целью переносит панель к новой цели.
- `focus_on_show`: фокус получает первый элемент с `autofocus`; `on_show` и `on_hide`.
- Слушатели окна вешаются только на время показа и снимаются при скрытии и при размонтировании.

## Отличия

| Отличие | Причина |
|---|---|
| Панель выводится внутри собственного места в разметке, `appendTo` нет | Так же устроен `Select`; нужен предок с `position: relative`, а `overflow: hidden` у предка обрежет панель. |
| Управление — `PopoverRef`, а не ссылка на экземпляр (`#op`) | В Leptos нет шаблонных ссылок на компонент. |
| Нет анимации появления и скрытия (`@primeuix/motion`) | Панель появляется и исчезает мгновенно; `motionOptions` и `*TransitionOptions` не переносятся. |
| Нет `OverlayService` | Клик во вложенном оверлее, чья панель лежит вне DOM-поддерева `Popover`, закроет внешнюю панель. Панель `Select` лежит внутри своего корня, поэтому такой клик считается внутренним. |
| На iOS закрытие по `click`, а не по `touchstart` | Один тип события на всех. |
| Нет `aria-expanded` / `aria-controls` на триггере | Триггер — произвольный элемент пользователя; `aria-expanded` задают по `op.visible()`. |
| Содержимое — `children`, нет шаблона `content` с `closeCallback` | Закрытие из содержимого — вызов `op.hide()`. |
| Демонстрация `datatable` не перенесена; значков `share-alt`, `copy`, `users`, `image` в `prime-icons` нет | Нужен DataTable; кнопки без значков, аватары — инициалы. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |

## Стоимость

33 аллокации на экземпляр при SSR (скрытая панель); вклад в wasm ≈ 291 КБ (бюджет 325 КБ в `benches/budgets.toml`).

## Не проверено

Касание (`touchstart`, `resize` на сенсорном устройстве), `rtl` и экранные считыватели. Проверены разметка, показ и скрытие, клик снаружи и внутри, Escape, смена цели, явная цель, `autofocus`, переворот над целью у нижнего края, закрытие по прокрутке предка и снятие слушателей в Chrome (браузерные тесты и e2e). Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось.
