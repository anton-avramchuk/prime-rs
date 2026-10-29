# Design

## Context

Требования — в `specs/rating`. Исходники PrimeNG 21.1.10: `packages/primeng/src/rating/rating.ts`, `rating/style/ratingstyle.ts`, демонстрации `apps/showcase/doc/rating`. Образцы — `crates/prime-radiobutton` (нативный `<input>` в группе, `docs/radiobutton.md`) и `crates/prime-selectbutton`/`prime-togglebutton` (реактивные замыкания `rx`, правила работы с отслеживанием). В `prime-icons` есть `StarIcon` и `StarFillIcon`, в `prime-core` — переводы `aria.star` и `aria.stars`, а `p-hidden-accessible` уже в базовом CSS темы.

Наблюдения по исходнику, повлиявшие на решения:
- У Rating нет кнопки отмены (в демонстрации `withoutcancel` ничего не отличается от `basic`); сброс — повторный выбор текущего значения.
- Событие `onOptionSelect` в PrimeNG сбрасывает значение, если `focusedOptionIndex === value || value === this.value`; индекс фокуса выставляет обработчик `focus`, который срабатывает раньше `change` при переходе стрелкой, поэтому в PrimeNG стрелка на «сфокусированной» звезде, по-видимому, сбрасывает значение. Это выглядит как побочный эффект реализации, а не задуманное поведение, и не переносится (см. D2).
- `[attr.value]="modelValue()"` у `<input>` перекрывает `[value]="star + 1"`, так что у всех радиокнопок `value` равен текущему значению. Это ломает нативную отправку формы и не переносится (как у RadioButton).

## Decisions

### D1. Значение
`value: Option<RwSignal<Option<u32>>>`; без него создаётся внутренний сигнал с `default_value` (`OptProp<u32>`, читается один раз). Номера звёзд — с 1; значение `0` не используется (в PrimeNG `value || null`).

### D2. Выбор
Чистая функция `next_value(current, star) -> Option<u32>` (`state.rs`): равное значение сбрасывается в `None`, иное — `Some(star)`. Клик по `div.p-rating-option` выбирает так же, как в PrimeNG: при `readonly`/`disabled` ничего не происходит; иначе значение меняется, вызывается `on_rate`, затем фокус переводится на радиокнопку звезды (`onOptionClick`). Событие `change` радиокнопки (стрелки, программный `click()`) устанавливает значение звезды **без** переключения в `None`: нативный `change` приходит только при смене выбранной радиокнопки, то есть значение там по определению новое. Сбросить значение с клавиатуры нельзя (как и у нативной группы), только кликом.

### D3. Разметка и реактивность
Корень — `<div>`; звёзды строятся один раз замыканием над `stars` (число звёзд — единственное, что оно читает), а всё зависящее от значения (классы опции, `checked`, иконка) — отдельные замыкания внутри звезды. Замыкание списка не читает `value`: чтение сигнала значения в нём пересоздавало бы звёзды и роняло фокус (урок `port-selectbutton`; внутри списка только `*_untracked`). Порядок классов опции: `p-rating-option`, `p-rating-option-active`, `p-focus-visible`, затем `pt.option`; иконки: `p-rating-icon p-rating-on-icon|p-rating-off-icon`, `p-invalid`, `pt`. `data-p` корня: `readonly disabled`; классы корня: `p-rating`, `p-readonly`, `p-disabled`, `style_class`, `pt.root`. Чистая логика (классы, `data-p`, `next_value`) — в `state.rs` с табличным тестом.

### D4. Фокус и `p-focus-visible`
В PrimeNG видимый фокус определяется `sourceCapabilities.firesTouchEvents`, которого нет в `web-sys` и в Firefox. Вместо этого обработчик `focus` проверяет `input.matches(":focus-visible")` и хранит номер звезды в `visible_focus: RwSignal<Option<u32>>`; `blur` сбрасывает его. Фокус после клика мышью (программный) браузер не считает видимым, поэтому `p-focus-visible` у такой звезды не появляется — как в PrimeNG. `on_focus`/`on_blur` вызываются при `focus`/`blur` радиокнопки (`on_focus` — только если не `readonly` и не `disabled`).

### D5. Имя группы
`name: OptProp<String>`, иначе `pn_id_<n>_name`, где `<n>` — номер из общего контекста гидратации (одинаков на сервере и в браузере). Логика уже есть в `prime-core::collapsible` (`next_unique`); выносим её в `prime-core::ids` (`pub fn next_unique`) и используем в `collapsible` и `Rating`. Поведение `Panel`/`Fieldset`/`Accordion` не меняется (их тесты и снапшоты остаются как есть).

### D6. Иконки и слоты
По умолчанию `StarFillIcon`/`StarIcon` с классами иконки и `pt`. `on_icon`/`off_icon: Option<ViewFn>` заменяют звезду; `on_icon_template`/`off_icon_template: Callback<RatingIcon, AnyView>` (`RatingIcon { star: u32, class: String }`, как контекст `{ $implicit, class }` PrimeNG) заменяют иконку целиком, приоритет: шаблон, затем `on_icon`/`off_icon`, затем звезда. Классовые `icon_on_class` и им подобные не переносятся: шрифта иконок нет.

### D7. Подписи
`aria-label` звезды: `aria.star` для первой, иначе `aria.stars` с подстановкой `{star}`; строки берутся из `use_prime_config().locale` один раз при создании компонента (как у других компонентов с локалью).

### D8. CSS
`rating` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`. Блок `/* For PrimeNG */` содержит только правило `p-rating.ng-invalid.ng-dirty` Angular Forms, поэтому `primeng_extra` не расширяется. Иконки красятся и обводятся через `stroke` темы; для `p-invalid` берётся `rating.invalid.icon.color`.

### D9. Демонстрации
`basic`, `disabled`, `numberofstars`, `readonly`, `template`. Демонстрации с формами и `withoutcancel` пропускаются. В `template` вместо внешних картинок CDN пользовательские иконки — символы `♥` и `♡` в `span` (через `on_icon_template`/`off_icon_template`). У `basic` — `echo` значения для e2e.

### D10. Тесты
SSR: структура, число звёзд, классы и `data-p`, `checked`, подписи, атрибуты `name`/`required`/`readonly`/`disabled`, иконки и слоты, `invalid`, `pt`, паритет статических и реактивных свойств. Браузерные (`tests/web.rs`): выбор и сброс кликом, `readonly`/`disabled`, один `on_rate` на действие, переключение радиокнопки (`input.click()`) без сброса, внешний сигнал, реактивное `stars`, `p-focus-visible` и фокус, сохранение элементов звёзд и фокуса после выбора. Крейт с `tests/web.rs` добавляется в браузерный job `ci.yml` (3 места). e2e дополнительно проверяет стрелки клавиатуры.

## Risks / Trade-offs

- [Пять звёзд по четыре узла и по нескольку замыканий дороже одной кнопки] → бюджет по измерению; при превышении — свернуть замыкания звезды (общий `rx` на звезду).
- [Расхождение с PrimeNG при стрелках (D2) и в определении видимого фокуса (D4)] → фиксируется в `docs/rating.md`; спека описывает предсказуемое поведение.
- [Вынос `next_unique` затрагивает `collapsible`] → чисто механический перенос, существующие тесты `prime-core` и компонентов (Panel, Fieldset, Accordion) должны остаться зелёными без правок.

## Migration Plan

Миграции нет: новый крейт и внутренний хелпер. Откат — `git revert`.

## Open Questions

Нет.
