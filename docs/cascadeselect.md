# CascadeSelect

Порт `p-cascadeselect` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-cascadeselect::CascadeSelect`, `CascadeSelectPt`, `CascadeOption`, `CascadeOptionContext` | `packages/primeng/src/cascadeselect/cascadeselect.ts` (`CascadeSelect` и `CascadeSelectSub`), `cascadeselect/style/cascadeselectstyle.ts` |
| Модель (путь, фокус, клавиши, поиск) | `prime-cascadeselect::state` — чистые функции с табличными тестами |
| CSS | `@primeuix/styles/cascadeselect`, пресет Aura и блок `/* For PrimeNG */` — `cargo xtask gen-theme` (`cascadeselect` в `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/cascadeselect/*-doc.ts` (`basic`, `clearicon`, `disabled`, `invalid`, `loading`, `template`) |

Спека: `openspec/specs/cascadeselect`. Варианты — `CascadeOption` (лист со `value` либо группа с `children`), значение — `RwSignal<Option<String>>` со значением выбранного листа (если `value` не задан, берётся подпись). Ключи узлов — индексы предков через `_` (`"0_1_2"`), они же в `id` элементов `li` (`<id поля>_<ключ>`).

```rust
let value = RwSignal::new(None::<String>);
view! { <CascadeSelect options=countries value=value placeholder="Select a City" show_clear=true /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-cascadeselect.p-component.p-inputwrapper` (состояния: `-clearable|mobile|open|fluid|sm|lg`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`) > скрытое поле `input[role=combobox][readonly]` > `p-cascadeselect-label` > значок очистки > `p-cascadeselect-dropdown` > панель `p-cascadeselect-overlay` > `p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` > `li.p-cascadeselect-option[role=treeitem]` (`-option-group|active|selected`, `p-focus`, `p-disabled`, `aria-*`) > `div.p-cascadeselect-option-content` > текст и значок группы; вложенные списки `ul.p-cascadeselect-option-list[role=group]`.
- Путь активных групп (`onOptionChange`/`onOptionClick`): клик по группе раскрывает её и закрывает соседние, повторный — сворачивает; наведение работает по правилам `onOptionMouseEnter` (до первого раскрытия — корневые группы, затем любые; при выбранном значении — сразу); на узком окне наведение путь не меняет; при открытии раскрывается путь к выбранному значению.
- Клавиатура по `onArrow*Key`: ↓ ↑ по списку уровня, фокус на группу раскрывает её, → входит в раскрытую группу (раскрывает закрытую), ← закрывает группу уровня и возвращает фокус на родителя, `Alt+↑` закрывает, `Home` `End` `Enter` `Space` `Escape` `Tab` `Backspace`, поиск по началу подписи с буфером на 500 мс.

## Отличия

| Отличие | Причина |
|---|---|
| Варианты — `CascadeOption`, значение — строка | В PrimeNG произвольные объекты с `optionLabel`/`optionValue`/`optionGroupLabel`/`optionGroupChildren`/`dataKey`. |
| Перенос фокуса стрелкой на лист закрывает открытые группы его уровня; клик на уже раскрытую группу стрелкой её не сворачивает | В PrimeNG раскрытые группы остаются открытыми, а перенос фокуса вызывает `onOptionClick`, который свернул бы раскрытую группу. |
| Наведение на лист корня до первого клика только переносит фокус | В PrimeNG такое наведение выбирало бы лист. |
| Нет формы Angular, `appendTo`, `onBeforeShow`/`onBeforeHide`, `select_on_focus`, отзеркаливания вложенного списка у края окна (`position()`), шаблонов `header`/`footer`/иконок и сообщений для скринридеров | Вне объёма порта; положение вложенных списков задаёт CSS темы. |
| Подпись пустого поля без подсказки — слово `p-emptylabel` под `p-cascadeselect-label-empty` (скрыта темой) | Так же в PrimeNG. |
| `value_view` (замена метки) и `option` (содержимое варианта) вместо `pTemplate` | Нет `pTemplate`. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Вложенные списки у правого края окна (выходят за окно, как в PrimeNG без `position()`).

## Производительность

См. `benches/` (`cascadeselect-1`, `cascadeselect-1000`, `case-cascadeselect`) и `benches/*budgets.toml`.
