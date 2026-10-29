# Proposal

## Why

Rating — выбор оценки звёздами, часто нужный в формах и карточках; по сложности это компонент среднего размера, который строится на группе нативных радиокнопок (как RadioButton) без оверлеев и общей логики. Следующий в очереди порт после SelectButton.

## What Changes

- Новый крейт `prime-rating` с компонентом `Rating`: двусторонний `value` (`RwSignal<Option<u32>>`) или внутреннее состояние с `default_value`, `stars` (по умолчанию 5), `readonly`, `disabled`, `invalid`, `name`, `required`, `style`, `style_class`, `on_icon`/`off_icon` (`ViewFn`) для замены звёзд по умолчанию, слоты `on_icon_template` и `off_icon_template` (получают номер звезды и класс иконки), события `on_rate` (новое значение и исходное событие), `on_focus`, `on_blur`, `pt`.
- Разметка как в PrimeNG 21.1.10 (`rating.ts`, `ratingstyle.ts`): корень `<div class="p-rating" data-p>` с `p-readonly` и `p-disabled`; на каждую звезду `div.p-rating-option` (`p-rating-option-active`, `p-focus-visible`) со скрытым `<input type="radio">` в `span.p-hidden-accessible` и иконкой `p-rating-icon p-rating-on-icon` или `p-rating-off-icon` (`p-invalid`). Звёзды по умолчанию — `StarFillIcon` и `StarIcon` из `prime-icons`. Повторный выбор текущего значения сбрасывает его в `None`; навигация по звёздам — стрелками, как у нативной группы радиокнопок. `aria-label` каждой звезды — из локали (`aria.star`, `aria.stars`).
- `prime-core` получает общий генератор уникальных номеров (вынос `next_unique` из `collapsible`), он же даёт имя радиогруппы по умолчанию (`pn_id_<n>_name`).
- Страница `/rating` (`basic`, `disabled`, `numberofstars`, `readonly`, `template`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/rating.md`, CSS темы `rating`.
- **Не входит:** Angular Forms (`ngModel`, реактивные формы); `autofocus`; `icon_on_class`/`icon_off_class`/`icon_on_style`/`icon_off_style` (классы шрифта иконок: вместо них `on_icon`/`off_icon` и шаблоны); блок `/* For PrimeNG */` (правило `.ng-invalid.ng-dirty`); демонстрации `reactiveforms` и `templatedrivenforms`; демонстрация `withoutcancel` (в 21.1.10 кнопки отмены у Rating уже нет, демонстрация дублирует `basic`).

## Capabilities

### New Capabilities
- `rating`: компонент Rating — значение и события, выбор мышью и клавиатурой, структура и классы, доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет: `prime-core` получает внутренний общий хелпер без изменения поведения. -->

## Impact

- Новый крейт `crates/prime-rating` (`publish = false`); зависимости `prime-core`, `prime-icons` (`star`, `starfill`), `prime-dom` не нужен.
- Изменяются `prime-core` (`ids`, `collapsible`), `Cargo.toml` workspace, `upstream.toml` (`rating` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
