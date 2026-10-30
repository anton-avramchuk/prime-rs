# Proposal

## Why

`Breadcrumb` — следующий компонент волны 2: путь по иерархии страниц, который нужен в любом приложении с вложенной навигацией. Он опирается на уже готовые `Badge`, `ChevronRightIcon`, `HomeIcon` и CSS темы. В исходном плане стояли ещё `Steps`, но в PrimeNG 21 компонент помечен устаревшим (заменён `Stepper`), у него нет страницы в showcase и демонстраций для сравнения, поэтому в проект он не переносится; `Stepper` — отдельная, более крупная работа.

## What Changes

- Новый крейт `prime-breadcrumb`: `Breadcrumb` и модель пункта `BreadcrumbItem`.
  - Корень `<nav class="p-breadcrumb p-component">` с `<ol class="p-breadcrumb-list">`; пункт домой (`li.p-breadcrumb-home-item`, по умолчанию с `HomeIcon`), пункты модели (`li.p-breadcrumb-item`, `p-disabled` для отключённых), разделители `li.p-breadcrumb-separator` с `ChevronRightIcon` или своим шаблоном.
  - Ссылка пункта — `a.p-breadcrumb-item-link` с `href` (`url`), `target`, `title`, `tabindex`, значком, подписью `span.p-breadcrumb-item-label` и необязательным `Badge`; клик отключённого пункта отменяется, пункт без `url` не переходит по ссылке, у остальных вызывается `command` и `on_item_click`.
  - Свойства: `model`, `home`, `home_aria_label`, `style`, `style_class`, `pt`; шаблоны `item` и `separator` — колбэки.
- CSS темы `breadcrumb` добавляется в `theme.components`.
- Страница showcase `/breadcrumb` с демонстрациями `basic` и `template` (демонстрация `router` опирается на Angular Router и не переносится), e2e, бенчмарки и бюджеты, `docs/breadcrumb.md`.
- **Не входит:** `Steps` (устарел, нет демонстраций); `routerLink` и его параметры (`queryParams`, `fragment` и т. п.) — вместо них `url`; общий для всех меню тип `MenuItem` (появится вместе со вторым потребителем — `Menu` или `Tabs`); `tooltipOptions`; HTML-метки (`escape = false`); `pt` по пункту с контекстом `{item, index}`.

## Capabilities

### New Capabilities
- `breadcrumb`: путь по иерархии страниц с пунктом домой, разделителями и шаблонами.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-breadcrumb` (`publish = false`); зависимости: `prime-core`, `prime-badge`, `prime-icons` (`chevronright`, `home`), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`.
