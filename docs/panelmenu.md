# PanelMenu

Порт `p-panelmenu` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-panelmenu::PanelMenu`, `PanelMenuPt`, `PanelMenuItemContext`, `PanelMenuRef` | `packages/primeng/src/panelmenu/panelmenu.ts` (`PanelMenu`, `PanelMenuList`, `PanelMenuSub`), `panelmenu/style/panelmenustyle.ts` |
| Модель (раскрытые ключи, видимые пункты, переходы, поиск) | `prime-panelmenu::state` — своя, на `prime_tieredmenu::{Node, build, item_at}` |
| CSS | `@primeuix/styles/panelmenu`, пресет Aura и блок `/*For PrimeNG*/` — `cargo xtask gen-theme` (`panelmenu` в `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/panelmenu/*-doc.ts` (`basic`, `multiple`, `controlled`, `command`, `template`) |

Спека: `openspec/specs/panelmenu`. Модель пунктов — `prime_menu::MenuItem`: поле `items` задаёт ветки, `expanded` — начальное раскрытие, `command` — действие. Ключ пункта — индексы предков через `_` (`"0"`, `"0_1"`), он же в `PanelMenuRef::is_expanded` и в `id` элементов `li` (`<id меню>_<ключ>`).

```rust
let panel_ref = PanelMenuRef::new();
view! {
    <Button label="Toggle All" on_click=Callback::new(move |_| panel_ref.toggle_all()) />
    <PanelMenu model=items multiple=true panel_ref=panel_ref />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-panelmenu` > `div.p-panelmenu-panel` > `div.p-panelmenu-header[role=button]` (`-header-active`, `p-disabled`, `aria-expanded`, `aria-controls`, `data-p-highlight`) > `-header-content` > `a.p-panelmenu-header-link` (шеврон, значок, подпись) и `div.p-panelmenu-content-container[role=region]` > `-content-wrapper` > `-content` > `ul.p-panelmenu-root-list.p-panelmenu-submenu[role=tree]` > `li.p-panelmenu-item[role=treeitem]` (`aria-level`, `aria-setsize`, `aria-posinset`, `p-focus`, `data-p-focused`) > `-item-content` > `a.p-panelmenu-item-link`; вложенные ветки — контейнер раскрытия с `ul.p-panelmenu-submenu`.
- Одиночный режим: клик по заголовку сворачивает остальные корневые панели (в том числе по листу-заголовку); вложенные ветки независимы. `multiple` снимает ограничение.
- Клавиатура заголовка: ↓ на раскрытом — в дерево, иначе на следующий заголовок (по кругу, отключённые пропускаются); ↑ — на предыдущий (в его дерево, если раскрыт); `Home` `End`; `Enter` `Space` — клик по ссылке заголовка. Клавиатура дерева: ↓ ↑ по видимым пунктам (на краях — на соседний заголовок / свой заголовок), → раскрывает ветку, а на раскрытой идёт вниз, ← сворачивает или идёт к родителю, `Home` `End`, `Enter` `Space`, поиск по началу подписи с буфером 500 мс.
- Корневые списки получают `tabindex="-1"` (как итоговый DOM PrimeNG, где host-привязка перекрывает свойство `tabindex`); ссылки пунктов — `tabindex="0"`, пока раскрыты все предки, иначе `-1`.
- Раскрытие и сворачивание — анимация `p-collapsible-enter-active` / `-leave-active` из `prime_core::collapsible`.

## Отличия

| Отличие | Причина |
|---|---|
| ← на раскрытой ветке всегда сворачивает её | В PrimeNG ветка, раскрытая кликом, остаётся раскрытой из-за флага `expanded`, оставшегося на модели (← убирает её только из пути активных). |
| Вход фокуса в дерево с заголовка заново выбирает пункт (первый или кликнутый), снаружи — последний | В PrimeNG пункт выбирается один раз, при возврате остаётся прежний. |
| Лист не получает контейнера раскрытия и пустого `ul` | В PrimeNG они выводятся и у листьев, но пусты и скрыты. |
| Раскрытие хранится в сигнале набора ключей, а не в полях модели; начальное состояние берётся из `MenuItem::expanded` один раз | `MenuItem` у нас неизменяемая модель. `PanelMenuRef` заменяет `collapseAll` и ручную правку `expanded` из демонстрации `controlled`. |
| Нет `key` пункта: `PanelMenuRef` адресует пункты позицией (`"0_1"`) | У `MenuItem` нет поля `key`. |
| Нет свойств `tabindex` и `transitionOptions` | Первое не влияет на итоговый DOM, второе заменено общей анимацией. |
| Нет `routerLink`, `Tooltip`/`Badge`, `escape = false`, `Ripple`, `Bind`, `pTemplate`; значки — `ViewFn` | Нет маршрутизатора, `innerHTML` и PrimeIcons. |
| Шаблон `item` применяется и к заголовкам (`PanelMenuItemContext::root`), `header_icon` — шеврон заголовка | Как `#item` и `#headericon` PrimeNG. |
| Демонстрации `router` и `accessibility` не перенесены; значки без аналога в `prime-icons` заменены близкими | Нет маршрутизатора и PrimeIcons. |

Попутно `prime_core::collapsible::animate` получил защиту от гонки: запоздавшее завершение прежней анимации (запасной таймер 400 мс) больше не скрывает уже раскрытую область при быстром повторном переключении; затрагивает Panel, Fieldset, Accordion.

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Посимвольное сравнение DOM с PrimeNG (только сверка вручную и скриншот `panelmenu-basic` в `e2e/tests/pages.json`).

## Производительность

См. `benches/` (`panelmenu-1`, `panelmenu-1000`, `case-panelmenu`) и `benches/*budgets.toml`.
