# Design

## Context

Требования — в `specs/chip`. Исходники PrimeNG 21.1.10: `packages/primeng/src/chip/chip.ts`, `chip/style/chipstyle.ts`, демонстрации `apps/showcase/doc/chip`. Поведение: `close()` ставит `visible = false` и вызывает `onRemove`; `visible = false` даёт inline-стиль `display: none` на корне; Enter и Backspace на элементе удаления тоже закрывают; `aria-label` берётся из `config.getTranslation(ARIA).removeLabel` (в нашей локали ключ `aria.removeLabel` уже есть). Шаблон компонента — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Chip с теми же классами, `data-p`, порядком содержимого и поведением удаления; статическая ветвь там, где она применима.
**Non-Goals:** `chipProps`, шрифтовые иконки, `unstyled`.

## Decisions

### D1. Корень — `<div>`
Хост `<p-chip>` в Angular — деталь фреймворка; у нас `<div class="p-chip …">`.

### D2. Состояние видимости
`visible` — внутренний `RwSignal<bool>`, создаётся только в реактивной ветви. Статическая ветвь выбирается условием `all_static!(…) && !removable`: Chip без удаления не может скрыться, а удаляемому нужна реактивность корня (`display: none`), поэтому он идёт реактивным путём, даже если все свойства статические. В реактивной ветви стиль корня — замыкание, зависящее от `visible`.

### D3. Элемент удаления
`span.p-chip-remove-icon[role=button][tabindex][aria-label]` с `TimesCircleIcon` (декоративной) внутри либо `remove_icon`. Отличие от PrimeNG: для иконки по умолчанию PrimeNG ставит эти атрибуты прямо на `<svg>`; у нас оболочка `span`, так как иконка `prime-icons` выставляет собственную a11y-семантику, а `role="button"` должна быть на элементе с обработчиками. Подпись берётся из `PrimeConfig.locale` при создании блока.

### D4. События
`on_remove: Callback<web_sys::Event>` получает `MouseEvent` или `KeyboardEvent` как `Event`; `on_image_error: Callback<web_sys::Event>`. Клавиши Enter и Backspace обрабатываются на `keydown` элемента удаления.

### D5. Тесты
SSR: структура, классы, порядок, `pt`, подпись из локали, паритет ветвей. Браузер (`wasm-bindgen-test`): клик, Enter, Backspace, один вызов `on_remove`, скрытие, ошибка изображения, `tabindex` при `disabled` — для статического (не удаляемого) и реактивного вариантов там, где применимо.

## Risks / Trade-offs

- [Удаляемый Chip всегда реактивный] → осознанно: корень зависит от внутреннего состояния; в спеке это зафиксировано.
- [Оболочка `span` вокруг иконки удаления отличается от PrimeNG] → задокументировано в `docs/chip.md`, проверяется сравнением скриншотов.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
