# Design

## Context

Требования — в `specs/fieldset`. Исходники PrimeNG 21.1.10: `packages/primeng/src/fieldset/fieldset.ts`, `fieldset/style/fieldsetstyle.ts`, демонстрации `apps/showcase/doc/fieldset`. Образец — `crates/prime-panel` (`docs/panel.md`): те же `CollapseState`, `CollapseIds`, `bind_region`, `aria_*`, `hidden_style`, структура двух ветвей и `macro_rules!`. Логика сворачивания не меняется, поэтому спека `collapsible` не затрагивается.

## Decisions

### D1. Только разметка и привязка
`Fieldset` создаёт `CollapseState::new(collapsed, default_collapsed)` и `CollapseIds::new(id)`, вызывает `bind_region` (только `wasm32` и `toggleable`), атрибуты ARIA берёт из функций `prime_core::collapsible`. Копий анимации, `tabindex` или идентификаторов в крейте нет.

### D2. Идентификаторы
`{id}_header` — `id` кнопки, на него указывает `aria-labelledby` области; `{id}_content` — идентификатор области. Без `toggleable` кнопки нет, поэтому `aria-labelledby` не выводится (в PrimeNG он остаётся висячей ссылкой). `{id}-titlebar` не используется.

### D3. Легенда
- без `toggleable`: `<legend><span.p-fieldset-legend-label>{legend}</span>{header_template}</legend>`;
- с `toggleable`: `<legend><button …>{иконка}<span.p-fieldset-legend-label>{legend}</span>{header_template}</button></legend>`.
Иконка: `span.p-fieldset-toggle-icon` вокруг слота `expand_icon_template`/`collapse_icon_template`, а без слотов — svg `PlusIcon`/`MinusIcon` с классом `p-fieldset-toggle-icon`. Иконка зависит от `collapsed`, поэтому читается во вложенном замыкании, а легенда не перерисовывается целиком.

### D4. Кнопка
Собственный `<button type="button">` с `aria-controls`, `aria-expanded`, `aria-label`. Enter/Space — нативный `click` (как в Panel). `on_before_toggle` получает `MouseEvent` и прежний `collapsed`; `preventDefault` вызывается.

### D5. Ветви и классы
Статическая ветвь — не `toggleable` и все свойства статические. Разметка — `macro_rules!`; `Arc<Props>` и `Arc<FieldsetPt>`; `NodeRef` только на `wasm32` (как в Panel). `data-p` на корне и легенде. Порядок классов — как в `fieldsetstyle.ts`. Корень — `<fieldset>`, как в PrimeNG.

### D6. Тесты
SSR: структура, классы, ARIA, слоты, `pt`, паритет. Браузер: клик, сигнал, `aria-expanded`, анимационный класс и `on_after_toggle`, `tabindex` потомков, события. e2e — реальная CSS-анимация.

## Risks / Trade-offs

- [`<button>` внутри `<legend>` зависит от стилей темы] → проверяется в e2e по видимости и клику.
- [Разметка Panel и Fieldset похожа] → общая только логика (`prime-core`), разметка различается; выносить общий компонент рано.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
