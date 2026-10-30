# Design

## Context

Требования — в `specs/breadcrumb`. Исходники PrimeNG 21.1.10: `breadcrumb/breadcrumb.ts`, `breadcrumb/style/breadcrumbstyle.ts`; демонстрации `apps/showcase/doc/breadcrumb/{basic,template,router}-doc.ts`. Образцы в проекте: `crates/prime-chip` (клик, значок `ViewFn`, `web-sys` события), `crates/prime-badge` (вложенный `Badge`), `crates/prime-timeline` (шаблоны-колбэки, список из `Prop<Vec<_>>`), `prime_core::Pt`.

Наблюдения по исходнику:
- В шаблоне два почти одинаковых блока ссылки — с `href` и с `routerLink`; без Angular Router остаётся только первый.
- `onClick`: отключённый — `preventDefault` и выход; без `url` и `routerLink` — `preventDefault`; затем `command` и `onItemClick`.
- Разделитель после пункта «домой» выводится при `model && home`; после пункта модели — `!end && visible`.
- Блока `/* For PrimeNG */` у стиля нет — `primeng_extra` не меняется.
- `Steps` в 21.1.10 помечен устаревшим, у него нет страницы в showcase; в проект не переносится.

## Decisions

### D1. Крейт и состав
`prime-breadcrumb` с `Breadcrumb`, `BreadcrumbItem`, `BreadcrumbPt`. Зависимости: `prime-core`, `prime-badge`, `prime-icons` (`chevronright`, `home`), `web-sys` (`Event`, `MouseEvent`). Общего `MenuItem` не вводим до второго потребителя.

### D2. Модель
`BreadcrumbItem { id, label, icon: Option<ViewFn>, url, target, title, disabled, visible (по умолчанию true), tabindex, style_class, style, link_class, badge, command: Option<Callback<BreadcrumbClick>> }` с построителем (`BreadcrumbItem::new().label("…")`), `Clone`. `BreadcrumbClick { event: MouseEvent, item: BreadcrumbItem }` — также аргумент `on_item_click`. `model: Prop<Vec<BreadcrumbItem>>`, `home: OptProp<BreadcrumbItem>`.

### D3. Разметка и клики
Список пунктов строится в одном замыкании по `model`/`home` (сигнал перестраивает список целиком). Клик: `on:click` ссылки вызывает чистую функцию `click_plan(disabled, has_url) -> ClickPlan { prevent_default, run_commands }` (табличный тест), по результату — `prevent_default`, `command`, `on_item_click`. Вызов на SSR невозможен, поэтому логику проверяют табличным тестом, а поведение — e2e.

### D4. Шаблоны
`item: Option<Callback<BreadcrumbItem, AnyView>>` и `separator: Option<ViewFn>`. Шаблон `item` заменяет содержимое `li`, обёртка и классы остаются.

### D5. CSS
`breadcrumb` добавляется в `theme.components`; `cargo xtask gen-theme`, проверка, что CSS остальных не изменился.

### D6. Демонстрации и тесты
`/breadcrumb`: `basic` (домой + пять пунктов), `template` (значки вместо подписей, разделитель `/`; вместо `pi-sitemap`, `pi-book` и др. — имеющиеся иконки `prime-icons`), `router` не переносится. SSR-тесты на каждый сценарий; табличный тест `click_plan`; e2e: разметка, число разделителей, клик по пункту с командой (счётчик в демонстрации не нужен — проверяется через `url` и `preventDefault`), отключённый пункт. Бенчмарки `breadcrumb-1` и `-1000`, `case-breadcrumb`.

## Risks / Trade-offs

- [Своя модель вместо общего `MenuItem`] → при появлении `Menu`/`Tabs` выделить общий тип; `BreadcrumbItem` останется псевдонимом или обёрткой.
- [Клики не проверяются в SSR] → табличный тест решения и e2e.
- [Нет `routerLink`] → навигацию SPA пользователь ведёт через `command` (или `url` и свой роутер); различие фиксируется в `docs/breadcrumb.md`.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
