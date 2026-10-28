# Proposal

## Why

ScrollPanel — кроссбраузерная тематизируемая замена нативной полосы прокрутки. Это первый компонент, который читает геометрию DOM (`scrollWidth`, `clientHeight`, `scrollTop`…), двигает полосы по `requestAnimationFrame`, перетаскивает их мышью через слушатели документа и управляется с клавиатуры. На нём отрабатывается клиентский слой «измерить → записать стиль» без SSR-зависимостей, нужный в дальнейшем Listbox, Table и виртуальной прокрутке.

## What Changes

- Новый крейт `prime-scrollpanel` с компонентом `ScrollPanel`: `step` (шаг клавиатуры, по умолчанию 5), `style`, `style_class`, слот содержимого `content_template` (замена шаблона `#content`), `children`, `pt` (части `root`, `content_container`, `content`, `bar_x`, `bar_y`).
- Разметка и классы как в PrimeNG 21.1.10 (`scrollpanelstyle.ts`): корень `div.p-scrollpanel.p-component`, `p-scrollpanel-content-container`, `p-scrollpanel-content`, две полосы `p-scrollpanel-bar p-scrollpanel-bar-x|bar-y` (`role="scrollbar"`, `tabindex="0"`, `aria-orientation`, `aria-valuenow`, `aria-controls`, `data-pc-group-section="bar"`).
- Клиентское поведение: пересчёт полос (`moveBar`) при `scroll`, `mouseenter` на содержимом и `resize` окна; класс/атрибут `p-scrollpanel-hidden`/`data-p-scrollpanel-hidden`, когда прокрутка по оси не нужна; расчёт высоты контейнера при `max-height` (`calculateContainerHeight`); перетаскивание полос мышью с `p-scrollpanel-grabbed` на полосе и `body`; стрелки при фокусе на полосе (повтор через 40 мс до `keyup`).
- Публичные методы `refresh()` и `scroll_top(value)` — через `ScrollPanelRef` (ручка, передаваемая свойством `panel_ref`).
- Страница `/scrollpanel` (`basic`, `custom`), e2e, бенчмарки, бюджеты, `docs/scrollpanel.md`, CSS темы `scrollpanel`.
- **Не входит:** `BlockableUI`, шаблон-синонимы, `appendTo`; метод `scrollTop` как публичный метод экземпляра Angular заменён ручкой.

## Capabilities

### New Capabilities
- `scrollpanel`: компонент ScrollPanel — структура и классы, ARIA полос, расчёт и скрытие полос, перетаскивание, клавиатура, `refresh`/`scroll_top`, свойства и `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-scrollpanel` (`publish = false`); зависимости `prime-core`, `web-sys` (элементы, `MouseEvent`, `KeyboardEvent`).
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`scrollpanel` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, браузерный job CI.
