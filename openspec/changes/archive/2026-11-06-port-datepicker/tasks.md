# Tasks

## 1. Основа

- [x] 1.1 `datepicker` в `theme.components`, `cargo xtask gen-theme`; есть `p-datepicker-day-selected-range`
- [x] 1.2 Свериться с `createMonth`, `isSelectable`, `selectDate`, `formatDate`, `parseDate`, навигацией исходника

## 2. Крейт `prime-datepicker`

- [x] 2.1 Модуль `calendar.rs` с табличными тестами
- [x] 2.2 Компонент `DatePicker`, `DatePickerPt`: поле, значки, панель, шапка, сетка дней, месяцы, годы, кнопки панели, SSR-тесты, rustdoc
- [x] 2.3 Выбор (`single`/`multiple`/`range`), ограничения, ввод текста, форматирование
- [x] 2.4 Оверлей (открытие, закрытие, `inline`), навигация и виды, кнопки «Сегодня»/«Очистить»
- [x] 2.5 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/datepicker`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `datepicker.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-datepicker`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/datepicker.md`; `prime-datepicker` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-datepicker`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; после архива — пуш и один прогон CI всей волны 6.
