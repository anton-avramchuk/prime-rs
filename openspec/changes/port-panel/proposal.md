# Proposal

## Why

Panel — контейнер с шапкой, необязательным сворачиванием и подвалом; первый компонент с анимацией раскрытия и связкой `aria-expanded`/`aria-controls`. Логика сворачивания (двусторонний `collapsed`, переключатель, анимация, ARIA) нужна и Fieldset, и будущим Accordion, поэтому её выделяют в общий код при первом же порте, а не копируют.

## What Changes

- Новый крейт `prime-panel` с компонентом `Panel`: `header`, `toggleable`, двусторонний `collapsed` (`RwSignal<bool>`), `icon_pos` (`start`/`end`/`center`), `show_header`, `toggler` (`icon`/`header`), `id`, `style`, `style_class`, слоты `header_template`, `icons_template`, `header_icons_template` (получает `collapsed`), `footer_template`, события `on_before_toggle`, `on_after_toggle`, `pt`.
- Классы как в PrimeNG 21.1.10 (`panelstyle.ts`): `p-panel p-component`, `p-panel-toggleable`, `p-panel-expanded`/`p-panel-collapsed`, `p-panel-header`, `p-panel-title`, `p-panel-header-actions` (+ `p-panel-icons-start|end|center`), `p-panel-toggle-button`, `p-panel-content-container`, `p-panel-content-wrapper`, `p-panel-content`, `p-panel-footer`; `data-p` как в `dataP`.
- Доступность: кнопка-переключатель с `aria-controls`, `aria-expanded`, `aria-label` (текст заголовка), Enter/Space; контейнер содержимого с `role="region"`, `aria-labelledby`, `aria-hidden`, `tabindex="-1"` при свёрнутом; фокусируемые элементы внутри свёрнутого содержимого получают `tabindex="-1"`.
- Анимация раскрытия: классы `p-collapsible-enter-active`/`p-collapsible-leave-active` из `base.css` темы; `on_after_toggle` вызывается по окончании анимации (`animationend`).
- Общая логика сворачивания выделяется в модуль `prime-core::collapsible` (состояние, переключение, анимация, ARIA-атрибуты), на нём строятся Panel и следующий Fieldset.
- Страница `/panel` (`basic`, `toggleable`, `header-toggler`, `template`), e2e, бенчмарки, бюджеты, `docs/panel.md`, CSS темы `panel`.
- **Не входит:** Fieldset (отдельный change `port-fieldset`), `BlockableUI`, `toggleButtonProps`, `motionOptions`/`transitionOptions`, `<p-header>`/`<p-footer>` как фасеты (заменены слотами).

## Capabilities

### New Capabilities
- `panel`: компонент Panel — структура и классы, сворачивание и двусторонний `collapsed`, ARIA и клавиатура, анимация и `on_after_toggle`, слоты, `pt`, производительность.
- `collapsible`: общая логика сворачивания в `prime-core` — состояние, переключение, события до/после, ARIA-атрибуты переключателя и области, анимация; переиспользуется Panel и Fieldset.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-panel` (`publish = false`); зависимости `prime-core`, `prime-icons` (`minus`, `plus`), `web-sys`.
- `prime-core`: новый модуль `collapsible`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`panel` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, браузерный job CI.
