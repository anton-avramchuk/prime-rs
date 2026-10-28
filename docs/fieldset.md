# Fieldset

Порт `p-fieldset` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-fieldset::Fieldset` | `packages/primeng/src/fieldset/fieldset.ts`, `fieldset/style/fieldsetstyle.ts` |
| Логика сворачивания | `prime-core::collapsible` (общая с Panel, см. `docs/panel.md`) |
| CSS | `@primeuix/styles/fieldset`, базовые `p-collapsible-*` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/fieldset/*-doc.ts` (`basic`, `toggleable`, `template`) |

Спека: `openspec/specs/fieldset`.

## Что совпадает с PrimeNG

- Корень `<fieldset class="p-fieldset p-component">` с `<legend class="p-fieldset-legend">`; классы `p-fieldset-toggleable`, `p-fieldset-collapsed`, `p-fieldset-legend-label`, `p-fieldset-toggle-button`, `p-fieldset-toggle-icon`, `p-fieldset-content-container`, `p-fieldset-content-wrapper`, `p-fieldset-content`; `data-p="toggleable"` на корне и легенде.
- При `toggleable` внутри легенды `<button>` с иконкой «плюс»/«минус», подписью и слотом; `aria-controls`, `aria-expanded`, `aria-label` (текст легенды).
- Область содержимого: `role="region"`, `aria-labelledby` (id кнопки), `aria-hidden`, `tabindex="-1"` при сворачивании; фокусируемые потомки свёрнутой области получают `tabindex="-1"`.
- Анимация `p-collapsible-enter-active`/`p-collapsible-leave-active`; `on_after_toggle` — по окончании анимации.

## Отличия

| Отличие | Причина |
|---|---|
| `collapsed` — `RwSignal<bool>`; начальное значение без сигнала — `default_collapsed` | Как у Panel (`docs/panel.md`). |
| Слоты — `ViewFn` (`header_template`, `expand_icon_template`, `collapse_icon_template`) вместо `ng-template` и `<p-header>` | В Leptos нет шаблонов Angular. |
| Без `toggleable` атрибут `aria-labelledby` у области не выводится | В PrimeNG он указывает на отсутствующую кнопку. |
| `pt.toggle_icon` у стандартных SVG-иконок применяет только класс | Компоненты иконок не принимают произвольные атрибуты; для слота применяются класс и атрибуты. |
| Enter/Space обрабатывает нативная `<button>` | Отдельный `keydown` не нужен. |
| Нет `BlockableUI`, `transitionOptions`, `motionOptions` | Вне объёма change; анимация настраивается CSS темы. |

## Производительность

SSR: 57 аллокаций на сворачиваемый Fieldset (бюджет 63); вклад в сырой wasm +220 КБ (бюджет 255 КБ) — основная часть это привязки `web-sys` и обработчики событий.
Шаблон — как у Panel (`docs/panel.md`).
