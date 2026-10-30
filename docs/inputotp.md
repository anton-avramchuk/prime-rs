# InputOtp

Порт `p-inputotp` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-inputotp::InputOtp` | `packages/primeng/src/inputotp/inputotp.ts`, `inputotp/style/inputotpstyle.ts` |
| CSS | `@primeuix/styles/inputotp` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/inputotp/*-doc.ts` (`basic`, `mask`, `integeronly`, `sizes`, `template`) |

Спека: `openspec/specs/inputotp`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-inputotp.p-component`, в нём `length` (по умолчанию 4) полей `input.p-inputtext.p-component.p-inputotp-input`; классы `p-inputtext-sm|lg`, `p-invalid`, `p-variant-filled` и `data-p` берутся из `prime-inputtext` (`write_classes`, `data_p`).
- `maxlength`: у первой ячейки `length` (она принимает вставку и автозаполнение), у остальных `1`; `inputmode="numeric"` при `integer_only`, `type="password"` при `mask`.
- Логика клавиш (`key_decision`): `ArrowLeft`/`ArrowRight` — соседняя ячейка, `ArrowUp`/`ArrowDown` отменяются, `Backspace` в пустой ячейке — назад; при `integer_only` не цифры отменяются; при заполненных `length` символах без выделения набор отменяется (кроме `Delete`); комбинации с `Alt`/`Ctrl`/`Meta` не обрабатываются.
- Автопереход по `inputType` (`input_move`): `deleteContentBackward` — назад, `insertText` и `deleteContentForward` — вперёд.
- При фокусе содержимое ячейки выделяется. Вставка раскладывает до `length` символов с первой ячейки; несколько символов в первой ячейке (автозаполнение) обрабатываются как вставка.
- Шаблон `input` получает токен, индекс и обработчики событий (`InputOtpEvents`); фокус между ячейками переводится по соседним `<input>`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-inputotp>` | Хост в Angular — деталь фреймворка. |
| `value: RwSignal<String>` вместо `ngModel`; без него токены хранятся внутри, результат приходит через `on_change` | Двусторонняя привязка без форм-инфраструктуры. |
| Пустые ячейки посреди кода сохраняются в локальных токенах, но в `value` попадает склейка без них (`1_3` → `13`) | Так же склеивает `tokens.join('')`; но при внешнем изменении `value` ячейки перекладываются заново. |
| Вставка обрезается строго до `length`; при `integer_only` нецифровая вставка игнорируется целиком | PrimeNG берёт `length + 1` символов, но потом обрезает значением — итог тот же. |
| Фокус между ячейками ищется среди `<input>`-соседей, посторонние элементы шаблона (разделители) пропускаются | PrimeNG ходит по `nextElementSibling` и теряет фокус на разделителе. |
| Управляющие клавиши (`Tab`, `Enter`, `Home`…) не ограничиваются при `integer_only` и заполненном коде | В PrimeNG ветка `default` отменяет и их. |
| Нет `autofocus` через директиву, интеграции с Angular Forms, блока `/* For PrimeNG */`; демонстрации `reactiveforms`, `templatedrivenforms`, `sample` не перенесены | Вне объёма change; формы — Angular-специфика. |
| В `template` вместо значка `pi-minus` — тире | В `prime-icons` нет этой иконки. |

Список ячеек строится замыканием над `length` и не читает токены: иначе ввод пересоздавал бы поля и ронял фокус.

## Что не проверялось

Автозаполнение из SMS, экранные клавиатуры и ввод через IME различаются в браузерах и на устройствах; проверены клавиши, вставка (`ClipboardEvent` с `DataTransfer`) и ввод `InputEvent` в Chromium. Поведение на реальных мобильных устройствах не проверялось.

## Производительность

SSR: 209 аллокаций на экземпляр с шестью ячейками (бюджет 215); вклад в сырой wasm +280 КБ (бюджет 283 КБ).
