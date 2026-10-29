# Textarea

Порт `pTextarea` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-textarea::Textarea` | `packages/primeng/src/textarea/textarea.ts`, `textarea/style/textareastyle.ts` |
| CSS | `@primeuix/styles/textarea` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/textarea/*-doc.ts` (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`, `floatlabel`, `iftalabel`) |

Спека: `openspec/specs/textarea`.

## Что совпадает с PrimeNG

- Классы `p-textarea p-component`, `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-textarea-lg p-inputfield-lg`, `p-invalid` в порядке `textareastyle.ts` (`data-p` у директивы нет).
- Вариант: свойство `variant`, иначе `PrimeConfig.input_variant`.
- `auto_resize`: `height = auto`, затем `scrollHeight`; при достижении встроенного `max-height` — `overflow-y: scroll`, иначе `overflow: hidden`; `on_resize` после подгонки.

## Отличия

| Отличие | Причина |
|---|---|
| Компонент `<Textarea />`, а не директива на `<textarea>` | В Leptos нет директив на произвольный элемент. |
| `value: RwSignal<String>` вместо `ngModel`; без `value` поле неконтролируемое, `p-filled` не отслеживается | Как у InputText (`docs/inputtext.md`). |
| Подгонка высоты — по изменению сигнала и по вводу, а не при каждой проверке изменений | В Leptos нет аналога `ngAfterViewChecked`. |
| Начальный текст SSR экранируется вручную, ведущий перевод строки дублируется | Leptos выводит детей `<textarea>` без экранирования (как `<script>`): иначе значение `</textarea><script>` ломало бы разметку, а HTML-парсер отбрасывает первый перевод строки. |
| Нет `KeyFilter` и интеграции с формами; демонстрация `keyfilter` не перенесена | Вне объёма change. |
| Свойство `fluid` — `OptProp<bool>`: без значения берётся из контекста `Fluid` | Как `hasFluid = fluid() ?? !!pcFluid` в PrimeNG; см. `docs/field-wrappers.md`. |

## Производительность

SSR: 10 аллокаций на экземпляр (бюджет 12); вклад в сырой wasm +255 КБ (бюджет 300 КБ) — основная часть это привязки `web-sys` для подгонки высоты и событий.
Шаблон — как у InputText (`docs/inputtext.md`).
