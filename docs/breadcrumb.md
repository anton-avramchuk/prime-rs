# Breadcrumb

Порт `p-breadcrumb` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-breadcrumb::Breadcrumb`, `BreadcrumbItem` | `breadcrumb/breadcrumb.ts`, `breadcrumb/style/breadcrumbstyle.ts` |
| CSS | `@primeuix/styles/breadcrumb` — `cargo xtask gen-theme` (блока `/* For PrimeNG */` у компонента нет) |
| Демонстрации | `apps/showcase/doc/breadcrumb/{basic,template}-doc.ts` |

Спека: `openspec/specs/breadcrumb`.

## Что совпадает с PrimeNG

- Разметка и классы: `nav.p-breadcrumb.p-component` > `ol.p-breadcrumb-list` > `li.p-breadcrumb-home-item`, `li.p-breadcrumb-item` (с `p-disabled`), `li.p-breadcrumb-separator`; в пункте `a.p-breadcrumb-item-link` со значком, подписью и бейджем.
- Пункт «домой» без своего значка получает `HomeIcon`; разделитель по умолчанию — `ChevronRightIcon`.
- Разделитель после пункта «домой» выводится, только если есть пункты модели; после последнего пункта — нет.
- Клик: отключённый пункт отменяет действие и ничего не вызывает; пункт без `url` отменяет переход, но вызывает `command` и `on_item_click` (логика — чистая функция `click_plan` с табличным тестом).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<nav>` без хоста `<p-breadcrumb>` | Хост в Angular — деталь фреймворка. |
| `BreadcrumbItem` вместо `MenuItem` | Общий тип меню появится вместе со вторым потребителем (`Menu`, `Tabs`). |
| Нет `routerLink`, `queryParams`, `fragment` и др. | Это API Angular Router; навигацию SPA ведёт `command` или `url` и свой роутер. |
| Значок пункта — `ViewFn` (иконка из `prime-icons`), а не имя класса шрифта | Иконки проекта — SVG-компоненты. |
| Шаблоны `item` и `separator` — колбэк и `ViewFn` | Вместо `ng-template`. |
| Нет `tooltipOptions`, HTML-меток (`escape = false`), `pt` по пункту с контекстом `{item, index}`, демонстрации `router` | Привязаны к Angular или выходят за объём партии. |
| `Steps` не переносится | В PrimeNG 21 устарел (заменён `Stepper`), нет страницы и демонстраций. |
| В демонстрации `template` вместо иконок `sitemap`, `book`, `wallet`, `shopping-bag`, `calculator` — `star`, `calendar`, `eye`, `info-circle`, `search` | Этих иконок нет в `prime-icons`. |

## Стоимость

176 аллокаций на экземпляр при SSR (домой и четыре пункта, четыре разделителя с SVG), вклад в wasm ≈ 244 КБ (из них основная часть — общий `Badge` и иконки).

## Не проверено

Клавиатура и экранные считыватели не проверялись на реальных устройствах: проверены разметка, вычисленные стили и отсутствие перехода по клику (e2e).
