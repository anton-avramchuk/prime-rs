# Proposal

## Why

Следующая партия волны 2 — компоненты отображения без ввода и без модели `MenuItem`: `OverlayBadge` (бейдж поверх произвольного содержимого), `Timeline` (цепочка событий) и `MeterGroup` (составная шкала с подписями). Они опираются только на готовый `Badge`, `prime-core` и CSS темы, поэтому их можно сделать одним пакетом по уже отработанному циклу. `Breadcrumb` и `Steps` зависят от общей модели пунктов меню (`MenuItem`: ссылки, `routerLink`, `command`, видимость, отключение), которой в проекте ещё нет; их разумнее вынести в отдельную партию вместе с этой моделью.

## What Changes

- Три новых крейта:
  - `prime-overlaybadge` (`OverlayBadge`): `<div class="p-overlaybadge">` с дочерним содержимым и вложенным `Badge`; свойства `value`, `severity`, `badge_size`, `badge_disabled`, `style_class`, `style`, `pt` (`root`, `pc_badge`).
  - `prime-timeline` (`Timeline<T>`): события из `value`, `align` (`left`/`right`/`top`/`bottom`/`alternate`, по умолчанию `left`), `layout` (`Vertical`/`Horizontal`), шаблоны `content`, `opposite`, `marker` как `Callback<T, AnyView>`; корень `p-timeline p-component p-timeline-<align> p-timeline-<layout>`, части `event`, `eventOpposite`, `eventSeparator`, `eventMarker`, `eventConnector` (у последнего события нет), `eventContent`; `data-p`.
  - `prime-metergroup` (`MeterGroup`): `value: Vec<MeterItem>` (`label`, `color`, `value`, `icon`), `min`, `max`, `orientation`, `label_position`, `label_orientation`, шаблоны `label`, `meter`, `start`, `end`, `icon`; `role="meter"` с `aria-valuemin/max/now`; ширина или высота отрезка в процентах с округлением и ограничением 0–100.
- CSS темы `overlaybadge`, `timeline`, `metergroup` добавляются в `theme.components`.
- Страницы showcase `/overlaybadge`, `/timeline`, `/metergroup` с демонстрациями из документации PrimeNG; e2e, бенчмарки и бюджеты для трёх крейтов, `docs/display-batch.md`.
- **Не входит:** `Breadcrumb` и `Steps` (нужна модель `MenuItem` — отдельная партия); `ng-content`-шаблоны по типу `PrimeTemplate` (заменяются свойствами-колбэками); устаревшее свойство `size` у OverlayBadge; `hostName`; HTML-метки (`escape = false`); иконки-шрифты (`icon` у `MeterItem` — `ViewFn`/класс-имя иконки из `prime-icons`, не `<i class>`).

## Capabilities

### New Capabilities
- `overlaybadge`: бейдж, наложенный на правый верхний угол содержимого.
- `timeline`: вертикальная и горизонтальная цепочка событий с маркером, соединителем и подписями.
- `metergroup`: составная шкала из нескольких отрезков с подписями и общим значением.

### Modified Capabilities
<!-- Нет: существующие требования не меняются. -->

## Impact

- Новые крейты `crates/prime-overlaybadge`, `prime-timeline`, `prime-metergroup` (`publish = false`); зависимости: `prime-core`, `prime-badge` (у OverlayBadge).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`.
