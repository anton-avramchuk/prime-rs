# Proposal

## Why

Tabs — третий компонент волны 3: переключение между представлениями вкладками с навигацией по клавиатуре, полосой-индикатором активной вкладки и прокруткой длинного списка вкладок. Составной компонент того же вида, что `Accordion` (части общаются через контекст), поэтому опирается на готовые решения проекта.

## What Changes

- Новый крейт `prime-tabs` с пятью частями (порт `p-tabs`, `p-tablist`, `p-tab`, `p-tabpanels`, `p-tabpanel` из PrimeNG 21.1.10): `Tabs` — корень `div.p-tabs.p-component` (с `p-tabs-scrollable`), `TabList` — `div.p-tablist` с кнопками прокрутки, `div.p-tablist-content` > `div.p-tablist-tab-list[role=tablist]` и полосой `span.p-tablist-active-bar`, `Tab` — `div.p-tab[role=tab]` (с `p-tab-active`, `p-disabled`), `TabPanels` — `div.p-tabpanels`, `TabPanel` — `div.p-tabpanel[role=tabpanel]`.
- `Tabs`: `value` (двусторонний `RwSignal<Option<String>>`), `default_value`, `scrollable`, `lazy`, `select_on_focus`, `show_navigators` (по умолчанию да), `tabindex` (0), `id`, `style`, `style_class`, `pt`. `Tab`: `value`, `disabled`. `TabPanel`: `value`, `lazy`. `TabList`: `prev_icon`, `next_icon` (`ViewFn`), `pt`.
- Поведение: клик (и фокус при `select_on_focus`) делают вкладку активной; стрелки влево/вправо переносят фокус на соседнюю доступную вкладку по кругу, `Home`/`End` — на первую/последнюю, `PageUp`/`PageDown` прокручивают к первой/последней, `Enter`/`Space` активируют; активной вкладке `tabindex` равен `tabindex` группы, остальным — −1. Полоса-индикатор следует за активной вкладкой; кнопки «назад»/«вперёд» появляются при прокрутке и прокручивают список на ширину видимой области. `lazy` откладывает вывод содержимого панели до первой активации (после этого оно остаётся).
- Страница showcase `/tabs` с демонстрациями `basic`, `controlled`, `disabled`, `scrollable`, `dynamic`, `lazy`, `customtemplate`; e2e с настоящей клавиатурой; бенчмарки и бюджеты; `docs/tabs.md`.
- Стили: `tabs` (в нём стили всех пяти частей) добавляется в `theme.components`.
- **Не входит:** `Ripple`, `MutationObserver` по содержимому вкладок (индикатор пересчитывается при смене значения и изменении размеров списка, `ResizeObserver`), числовые `value` (значения — строки), шаблон `content` панели через `ng-template` (содержимое `lazy`-панели — обычные `children`, вывод откладывается), демонстрация `tabmenu` (нужен Router/Menu), `pt` с контекстом.

## Capabilities

### New Capabilities
- `tabs`: вкладки с навигацией по клавиатуре, индикатором и прокруткой.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-tabs` (`publish = false`); зависимости: `prime-core`, `prime-icons` (`chevronleft`, `chevronright`), `web-sys` (`ResizeObserver`, `Element`, `HtmlElement`, `KeyboardEvent`, `ScrollIntoViewOptions`).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
