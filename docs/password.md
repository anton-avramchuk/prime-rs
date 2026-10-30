# Password

Порт `p-password` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-password::Password`, `PasswordStrength`, `PasswordPt` | `password/password.ts`, `password/style/passwordstyle.ts` |
| CSS | `@primeuix/styles/password` и блок `/* For PrimeNG */` (анимации, `min-width`) — `cargo xtask gen-theme` |
| Демонстрации | `apps/showcase/doc/password/*-doc.ts` |

Спека: `openspec/specs/password`.

## Что совпадает с PrimeNG

- Разметка и классы: `span.p-password.p-component.p-inputwrapper` (`p-inputwrapper-filled`, `-focus`, `p-variant-filled`, `p-password-fluid`) > поле `InputText` с классом `p-password-input`, значки `p-password-clear-icon`, `p-password-unmask-icon` / `p-password-mask-icon`, панель `p-password-overlay` > `p-password-content` > `p-password-meter` > `p-password-meter-label` (с `p-password-meter-weak|medium|strong` и шириной 33.33 / 66.66 / 100 %) и `p-password-meter-text`.
- Оценка по умолчанию равна регулярным выражениям `mediumRegex` и `strongRegex` PrimeNG: средняя — от 6 символов и два вида из строчных, прописных латинских и цифр; сильная — от 8 символов и все три; иначе непустая строка — слабая.
- Панель открывается по фокусу и вводу, закрывается по `Escape` и потере фокуса и на `resize` окна; выравнивается под полем, z-index из группы `Overlay`; появление и исчезновение — классами `p-password-enter` / `p-password-leave` (300 мс).
- Подписи берутся из локали (`passwordPrompt`, `weak`, `medium`, `strong`), если не заданы свойствами.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<span>` без хоста `<p-password>`; директива `pPassword` не переносится | Хост в Angular — деталь фреймворка; для поля без обёртки есть `InputText`. |
| `strength: Callback<String, Option<PasswordStrength>>` вместо `mediumRegex` / `strongRegex` | `regex` не поддерживает lookahead, нужный выражениям по умолчанию; встроенная оценка им эквивалентна. |
| Значок очистки показывается при непустой строке | В PrimeNG — при любом значении, кроме `null`, включая пустую строку. |
| Только `appendTo = self`; нет `overlayOptions`, `motionOptions`, `showTransitionOptions` / `hideTransitionOptions` | Привязаны к Angular `Overlay`; панель всегда внутри корня. |
| Нет `tabindex`, `aria-labelledby`, `autofocus` поля | Их нет у `InputText`. |
| Шаблоны `header`, `footer`, `content`, `show_icon`, `hide_icon`, `clear_icon` — `ViewFn`, без контекста `{ class }` | Вместо `ng-template`. |
| Прокрутка предков не закрывает панель | В PrimeNG закрывает `ConnectedOverlayScrollHandler`; у нас только `resize` окна. |
| Нет привязок Angular Forms, `onClear` — `on_clear` | Модель значения — `RwSignal<String>`. |
| Демонстрации `reactiveforms`, `templatedrivenforms` не перенесены; в `sizes` нет `type="text"` | Привязаны к Angular Forms. |

## Стоимость

103 аллокации на экземпляр при SSR (с `toggle_mask`, панель закрыта); вклад в wasm ≈ 483 КБ (бюджет 540 КБ в `benches/budgets.toml`; в основном общий `InputText`, иконки и `align_overlay`).

## Не проверено

Касание и экранные считыватели: проверены разметка, уровни, клавиша `Escape`, показ пароля, очистка и положение панели в Chrome (e2e и браузерные тесты). `aria-live` у подсказки не добавлялся — в PrimeNG его тоже нет.
