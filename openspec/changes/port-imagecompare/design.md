# Design

## Context

Требования — в `specs/imagecompare`. Исходники PrimeNG 21.1.10: `imagecompare/imagecompare.ts` (около 150 строк), `imagecompare/style/imagecomparestyle.ts`; стили `@primeuix/styles/imagecompare`; демонстрации `apps/showcase/doc/imagecompare/{basic,responsive}-doc.ts`. Образец в проекте: `prime-slider`/`prime-knob` (обработчики `input` и `cfg(wasm32)` под DOM-вызовы), `prime-divider` (простая разметка).

Наблюдения по исходнику:
- Шаблон: `left`, `right`, `input[type=range][min=0][max=100][value=50]`; обработчик `onSlide` читает `event.target.value`, находит `previousElementSibling` ползунка и задаёт ему `style.clipPath`.
- Тема: `.p-imagecompare img + img { clip-path: polygon(0 0, X 0, X 100%, 0 100%) }` с `X = 50%` по умолчанию; значит, правое изображение должно быть вторым `img` подряд после левого. Поэтому представления выводятся без обёрток.
- Направление определяется при создании и наблюдателем `MutationObserver` за `dir` на `<html>`: `isRTL = !!el.closest('[dir=rtl]')`; применяется только в `onSlide`, так что вычисление в момент ввода даёт тот же результат.
- `tabindex`, `aria-labelledby`, `aria-label` — на хосте (корне).

## Goals / Non-Goals

**Goals:** тот же DOM и классы; тот же `clip-path` при вводе, включая `rtl`.

**Non-Goals:** `MutationObserver`, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и API
`prime-imagecompare` с `ImageCompare`, `ImageComparePt { root, slider }`. Свойства: `left: ViewFn`, `right: ViewFn` (обязательные), `tabindex: OptProp<i32>`, `aria_label`, `aria_labelledby`, `style`, `style_class`, `pt`.

### D2. Чистая логика (`state.rs`)
`clip_path(value: f64, rtl: bool) -> String` — строка `polygon(...)` с форматированием процентов без хвостовых нулей (целые — без десятичной части; `30` → `30%`, `33.5` → `33.5%`); значения вне 0–100 прижимаются. Табличные тесты, включая `rtl`.

### D3. Обработчик ввода
`on:input` ползунка (только браузер; в SSR обработчик не вызывается): читает `HtmlInputElement::value`, находит `previous_element_sibling`, определяет `rtl` по `closest("[dir=rtl]")` ползунка, задаёт `style.clip-path` через `HtmlElement::style().set_property`. Отсутствие соседа или нечисловое значение — тихий выход.

### D4. CSS
`imagecompare` добавляется в `theme.components`. Проверить `p-imagecompare`, `p-imagecompare-slider`, переменные ручки; блок `For PrimeNG` в `imagecomparestyle.ts` нет.

### D5. Тесты
SSR: порядок и состав, атрибуты ползунка, `tabindex`/`aria-*`, `style`/`style_class`/`pt`, представления без обёрток. Браузерные `tests/web.rs`: ввод 30 → `clip-path` на предыдущем элементе, `rtl` в предке, повторный ввод, ввод без соседа. e2e: настоящее перетаскивание ручки (мышь по диапазону), `clip-path` от значения, скриншотная пара.

### D6. Демонстрации
Изображения с CDN заменяются двумя встроенными SVG (`data:` URI) разных цветов с подписью — независимость от сети. `responsive` — контейнер фиксированной ширины.

### D7. Бенчмарки
Сценарии `imagecompare-1`/`-1000` (две `<img>`), `case-imagecompare`; сначала `cargo bench -p render-bench --bench alloc`; для wasm — отдельный файл бюджетов `--budgets`.

## Risks / Trade-offs

- [Структура `img + img`] Если пользователь передаст не `img` — правило темы не сработает; как в PrimeNG, фиксируется в docs.
- [Ввод до гидратации] Обработчик подключается при гидратации; ранний ввод теряется, как у остальных компонентов.

## Open Questions

- Нужно ли двустороннее значение (`value` как сигнал): в PrimeNG нет, не добавляем.

## Migration Plan

Миграции нет. Откат — `git revert`.
