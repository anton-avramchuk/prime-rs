# Panel

Порт `p-panel` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-panel::Panel` | `packages/primeng/src/panel/panel.ts`, `panel/style/panelstyle.ts` |
| Логика сворачивания | `prime-core::collapsible` (общая с будущим Fieldset) |
| CSS | `@primeuix/styles/panel`, базовые `p-collapsible-*` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/panel/*-doc.ts` (`basic`, `toggleable`, `template`); `header-toggler` — по тексту `toggleable-doc` |

Спека: `openspec/specs/panel`, `openspec/specs/collapsible`.

## Что совпадает с PrimeNG

- Классы `p-panel p-component`, `p-panel-toggleable`, `p-panel-expanded`/`p-panel-collapsed`, `p-panel-header`, `p-panel-title`, `p-panel-header-actions` (+ `p-panel-icons-start|end|center`), `p-panel-toggle-button`, `p-panel-content-container`, `p-panel-content-wrapper`, `p-panel-content`, `p-panel-footer`; `data-p="toggleable"`.
- Кнопка-переключатель: классы `p-button` как у `<p-button severity="secondary" text rounded>` с иконкой, иконки «минус» (развёрнута) и «плюс» (свёрнута).
- ARIA: кнопка — `aria-label` (текст заголовка), `aria-controls`, `aria-expanded`; область — `role="region"`, `aria-labelledby`, `aria-hidden`, `tabindex="-1"` при сворачивании; фокусируемые потомки свёрнутой области получают `tabindex="-1"`.
- Анимация: классы `p-collapsible-enter-active`/`p-collapsible-leave-active`, как у `pMotion="p-collapsible"`; `on_after_toggle` — по окончании анимации.
- `toggler = header` переключает щелчком по шапке, `icon_pos`, `show_header`, `collapsed` как двусторонняя привязка.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-panel>` | Хост в Angular — деталь фреймворка. |
| `collapsed` — `RwSignal<bool>`; начальное значение без сигнала — `default_collapsed` | Двусторонняя привязка в Leptos делается сигналом (как `value` у InputText); одно свойство не может быть и сигналом, и литералом. |
| Слоты — `ViewFn` (`header_template`, `icons_template`, `footer_template`), `header_icons_template` получает `collapsed` | Вместо `ng-template` и фасетов `<p-header>`/`<p-footer>`. |
| Кнопка рисуется самим Panel, а не компонентом Button | `aria-expanded` меняется реактивно, а `pt` у Button принимает только статические атрибуты. Классы и `data-p` совпадают. |
| Идентификатор `{id}_header` есть у заголовка, а при его отсутствии — у кнопки | В PrimeNG он и у заголовка, и у кнопки (повтор `id`); `aria-labelledby` указывает на единственный элемент. |
| Свёрнутая область скрыта встроенным `display: none` (при первом рендере сразу), анимация идёт только при смене | Нет анимации при SSR и гидратации; без `animationend` (отключённые анимации, скрытая вкладка) завершение наступает по запасному таймеру 400 мс. |
| Обработчик `keydown` для Enter/Space не нужен | Нативная `<button>` превращает их в `click`. |
| Нет `BlockableUI`, `toggleButtonProps`, `motionOptions`, `transitionOptions` | Вне объёма change; анимация настраивается CSS темы. |
| Меню шестерёнки из демонстрации `template` не перенесено | Menu ещё не портирован; иконки — SVG из `prime-icons` вместо шрифта PrimeIcons. |

## Производительность

SSR: 63 аллокации на сворачиваемую панель (бюджет 70), 44 на простую (бюджет 48); вклад в сырой wasm +234 КБ (бюджет 270 КБ) — основная часть это привязки `web-sys` и обработчики событий.
Шаблон — как у Card (`docs/card.md`) и ToggleSwitch (`docs/toggleswitch.md`).
