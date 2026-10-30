# Tabs

Порт `p-tabs`, `p-tablist`, `p-tab`, `p-tabpanels`, `p-tabpanel` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-tabs::Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt` | `tabs/{tabs,tablist,tab,tabpanels,tabpanel}.ts`, `tabs/style/*` |
| CSS | `@primeuix/styles/tabs` — `cargo xtask gen-theme` (один модуль на все пять частей; блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/tabs/{basic,controlled,disabled,scrollable,dynamic,lazy,customtemplate}-doc.ts` |

Спека: `openspec/specs/tabs`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-tabs.p-component` (+ `p-tabs-scrollable`) > `div.p-tablist` > кнопки `p-tablist-prev-button` / `p-tablist-next-button` и `div.p-tablist-content.p-tablist-viewport` > `div.p-tablist-tab-list[role=tablist]` с вкладками `div.p-tab[role=tab]` (`p-tab-active`, `p-disabled`, `aria-selected`, `aria-controls`, `data-p-active`, `data-p-disabled`) и полосой `span.p-tablist-active-bar`; панели `div.p-tabpanels` > `div.p-tabpanel[role=tabpanel]` (`hidden` у неактивных).
- Идентификаторы `<id группы>_tab_<value>` и `<id группы>_tabpanel_<value>`, по умолчанию `pn_id_<N>`.
- Клавиши на вкладке: `ArrowRight`/`ArrowLeft` по кругу с пропуском отключённых, `Home`/`End`, `PageUp`/`PageDown` (прокрутка без фокуса), `Enter`/`NumpadEnter`/`Space`; `tabindex` — только у активной вкладки; `select_on_focus`.
- Полоса-индикатор получает `width` и `left` активной вкладки; кнопки прокрутки появляются по положению прокрутки и размерам списка; шаг прокрутки считается так же, как в PrimeNG (ширина видимой области, для «вперёд» — за вычетом кнопок).
- `lazy` (у `Tabs` или `TabPanel`): содержимое выводится при первой активации и остаётся.

## Отличия

| Отличие | Причина |
|---|---|
| Вкладка — `<div>` без хоста `<p-tab>` и т. п. | Хосты в Angular — деталь фреймворка. |
| Значения вкладок — строки (`value="0"`), `value` группы — `RwSignal<Option<String>>` | В PrimeNG — `string \| number`; единый тип проще для сравнения. |
| Поиск соседней вкладки — по DOM-детям списка и чистой функции `wrap_target` | Вместо рекурсии по `nextElementSibling`; результат тот же. |
| Индикатор пересчитывается при смене значения и по `ResizeObserver` списка; `MutationObserver` по содержимому вкладок нет | Содержимое вкладки, меняющее её ширину без изменения размера списка, индикатор не обновит. |
| `prev_icon` / `next_icon` — `ViewFn`, нет `Ripple` | Вместо `ng-template` и директивы `pRipple`. |
| Содержимое `lazy`-панели — обычные `children`, нет шаблона `content` | Вывод откладывается самим компонентом. |
| В `customtemplate` значки `pi-minus` / `pi-plus` заменены знаками «−» / «+», утилитные классы Tailwind — встроенными стилями | В `prime-icons` нет этих значков. |
| Демонстрация `tabmenu` не перенесена | Нужны `Router` и `Menu`. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |

## Стоимость

281 аллокация на экземпляр при SSR (три вкладки и три панели); вклад в wasm ≈ 374 КБ (бюджет 420 КБ в `benches/budgets.toml`).

## Не проверено

Касание, `rtl` в реальном браузере и экранные считыватели. Проверены разметка, клик, клавиатура, `lazy`, индикатор и кнопки прокрутки в Chrome (e2e и браузерные тесты).
