# Proposal

## Why

Tooltip — седьмой компонент волны 3 и единственный в ней, который в PrimeNG не компонент, а директива `pTooltip` на произвольном элементе (`<input pTooltip="Enter your username">`). Нужен простой способ подсказки у любого элемента, не меняющий разметку страницы; кроме того, подсказка — следующий потребитель позиционирования и слоёв (`ZGroup::Tooltip`) после `Popover` и `Select`.

## What Changes

- Новый крейт `prime-tooltip` (порт директивы `Tooltip` из PrimeNG 21.1.10). В Leptos аналог `pTooltip` — `use:`-директива: `<input use:tooltip=TooltipOptions::text("Enter your username") />`. Директива — функция `tooltip(el: web_sys::Element, options: TooltipOptions)`, которая вешает слушатели на элемент и создаёт подсказку при показе; разметка страницы не меняется (никаких обёрток).
- `TooltipOptions` (строитель): `content` (`TooltipContent`: `Text(Prop<String>)`, `Html(String)` — аналог `[escape]="false"`, `View(ViewFn)` — аналог шаблона), `position` (`TooltipPosition`: `Right` по умолчанию, `Left`, `Top`, `Bottom`), `event` (`TooltipEvent`: `Hover` по умолчанию, `Focus`, `Both`), `show_delay`, `hide_delay`, `life` (мс), `position_top`, `position_left` (смещения, px), `auto_hide` (да), `fit_content` (да), `hide_on_escape` (да), `show_on_ellipsis` (нет), `disabled` (`Prop<bool>`), `position_style`, `style_class`, `z_index` (`None` — автоматически по `ZGroup::Tooltip`), `id`.
- Подсказка — `div.p-tooltip.p-component.p-tooltip-<position>[role=tooltip]` в `body` с `div.p-tooltip-arrow` и `div.p-tooltip-text`. Позиция считается так же, как в PrimeNG: основная сторона, а при выходе за экран — следующие по приоритету (для `top`: top → bottom → right → left и т. д.); стрелка сдвигается к центру элемента.
- Поведение: `Hover` — показ на `mouseenter` и касание, скрытие на `mouseleave`, `click` и конец касания; `Focus` — на `focus`/`blur` самого элемента (или поля внутри обёртки `p-inputwrapper`); `show_delay`/`hide_delay`/`life`; при `auto_hide` выключенном подсказку можно навести мышью, и она скрывается при уходе курсора; `Escape`, изменение размера окна и прокрутка предка скрывают подсказку; при смене текста активная подсказка обновляется и пересчитывается; при `show_on_ellipsis` показ только если текст элемента обрезан.
- Страница showcase `/tooltip` с демонстрациями `position`, `event`, `delay`, `autohide`, `options`, `custom`; e2e с настоящей мышью и клавиатурой; бенчмарки и бюджеты; `docs/tooltip.md`.
- Стили: `tooltip` добавляется в `theme.components`.
- **Не входит:** `appendTo` (подсказка всегда в `body`; `target` и произвольный элемент не поддерживаются), `pTooltipPT` / `pTooltipUnstyled` (без `Bind`/`pt`), задержка 100 мс внутри `p-dialog` (в нашем `Dialog` она не нужна), анимация `fadeIn` как JS-утилита заменена CSS-переходом непрозрачности, демонстрация `accessibility` (текстовая страница).

## Capabilities

### New Capabilities
- `tooltip`: подсказка у элемента с показом по наведению или фокусу, задержками, выбором стороны и автоматическим переносом при нехватке места.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-tooltip` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
