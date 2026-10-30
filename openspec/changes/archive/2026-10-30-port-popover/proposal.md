# Proposal

## Why

Popover — четвёртый компонент волны 3: плавающая панель с произвольным содержимым, привязанная к элементу-триггеру. Это второй оверлей проекта после `Select`; на нём отрабатывается императивный показ (`toggle`/`show`/`hide` от события и цели), который затем понадобится `Menu`, `Tooltip`, `Dialog` и `Drawer`.

## What Changes

- Новый крейт `prime-popover` (порт `p-popover` из PrimeNG 21.1.10): `Popover` — `div.p-popover.p-component[role=dialog]` > `div.p-popover-content` с содержимым-`children`. Показывается и скрывается через управляющий объект `PopoverRef` (`Copy`): `toggle(&event)`, `toggle_at(&event, target)`, `show(&event)`, `show_at(&event, target)`, `hide()`, `visible()`; создаётся `PopoverRef::new()` и передаётся в `Popover` свойством `popover_ref`.
- Свойства `Popover`: `popover_ref`, `dismissable` (да), `focus_on_show` (да), `auto_z_index` (да), `base_z_index` (0), `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `pt`, обратные вызовы `on_show` / `on_hide`.
- Поведение: цель выравнивания — `currentTarget` события или явная `target`; панель позиционируется `prime_dom::align_overlay`, стрелка сдвигается свойством `--p-popover-arrow-left`, при размещении над целью ставятся `data-p-popover-flipped` и `p-popover-flipped`; клик вне панели и цели при `dismissable`, `Escape`, изменение размера окна (не на сенсорных устройствах) и прокрутка предка цели скрывают панель; повторный `toggle` с другой целью скрывает панель и показывает её у новой; при показе фокус получает первый элемент с `autofocus`; `z-index` по `ZGroup::Overlay`; у триггера не ставятся `aria-expanded`/`aria-controls` (см. «Не входит»).
- Страница showcase `/popover` с демонстрациями `basic`, `target`, `selectdata`, `template` (закрытие через `close`-колбэк), `datatable` — не переносится (нужен DataTable); e2e с настоящей мышью и клавиатурой; бенчмарки и бюджеты; `docs/popover.md`.
- Стили: `popover` добавляется в `theme.components`.
- **Не входит:** `appendTo` (панель всегда внутри корня, как у `Select`), `OverlayService`-шина кликов между оверлеями, анимации `@primeuix/motion` (панель без перехода), `MotionOptions`, `datatable`-демонстрация, автоматические `aria-expanded`/`aria-controls` на триггере (триггер — произвольный элемент пользователя; атрибуты ставит он сам по `visible()`), `pt` с контекстом.

## Capabilities

### New Capabilities
- `popover`: плавающая панель у элемента-триггера с императивным показом и закрытием.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-popover` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `web-sys` (`Element`, `HtmlElement`, `MouseEvent`, `KeyboardEvent`).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
