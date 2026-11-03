# Proposal

## Why

SpeedDial — восьмой компонент волны 4: плавающая кнопка действия, которая по нажатию раскрывает набор быстрых действий (по прямой, по кругу, полукругу или четверти круга). Использует готовые `Button`, `Tooltip` и модель пунктов, как `Dock`, но добавляет геометрию раскладки и клавиатурную навигацию по направлению.

## What Changes

- Новый крейт `prime-speeddial` (порт `p-speeddial` из PrimeNG 21.1.10).
- `SpeedDialItem` (вместо `MenuItem` PrimeNG, без маршрутизатора): `id`, `label`, `icon` (`ViewFn`), `disabled`, `visible` (по умолчанию `true`), `tabindex`, `command` (`Callback<SpeedDialCommand>`), `style_class`.
- `SpeedDial`: корень `div.p-speeddial.p-component.p-speeddial-<type>` (`p-speeddial-direction-<direction>` кроме круга, `p-speeddial-open`, `p-disabled`; инлайн `flex-direction` и выравнивание по направлению) > кнопка `button.p-button.p-button-icon-only.p-speeddial-button.p-button-rounded` (`p-speeddial-rotate`, `aria-expanded`, `aria-haspopup`, `aria-controls`, `aria-label`, `aria-labelledby`) со значком «плюс» > список `ul.p-speeddial-list[role=menu]` (`id`, `tabindex=-1`, `aria-activedescendant`) > `li.p-speeddial-item[role=menuitem]` (`p-hidden`, `p-focus`, `data-p-active`, `transition-delay` и точка раскладки) > кнопка `p-speeddial-action` (вторичная, круглая, малая) со значком; при `mask` и показе — `div.p-speeddial-mask.p-overlay-mask`.
- Раскладка: `type` (`Linear`, `Circle`, `SemiCircle`, `QuarterCircle`), `direction` (восемь значений, по умолчанию `Up`), `radius` (по умолчанию `20 × число пунктов`), `transition_delay` (30 мс; при показе задержка растёт к концу списка, при скрытии — к началу); смещения `--item-diff-x/y` по разнице размеров кнопки и пункта.
- Поведение: клик по кнопке переключает показ (`on_click`, `on_visible_change`, `on_show`, `on_hide`); клик по пункту вызывает `command` и скрывает; клик вне компонента скрывает (`hide_on_click_outside`); `Escape` скрывает и возвращает фокус на кнопку; на кнопке стрелки открывают список и ведут по пунктам (↓/← вперёд, ↑/→ назад), в списке стрелки идут по направлению раскладки, `Home`/`End`, `Enter`/`Space` активируют пункт под фокусом; отключённые пункты пропускаются.
- Свойства: `model`, `visible` (управляемый показ), `id`, `style`, `style_class`, `direction`, `transition_delay`, `type`, `radius`, `mask`, `disabled`, `hide_on_click_outside`, `button_style`, `button_class`, `mask_style`, `mask_class`, `show_icon`, `hide_icon` (`ViewFn`), `rotate_animation`, `aria_label`, `aria_labelledby`, `tooltip_options` (`TooltipOptions` для подсказок пунктов), `button_severity`, `pt`; представления `button_content`, `item_content`, `icon_content`.
- Страница showcase `/speeddial` с демонстрациями `linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`; e2e; бенчмарки и бюджеты; `docs/speeddial.md`.
- Стили: `speeddial` добавляется в `theme.components`.
- **Не входит:** `routerLink` пунктов, `buttonProps` целиком (оставлена `button_severity`), `Ripple`, `Bind`, `pTemplate`, `unstyled`, классы значков `pi pi-…` (значки — `ViewFn`).

## Capabilities

### New Capabilities
- `speeddial`: плавающая кнопка с раскрывающимся набором действий и разными раскладками.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-speeddial` (`publish = false`); зависимости: `prime-core`, `prime-button`, `prime-tooltip`, `prime-icons` (plus), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-speeddial`).
