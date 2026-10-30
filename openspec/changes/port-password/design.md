# Design

## Context

Требования — в `specs/password`. Исходники PrimeNG 21.1.10: `password/password.ts`, `password/style/passwordstyle.ts`; демонстрации `apps/showcase/doc/password/*-doc.ts`. Образцы в проекте: `prime-inputtext` (поле и модель значения), `prime-select` (панель внутри корня, `align_overlay(Relative)`, z-index `ZGroup::Overlay`, проверка `is_disposed()` в таймерах), `prime-slider` (слушатели `window`), `prime_core::fluid`.

Наблюдения по исходнику:
- Это `p-password` (компонент); директиву `pPassword` не переносим.
- Оценка: `strongRegex` (`^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.{8,})`) → 3, `mediumRegex` → 2, непустая строка → 1, иначе подсказка (`meter = null`).
- Панель: `onInputFocus` и `onKeyUp` открывают, `onInputBlur` и `Escape` закрывают; `updateUI` вызывается на `keyup`, а значит и при записи значения снаружи (`writeControlValue`).
- Корень получает `position: relative` при `appendTo = self`; панель — `position: absolute`.
- Значок очистки показывается при `value != null` (в PrimeNG пустая строка тоже считается значением); мы показываем при непустой строке.

## Decisions

### D1. Крейт и зависимости
`prime-password` с `Password`, `PasswordStrength { Weak, Medium, Strong }`, `PasswordPt`. Зависит от `prime-core`, `prime-dom`, `prime-inputtext`, `prime-icons` (`eye`, `eyeslash`, `times`), `web-sys`.

### D2. Чистая логика (`state.rs`)
- `strength(value: &str) -> Option<PasswordStrength>` — `None` для пустой строки; средняя: длина ≥ 6 (в символах) и не менее двух видов; сильная: длина ≥ 8 и три вида. Виды: ASCII `a-z`, `A-Z`, `0-9` (как в регулярных выражениях PrimeNG).
- `meter_width(level)` — `33.33%`/`66.66%`/`100%`; `meter_class(level)`.
- `root_classes(...)`, `data_p(...)`.
Табличные тесты, в том числе границы длины и состав.

### D3. Отличие: `strength` вместо регулярных выражений
`regex` не поддерживает lookahead, а тянуть второй движок ради двух строк избыточно. Оценка по умолчанию эквивалентна дефолтным выражениям PrimeNG; для иных правил — `strength: Callback<String, Option<PasswordStrength>>`. Фиксируется в `docs/password.md`.

### D4. Модель значения
Как у `InputText`: необязательный `RwSignal<String>`. Внутри поле — `InputText` со своим `value`, чтобы не дублировать состояние; тип (`password`/`text`) переключается сигналом `unmasked`. Если `InputText` не поддерживает реактивный `type`, поле рисуется самим `Password` с теми же классами (проверить при реализации; решение записать в docs).

### D5. Панель
Рендерится в корне при `Phase`-аналоге: `Hidden → Shown → Leaving` (классы `p-password-enter`/`p-password-leave` из блока `/* For PrimeNG */`, таймер 300 мс с проверкой `is_disposed()` и счётчиком поколений). Открывается по `focus` и `keyup`/`input`, закрывается по `blur` и `Escape`. После показа: z-index `generate(Overlay, base)`, `align_overlay(Relative)`, `min-width` по ширине поля (CSS); при закрытии — `zindex::revert`. На `resize` окна панель закрывается (если не сенсорное устройство, как в PrimeNG) — слушатель только на `wasm32`, снятие в `on_cleanup`. Прокрутка предков не отслеживается (отличие, записать в docs).

### D6. Значки
Очистка, «глаз» и «глаз перечёркнутый» — `TimesIcon`, `EyeIcon`, `EyeSlashIcon` с классами по PrimeNG; шаблоны — `Option<ViewFn>`. Клики обрабатываются на значках; `mousedown` значков не должен отнимать фокус у поля (иначе сработает `blur` и панель закроется раньше клика).

### D7. Фокус
Корень получает `p-inputwrapper-focus` по `focus`/`blur` поля. Автофокус (`autofocus`) в первой версии не переносится.

### D8. CSS
`password` в `theme.components` и `primeng_extra`; `cargo xtask gen-theme`, проверка, что CSS остальных компонентов не изменился. Селекторы `p-password.ng-invalid` в блоке безвредны.

### D9. Тесты
SSR: разметка по умолчанию, `feedback = false`, `toggle_mask`, `show_clear`, `data-p`. Браузерные `tests/web.rs`: ввод и уровни, открытие и закрытие панели, `Escape`, переключение типа, очистка, фокус остаётся в поле при клике по значку, z-index и его возврат. e2e: настоящий ввод, видимость панели, клавиша `Escape`, показ пароля. Крейт добавляется в три места `ci.yml`.

### D10. Бенчмарки
Сценарии `password-1` и `-1000` (SSR, панель закрыта), `case-password` для wasm; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет.

## Risks / Trade-offs

- [Нет `regex` с lookahead] → `strength` и эквивалентные правила по умолчанию (D3).
- [Потеря фокуса при клике по значкам] → `prevent_default` на `mousedown` (D6), покрыто браузерным тестом.
- [Предел 26 атрибутов tachys на элементе] → лишние атрибуты через `Vec<AnyAttribute>`.
- [Сенсорный ввод и экранные считыватели] → не проверялось, честно записано в docs.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
