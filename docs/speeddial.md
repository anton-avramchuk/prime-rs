# SpeedDial

Порт `p-speeddial` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-speeddial::SpeedDial`, `SpeedDialItem`, `SpeedDialCommand`, `SpeedDialItemContext`, `SpeedDialType`, `SpeedDialDirection`, `SpeedDialPt` | `packages/primeng/src/speeddial/speeddial.ts`, `speeddial/style/speeddialstyle.ts` |
| CSS | `@primeuix/styles/speeddial` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет); кнопки берут стили `button` |
| Демонстрации | `apps/showcase/doc/speeddial/*-doc.ts` (`linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`) |

Спека: `openspec/specs/speeddial`.

```rust
view! {
    <div style="position: relative; height: 300px">
        <SpeedDial model=items direction=SpeedDialDirection::Up style="position: absolute; right: 0; bottom: 0" />
    </div>
}
```

Корень в теме не позиционируется: контейнеру нужен `position: relative`, а самому `SpeedDial` — `position: absolute` через `style`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-speeddial.p-component.p-speeddial-<type>` (`p-speeddial-direction-<direction>` кроме круга, `p-speeddial-open`, `p-disabled`) > кнопка `p-speeddial-button` (`p-speeddial-rotate`, `aria-expanded`, `aria-haspopup`, `aria-controls`) и `ul.p-speeddial-list[role=menu]` > `li.p-speeddial-item[role=menuitem]` (`p-hidden`, `p-focus`, `data-p-active`) > кнопка `p-speeddial-action`; маска `p-speeddial-mask p-overlay-mask` при `mask` и показе.
- Раскладка: линейная (`flex-direction` по направлению), круг, полукруг и четверть круга с теми же формулами (радиус по умолчанию `20 × число пунктов`, смещения `--item-diff-x/y` по разнице размеров кнопки и пункта), `transition-delay` пунктов растёт к концу при показе и к началу при скрытии.
- Клик по кнопке переключает показ (`on_click`, `on_visible_change`, `on_show`, `on_hide`), клик по пункту вызывает `command` и скрывает список, клик вне и `Escape` скрывают его (`Escape` возвращает фокус на кнопку).
- Клавиатура: на кнопке ↓/← открывают список и идут к первому пункту, ↑/→ — к последнему; в списке стрелки идут по направлению раскладки (та же таблица, что в PrimeNG), `Home`/`End`, `Enter`/`Space` активируют пункт под фокусом, фокус возвращается на кнопку; `aria-activedescendant` указывает на пункт под фокусом.
- `tooltip_options` задаёт параметры подсказок пунктов (текст — подпись пункта); `button_content`, `item_content`, `icon_content`, `show_icon`/`hide_icon`, `rotate_animation`.

## Отличия

| Отличие | Причина |
|---|---|
| Отключённые и скрытые (`visible = false`) пункты пропускаются при навигации стрелками | В PrimeNG фильтр ищет внутри пункта `a`, которого в стандартной разметке нет, поэтому отключённые пункты фокус не пропускает. Порт делает то, что задумано. |
| `Enter`/`Space` на самой кнопке пункта обрабатывает её нативный клик; список реагирует на них, только когда фокус на нём | В PrimeNG у кнопки пункта и `(click)`, и `(keydown.enter)`, поэтому `Enter` вызывал команду дважды. |
| Нет флага `isItemClicked`: клик внутри компонента не считается «вне» за счёт проверки вложенности | Слушатель окна вешается на время показа в следующем такте, так что клик, открывший список, его не ловит. |
| `SpeedDialItem` — свой тип (без `routerLink`, `url`, `tooltipOptions` пункта, `icon` как класс) | Нет роутера и PrimeIcons; маршрутизацию заменяет `command`. |
| Вместо `buttonProps` — только `button_severity` (кнопка всегда круглая) | Единственное свойство, которое используют демонстрации. |
| Нет `Ripple`, `Bind`, `pTemplate`, `unstyled` | Не переносятся в проекте. |
| Значки PrimeIcons в демонстрациях заменены значками `prime-icons`; `pi-external-link` — стрелкой вправо | Нет PrimeIcons. |
| `visible` — двусторонний `RwSignal<bool>` вместо пары `visible`/`visibleChange` | Идиома Leptos. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Диагональные направления у линейного и полукруглого типов: как в PrimeNG, для них раскладка не задана.

## Производительность

См. `benches/` (`speeddial-1`, `speeddial-1000`, `case-speeddial`) и `benches/*budgets.toml`.
