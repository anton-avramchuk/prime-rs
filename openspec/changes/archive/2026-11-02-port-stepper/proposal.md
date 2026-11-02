# Proposal

## Why

Stepper — последний компонент волны 3: пошаговый мастер (wizard) с нумерованными шагами, разделителями и панелями содержимого, горизонтальный или вертикальный, свободный или линейный. Он близок к Tabs по устройству (составной компонент с общим активным значением) и закрывает волну 3 из плана.

## What Changes

- Новый крейт `prime-stepper` (порт `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item`, `p-stepper-separator` из PrimeNG 21.1.10).
- Составной компонент, как Tabs: [`Stepper`] хранит активное значение и общается с частями через контекст Leptos. Части: `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem` (вертикальная раскладка).
- Разметка и классы как в PrimeNG: `div.p-stepper.p-component[role=tablist]` (`p-readonly` при `linear`), `div.p-steplist`, `div.p-step` (`p-step-active`, `p-disabled`, `aria-current="step"`, `data-p-active`, `data-p-disabled`) с кнопкой `button.p-step-header[role=tab]` (`span.p-step-number`, `span.p-step-title`) и разделителем `div.p-stepper-separator` (кроме последнего шага), `div.p-steppanels`, `div.p-steppanel[role=tabpanel]` (`p-steppanel-active` в вертикальном режиме) с `div.p-steppanel-content-wrapper` > разделитель (вертикальный режим, кроме последнего элемента) + `div.p-steppanel-content`, `div.p-stepitem` (`p-stepitem-active`).
- Состояние: `Stepper.value: RwSignal<Option<u32>>` (двусторонний) и `default_value`; `linear` (шаги недоступны для клика, пока не активны; переходы — программно); `Step.disabled`; идентификаторы `<id>_step_<N>` и `<id>_steppanel_<N>`, `aria-controls` связывает шаг и панель.
- Свои представления: `content` у `Step` и `StepPanel` получает `StepContext { value, active, activate }` (аналог `activateCallback`, `value`, `active` шаблона `content`).
- Поведение: клик по шагу активирует его (если не отключён и не `linear`); активна одна панель (горизонтальный режим — содержимое выводится только у активной; вертикальный — все панели в разметке, неактивные скрыты, раскрытие анимируется классами темы); разделитель не выводится после последнего шага/элемента, закрашивается перед активным.
- Страница showcase `/stepper` с демонстрациями `basic`, `linear`, `vertical`, `template`, `stepsonly`; e2e; бенчмарки и бюджеты; `docs/stepper.md`.
- Стили: `stepper` добавляется в `theme.components`.
- **Не входит:** `transitionOptions`/`motionOptions`/`@primeuix/motion` (анимация — классы темы), `Bind`, `pTemplate`, форсированное закрытие других панелей, клавиатурная навигация между шагами (в PrimeNG её нет — шаги это обычные кнопки), демонстрация `accessibility` (текстовая).

## Capabilities

### New Capabilities
- `stepper`: пошаговый мастер с шагами, панелями, разделителями, линейным режимом и вертикальной раскладкой.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-stepper` (`publish = false`); зависимости: `prime-core`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
