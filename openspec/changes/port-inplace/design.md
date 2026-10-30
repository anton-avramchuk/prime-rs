# Design

## Context

Требования — в `specs/inplace`. Исходники PrimeNG 21.1.10: `inplace/inplace.ts`, `inplace/style/inplacestyle.ts`; демонстрации `apps/showcase/doc/inplace/{basic,input,image,lazy}-doc.ts`. Образцы в проекте: `prime-button` (кнопка закрытия), `prime-scrolltop` (разметка поверх `Button`), `prime-fieldset` (модель двустороннего сигнала и `Pt`).

Наблюдения по исходнику:
- `onActivateClick`/`onDeactivateClick` проверяют `preventClick`; `onKeydown` для `Enter` вызывает `activate` без этой проверки и `preventDefault`.
- `activate`/`deactivate` ничего не делают при `disabled`.
- Кнопка закрытия без `closeIcon` содержит `TimesIcon`; с `closeIcon` — значок из класса шрифта.

## Decisions

### D1. Крейт и зависимости
`prime-inplace` с `Inplace` и `InplacePt { root, display, content, close_button }`. Зависит от `prime-core`, `prime-button`, `prime-icons` (`times`), `web-sys`.

### D2. Чистая логика (`state.rs`)
`display_classes(disabled)` — `p-inplace-display` и `p-disabled`; `next_state(active, action, disabled, prevent_click) -> bool` — табличное решение для `Click`, `Enter`, `Close`. Табличный тест покрывает все сочетания.

### D3. Состояние
`active: Option<RwSignal<bool>>`; без него локальный сигнал с начальным значением `false`. `Callback<Callback<()>, AnyView>` для `content` вызывается при каждом показе содержимого; функция закрытия `Callback<()>` делает `deactivate` (без события — `on_deactivate` получает событие кнопки, а из колбэка содержимого — не вызывается с событием, поэтому `on_deactivate: Callback<()>` без аргумента).

### D4. Колбэки без события
`on_activate` и `on_deactivate` — `Callback<()>`: событие DOM в Leptos-обработчике нужно только для `prevent_default` и отдельно не интересно потребителям (отличие записывается в docs).

### D5. Разметка
Содержимое выбирается замыканием по `active`. Display — `div` с `on:click`, `on:keydown`; Content — `div` с `content.run(close)` и при `closable` — `Button` с `button_type="button"`, `text`-вариантом по умолчанию PrimeNG (без `text`, как в оригинале).

### D6. CSS
`inplace` добавляется в `theme.components`; `cargo xtask gen-theme`, проверка, что CSS остальных не изменился.

### D7. Тесты
SSR: вид по умолчанию, активное, `disabled`, `closable`, `pt`. Браузерные `tests/web.rs`: клик, `Enter`, `prevent_click`, `disabled`, кнопка закрытия, колбэк закрытия, внешний сигнал `active`, вызовы `on_activate`/`on_deactivate`. e2e: настоящие клики и клавиши на странице. Крейт добавляется в три места `ci.yml`.

### D8. Бенчмарки
Сценарии `inplace-1`/`-1000` (SSR, вид), `case-inplace`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет.

## Risks / Trade-offs

- [Перерисовка содержимого при каждом показе] → так ведёт себя `*ngIf` в оригинале; фиксируется в docs.
- [Фокус при переключении] → в оригинале фокус не переносится; повторяем.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
