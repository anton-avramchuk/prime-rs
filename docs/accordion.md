# Accordion

Порт `p-accordion` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-accordion::{Accordion, AccordionPanel, AccordionHeader, AccordionContent}` | `packages/primeng/src/accordion/accordion.ts`, `accordion/style/accordionstyle.ts` |
| Анимация | `prime-core::collapsible::bind_region_with` (общая с Panel и Fieldset) |
| CSS | `@primeuix/styles/accordion`, блок `/* For PrimeNG */` из `accordionstyle.ts` (`primeng_extra` в `upstream.toml`), базовые `p-collapsible-*` |
| Демонстрации | `apps/showcase/doc/accordion/*-doc.ts` (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`) |

Спека: `openspec/specs/accordion`, `openspec/specs/collapsible`.

## Что совпадает с PrimeNG

- Классы и атрибуты: `p-accordion p-component`, `p-accordionpanel` (+ `p-accordionpanel-active`, `p-disabled`), `p-accordionheader`, `p-accordionheader-toggle-icon`, `p-accordioncontent`, `p-accordioncontent-wrapper`, `p-accordioncontent-content`, `data-p-active`, `data-p-disabled`, `data-p="active"`.
- Заголовок: `role="button"`, `aria-expanded`, `aria-controls`, `aria-disabled`, `tabindex` `0`/`-1`; содержимое: `role="region"`, `aria-labelledby`; идентификаторы `{id}_accordionheader_{value}` и `{id}_accordioncontent_{value}`.
- Клавиатура: Enter/Space/NumpadEnter переключают, ArrowDown/ArrowUp/Home/End переносят фокус с пропуском отключённых панелей и переходом по кругу; `select_on_focus`.
- События `on_open` и `on_close` только при клике по заголовку (как в PrimeNG: переключение с клавиатуры их не вызывает) и только для той панели, по которой кликнули.

## Отличия

| Отличие | Причина |
|---|---|
| `value` — `RwSignal<Vec<String>>` для обоих режимов; начальное значение без сигнала — `default_value` | В PrimeNG `value` — `string \| number \| array`; один тип проще для Rust. Числа приводятся через `to_string`. |
| Четыре компонента связаны контекстом Leptos (`provide_context`/`use_context`), а не DI Angular | Аналог `inject(forwardRef(() => Accordion))`. Части вне `Accordion`/`AccordionPanel` паникуют с понятным сообщением. |
| Шеврон «вниз» тоже получает класс `p-accordionheader-toggle-icon` | В PrimeNG он есть только у шеврона «вверх» и у иконок из CSS-классов. |
| Слот `toggle_icon_template` вместо `expandIcon`/`collapseIcon` (CSS-классы иконок) | Шрифтов иконок в порте нет, слот принимает любой SVG. |
| Неактивное содержимое скрывается `display: none`, а не `visibility` | Как у Panel и Fieldset; общий механизм `collapsible`. |
| Корень анимации — `div.p-motion` внутри `p-accordioncontent` | Повторяет место `p-motion` PrimeNG, чтобы работало правило `.p-accordioncontent .p-motion` из его CSS. |
| Без ripple, `BlockableUI`, `transitionOptions`, `motionOptions` | Вне объёма change; анимация настраивается CSS темы. |
| Нет демонстрации `accessibility` | Это текстовая справка PrimeNG без интерактивного примера. |

## Производительность

SSR: 235 аллокаций на аккордеон из трёх панелей (бюджет 260), около 78 на панель; вклад в сырой wasm +231 КБ (бюджет 265 КБ) — основная часть это привязки `web-sys`, обработчики событий и две иконки-шеврона.
Статической ветви нет: активность каждой панели зависит от общего сигнала.
