# SelectButton

Порт `p-selectbutton` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-selectbutton::SelectButton` | `packages/primeng/src/selectbutton/selectbutton.ts`, `selectbutton/style/selectbuttonstyle.ts` |
| CSS | `@primeuix/styles/selectbutton` и пресет Aura (`cargo xtask gen-theme`); стили кнопок — CSS `togglebutton` |
| Демонстрации | `apps/showcase/doc/selectbutton/*-doc.ts` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`) |

Спека: `openspec/specs/selectbutton`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-selectbutton[role=group]`, внутри по `ToggleButton` на вариант с `on_label = off_label = label` (в PrimeNG — `p-togglebutton`).
- Классы корня и их порядок: `p-selectbutton p-component`, `p-invalid`, `p-selectbutton-fluid`; `data-p` — `invalid`.
- `size`, `fluid` и `style_class` передаются каждой кнопке (`style_class` — не корню); `invalid` кнопкам не передаётся.
- Выбор как в `onOptionSelect`: одиночный — повторный клик сбрасывает значение, `allow_empty = false` это запрещает; множественный — значение добавляется в конец списка или удаляется, `allow_empty = false` не даёт удалить последнее (`getAllowEmpty`).
- События: `on_change` (значение и исходное событие), затем `on_option_click` (вариант, индекс, исходное событие).
- `item_template` получает вариант и индекс и заменяет содержимое кнопки (`content_template` вложенного `ToggleButton`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-selectbutton>` | Хост в Angular — деталь фреймворка. |
| Варианты — `SelectButtonOption { label, value, disabled }`, значения — строки; нет `optionLabel`, `optionValue`, `optionDisabled`, `dataKey` | Поля произвольного объекта в Rust заменены типизированным вариантом; обобщение до `T` возможно отдельным change. |
| Одиночный режим — `value: RwSignal<Option<String>>`, множественный (`multiple`) — `values: RwSignal<Vec<String>>`; без сигнала внутреннее состояние (`default_value`, `default_values`) | Двусторонняя привязка без `ngModel`; два типа читаются проще, чем список из одного элемента. |
| Нет `unselectable`, `autofocus`, `tabindex`, интеграции с формами, блока `/* For PrimeNG */` | `unselectable` заменён `allow_empty`; остальное вне объёма change или не используется шаблоном PrimeNG. |
| Сигнал каждой кнопки выравнивается со значением явно (в браузере — ещё и `Effect`) | `ToggleButton` переключает свой `checked` до вызова `on_change`; без выравнивания отклонённый клик оставил бы кнопку включённой. |
| В демонстрации `template` метка в `<strong>` вместо иконок `pi-align-*` | В `prime-icons` нет таких иконок. |

## Производительность

SSR: 77 аллокаций на экземпляр с двумя вариантами (бюджет 86); вклад в сырой wasm +282 КБ (бюджет 330 КБ) — вместе с `ToggleButton`.
