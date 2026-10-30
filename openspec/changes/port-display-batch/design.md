# Design

## Context

Требования — в `specs/{overlaybadge,timeline,metergroup}`. Исходники PrimeNG 21.1.10: `overlaybadge/overlaybadge.ts`, `timeline/timeline.ts`, `metergroup/metergroup.ts` и их `style/*style.ts`. Образцы в проекте — `crates/prime-badge` (вложенный компонент, `state.rs` с чистой логикой классов и `data_p`), `crates/prime-avatargroup` (контейнер со слотом), `prime-accordion` (шаблоны как `Callback<_, AnyView>`), `prime_core::Pt`/`OptProp`/`Prop`/`all_static!`.

Наблюдения по исходнику:
- `OverlayBadge` — `div.p-overlaybadge` со слотом и `p-badge`; `styleClass`/`style` уходят в бейдж, а не в корень; всё положение — в CSS.
- `Timeline` — `*ngFor` по `value`, три части на событие плюс маркер и соединитель (у последнего события соединителя нет); `align` и `layout` — строки, попадающие в классы и `data-p`.
- `MeterGroup` — корень с ARIA `meter`, отрезки считают процент через `percent()`: `round(clamp((v - min) / (max - min) * 100, 0, 100))`, при `min == max` — 100; подписи вынесены в `MeterGroupLabel`. Подписи выводятся один раз: до отрезков при `labelPosition = start`, после — при `end` (в исходнике два блока с условием по позиции).
- Блоков `/* For PrimeNG */` у этих трёх нет — `primeng_extra` не меняется.

## Decisions

### D1. Крейты и зависимости
`prime-overlaybadge` (зависит от `prime-core`, `prime-badge`), `prime-timeline`, `prime-metergroup` (только `prime-core`). Каждый с `ssr`/`hydrate`, rustdoc с примером, SSR-тестами; публичные `…Pt { … }` со структурой по частям PrimeNG: `OverlayBadgePt { root, pc_badge: BadgePt }`, `TimelinePt { root, event, event_opposite, event_separator, event_marker, event_connector, event_content }`, `MeterGroupPt { root, meters, meter, label_list, label, label_icon, label_marker, label_text }`.

### D2. OverlayBadge
Простой контейнер: `children: Children` и `Badge` с пробросом свойств. Свойства — `OptProp`/`Prop`, как у `Badge`; чтобы не дублировать типы, перереэкспортируются `BadgeSeverity`, `BadgeSize`. Статической ветви нет: корень — один элемент, бейдж сам решает, реактивен ли он.

### D3. Timeline
Обобщённый `Timeline<T>`: `value: Prop<Vec<T>>` (сигнал пересоздаёт список), `align: Prop<TimelineAlign>`, `layout: Prop<TimelineLayout>`; шаблоны `content`, `opposite`, `marker` — `Option<Callback<T, AnyView>>` (`T: Clone + Send + Sync + 'static`). События выводятся `For`-подобным обходом по индексу, соединитель — по `index + 1 < len`. Чистые функции `root_classes`, `data_p` в `state.rs` с табличным тестом. Классы `align` — только пять допустимых значений (тип-перечисление вместо произвольной строки PrimeNG; отличие фиксируется в документации).

### D4. MeterGroup
`MeterItem { label: String, color: String, value: f64, icon: Option<ViewFn> }` (`Clone`). Чистые функции `percent(value, min, max) -> u32`, `total_percent`, `cumulative` — в `state.rs`, покрываются табличными тестами (границы, `min == max`, отрицательные, выше `max`). Свойства `min`/`max` — `Prop<f64>`, `orientation`, `label_position`, `label_orientation` — `Prop` перечислений. Шаблоны `label`, `meter`, `start`, `end`, `icon` — `Callback` с контекстом `MeterGroupContext { items, total_percent, percentages }` (для `meter` и `icon` — элемент и индекс). Подписи — отдельная внутренняя функция разметки, вызываемая в одном из двух мест по `label_position`. Ширина/высота — через `style` с `width`/`height` в процентах и `background-color`; при `match` на ориентации второе измерение не выводится (в PrimeNG оно `false`, то есть тоже не выводится).

### D5. CSS
`overlaybadge`, `timeline`, `metergroup` добавляются в `theme.components`; `cargo xtask gen-theme`, затем проверка, что CSS остальных компонентов не изменился.

### D6. Демонстрации
`/overlaybadge` (`basic`, `avatar`, `button`), `/timeline` (`basic`, `alignment`, `opposite`, `horizontal`, `customized` — без иконок, которых нет в `prime-icons`), `/metergroup` (`basic`, `multiple`, `icon`, `vertical`, `label`, `min-max`). Недостающие иконки заменяются имеющимися (`CheckIcon`, `SearchIcon`, `TimesIcon`, `SpinnerIcon`); отличие — в `docs/display-batch.md`. Заголовок демонстрации не должен совпадать с id элементов самого компонента.

### D7. Тесты
SSR на каждый сценарий спецификаций; для `percent` — табличные тесты чистой логики; шаблоны — тесты с `Owner::new().with(...)`. Браузерных тестов нет: компоненты статичны; e2e проверяют вычисленные стили (угол бейджа, ширину отрезков, направление линии таймлайна).

### D8. Бенчмарки
Сценарии `overlaybadge`, `timeline`, `metergroup` (`-1`, `-1000`) и `case-*` для wasm; сначала `cargo bench -p render-bench --bench alloc`, затем бюджеты.

## Risks / Trade-offs

- [`Timeline<T>` обобщённый — рост кода мономорфизации] → в демонстрациях один тип события; размер в wasm проверяется бюджетом.
- [Пересборка списка при смене `value`] → допустимо для статичных компонентов отображения; сигнал `value` читается только в замыкании списка.
- [`align` ограничен пятью значениями] → PrimeNG принимает любую строку, но CSS знает только эти пять.
- [Иконки в `MeterItem`: нет шрифтовых классов] → `ViewFn` вместо имени класса.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет. `Breadcrumb` и `Steps` вынесены в следующую партию вместе с моделью `MenuItem`.
