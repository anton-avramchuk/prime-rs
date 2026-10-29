# Rating

Порт `p-rating` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-rating::Rating` | `packages/primeng/src/rating/rating.ts`, `rating/style/ratingstyle.ts` |
| CSS | `@primeuix/styles/rating` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/rating/*-doc.ts` (`basic`, `disabled`, `numberofstars`, `readonly`, `template`) |

Спека: `openspec/specs/rating`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-rating`, на каждую звезду `div.p-rating-option` со скрытой радиокнопкой (`span.p-hidden-accessible > input[type=radio]`) и иконкой `p-rating-icon p-rating-on-icon` / `p-rating-off-icon`.
- Классы корня, опции и иконок и их порядок: `p-rating`, `p-readonly`, `p-disabled`; `p-rating-option`, `p-rating-option-active`, `p-focus-visible`; `p-invalid` у иконок. `data-p` корня — `readonly disabled`.
- Группа нативных радиокнопок с общим `name`: стрелки клавиатуры работают без собственного кода. Имя по умолчанию — `pn_id_<n>_name`, номер из контекста гидратации (общий хелпер `prime_core::ids::next_unique`, вынесенный из `collapsible`).
- Клик по звезде выбирает её, повторный клик по текущему значению сбрасывает его; после клика фокус переходит на радиокнопку звезды, и он не считается видимым.
- `aria-label` звезды из локали: `aria.star` для первой, `aria.stars` с подстановкой `{star}` для остальных.
- Иконки по умолчанию — `StarFillIcon` и `StarIcon`; шаблоны `on_icon_template` и `off_icon_template` получают номер звезды и класс иконки (`{ $implicit, class }`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-rating>` | Хост в Angular — деталь фреймворка. |
| `value: RwSignal<Option<u32>>` вместо `ngModel`; без него внутреннее состояние (`default_value`) | Двусторонняя привязка без форм-инфраструктуры. |
| Стрелка клавиатуры выбирает соседнюю звезду, а не сбрасывает значение | В PrimeNG `onOptionSelect` сбрасывает значение, если индекс фокуса равен выбранному, а `focus` приходит раньше `change`; это выглядит побочным эффектом, у нас `change` всегда ставит значение звезды. Сбросить значение с клавиатуры нельзя, только кликом. |
| Клик по самой радиокнопке обрабатывает `change`, а не `click` | Стрелки и программный `click()` шлют оба события, значение выставилось бы дважды. |
| Атрибут `value` радиокнопки — номер звезды | В PrimeNG `[attr.value]="modelValue()"` перекрывает `[value]`, и у всех кнопок `value` равен текущему значению, что ломает нативную отправку формы (так же сделано у RadioButton). |
| Видимый фокус — `:focus-visible` радиокнопки | `sourceCapabilities.firesTouchEvents` нет в `web-sys` и Firefox. |
| `on_icon`/`off_icon` — `ViewFn` в `span` с классами иконки вместо `iconOnClass`/`iconOffClass`/`iconOnStyle`/`iconOffStyle` | В `prime-icons` нет шрифта; стили задаёт `style`/`pt`. |
| Нет `autofocus`, интеграции с Angular Forms, блока `/* For PrimeNG */` | Вне объёма change. |
| Демонстрация `withoutcancel` не перенесена | В 21.1.10 кнопки отмены у Rating нет, демонстрация дублирует `basic`. |
| В демонстрации `template` символы `♥` и `♡` вместо внешних картинок | Без зависимости от CDN. |

Список звёзд строится замыканием над `stars` и не читает значение: иначе выбор пересоздавал бы звёзды и ронял фокус; всё, что зависит от значения, — отдельные замыкания внутри звезды.

## Производительность

SSR: 159 аллокаций на экземпляр с пятью звёздами (бюджет 175), HTML ≈ 9 КБ из-за контуров SVG в каждой иконке; вклад в сырой wasm +258 КБ (бюджет 300 КБ).
