# Proposal

## Why

Password — последний компонент плана волны 2: поле ввода пароля с индикатором надёжности и переключателем показа. Он собирается из уже портированных `InputText`, иконок и оверлейного механизма `Select` (выравнивание, z-index группы `Overlay`, закрытие по клику снаружи).

## What Changes

- Новый крейт `prime-password` с компонентом `Password` (порт `p-password` из PrimeNG 21.1.10): корень `span.p-password.p-component.p-inputwrapper` с полем `InputText` (`p-password-input`, `type=password`), необязательными значками очистки и показа/скрытия и панелью-подсказкой `div.p-password-overlay` с шкалой надёжности (`p-password-meter`, `p-password-meter-label`, `p-password-meter-text`).
- Свойства: `value` (`RwSignal<String>`), `feedback` (по умолчанию `true`), `toggle_mask`, `show_clear`, `prompt_label`, `weak_label`, `medium_label`, `strong_label` (по умолчанию из локали), `strength` (своя оценка надёжности), `placeholder`, `autocomplete`, `input_id`, `input_style`, `input_style_class`, `max_length`, `disabled`, `readonly`, `invalid`, `required`, `variant`, `size`, `fluid`, `aria_label`, `style`, `style_class`, `pt`; шаблоны `header`, `footer`, `content`, `show_icon`, `hide_icon`, `clear_icon` — `ViewFn`; обратные вызовы `on_focus`, `on_blur`, `on_clear`.
- Надёжность: пусто → подсказка; непусто → слабая; «средняя» — не менее 6 символов и два вида из {строчные, прописные, цифры}; «сильная» — не менее 8 символов и все три вида (то же, что регулярные выражения PrimeNG по умолчанию, но без `mediumRegex`/`strongRegex`).
- Панель: открывается по фокусу и вводу, закрывается по `Escape`, потере фокуса; выравнивается под полем (`align_overlay`), z-index группы `Overlay`, анимация классами `p-password-enter`/`p-password-leave`.
- Страница showcase `/password` с демонстрациями `basic`, `meter`, `togglemask`, `clearicon`, `template`, `locale`, `filled`, `sizes`, `fluid`, `disabled`, `invalid`, `floatlabel`, `iftalabel`; e2e; бенчмарки и бюджеты; `docs/password.md`.
- Стили: `password` добавляется в `theme.components` и `primeng_extra` (блок `/* For PrimeNG */` с анимациями и `min-width`).
- **Не входит:** `tabindex` и `aria-labelledby` поля (их нет у `InputText`), директива `pPassword` (рядом с `InputText` она не нужна), `mediumRegex`/`strongRegex` (вместо них `strength`), `appendTo` кроме `self`, `overlayOptions`, `motionOptions` и устаревшие `showTransitionOptions`/`hideTransitionOptions`, привязки Angular Forms (`ngModel`, reactive forms), `pt` по частям с контекстом.

## Capabilities

### New Capabilities
- `password`: поле пароля с индикатором надёжности, переключателем показа и очисткой.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-password` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-inputtext`, `prime-icons` (`eye`, `eyeslash`, `times`), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
