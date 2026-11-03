# Proposal

## Why

SplitButton — третий компонент волны 4: кнопка с действием по умолчанию и соседней кнопкой-стрелкой, раскрывающей список дополнительных команд. Он собирается из готовых `Button` и всплывающего `Menu` и закрывает типовой сценарий «Сохранить ▾ → Сохранить как…».

## What Changes

- Новый крейт `prime-splitbutton` (порт `p-splitbutton` из PrimeNG 21.1.10).
- `SplitButton`: корень `div.p-splitbutton.p-component` (модификаторы `p-splitbutton-raised`, `-rounded`, `-outlined`, `-text`, `-sm`/`-lg`; `data-p-severity`) > кнопка по умолчанию `p-splitbutton-button` (`Button`) > кнопка раскрытия `p-splitbutton-dropdown p-button-icon-only` со стрелкой (`aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) > всплывающее меню (`Menu` с `popup`, привязанное к корню).
- Свойства: `model` (`Vec<MenuItem>`), `severity`, `raised`, `rounded`, `text`, `outlined`, `plain`, `size`, `icon`, `icon_pos`, `label`, `button_aria_label`, `expand_aria_label`, `dropdown_icon`, `disabled`, `button_disabled`, `menu_button_disabled`, `tabindex`, `autofocus`, `style`, `style_class`, `menu_style`, `menu_style_class`, `pt`; представление `content` (вместо метки кнопки); события `on_click`, `on_dropdown_click`, `on_menu_show`, `on_menu_hide`.
- Поведение: клик по основной кнопке вызывает `on_click` и закрывает меню; клик по кнопке раскрытия вызывает `on_dropdown_click` и переключает меню у корня; ↓/↑ на кнопке раскрытия открывают меню (без прокрутки страницы); показ и скрытие меняют `aria-expanded` и вызывают `on_menu_show`/`on_menu_hide`.
- Страница showcase `/splitbutton` с демонстрациями `basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`; e2e; бенчмарки и бюджеты; `docs/splitbutton.md`.
- Стили: `splitbutton` добавляется в `theme.components`.
- **Не входит:** вложенные подменю (`nested`): в PrimeNG меню — `TieredMenu`, а в проекте пока только плоский `Menu` с группами; вложенность появится вместе с `TieredMenu` (последний компонент волны 4), тогда внутренний `Menu` заменится без изменения API. Также не входят `tooltip`/`tooltipOptions` (директива `use:tooltip` доступна снаружи), `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `Ripple`, `@primeuix/motion`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `splitbutton`: кнопка с действием по умолчанию и раскрывающимся списком команд.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-splitbutton` (`publish = false`); зависимости: `prime-core`, `prime-button`, `prime-menu`, `prime-icons`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
