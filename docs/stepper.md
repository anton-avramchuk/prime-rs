# Stepper

Порт `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item` и `p-stepper-separator` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-stepper::Stepper`, `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem`, `StepContext`, `StepPt`, `StepPanelPt` | `stepper/stepper.ts`, `stepper/style/*` |
| CSS | `@primeuix/styles/stepper` и правило `.p-steppanel .p-motion` из `stepperstyle.ts` — `cargo xtask gen-theme`, `primeng_extra` |
| Демонстрации | `apps/showcase/doc/stepper/{basic,linear,vertical,template,stepsonly}-doc.ts` |

Спека: `openspec/specs/stepper`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-stepper.p-component[role=tablist]` (`p-readonly` при `linear`), `div.p-steplist`, `div.p-step[role=presentation]` (`p-step-active`, `p-disabled`, `aria-current="step"`, `data-p-active`, `data-p-disabled`) с кнопкой `button.p-step-header[role=tab]` (`id`, `aria-controls`, `tabindex=-1` и `disabled` у недоступного) > `span.p-step-number` + `span.p-step-title`, `div.p-stepper-separator` после каждого шага списка, кроме последнего; `div.p-steppanels`, `div.p-steppanel[role=tabpanel]` (`id`, `aria-controls`, `data-p-active`, `data-pc-name="steppanel"`) > `div.p-steppanel-content-wrapper` > `div.p-steppanel-content`; `div.p-stepitem` (`p-stepitem-active`).
- Идентификаторы `<id>_step_<N>` и `<id>_steppanel_<N>`; `id` по умолчанию `pn_id_<N>`.
- Недоступность: `!active && (linear || disabled)`; клик по шагу записывает его значение в `value`.
- В горизонтальной раскладке содержимое выводится только у активной панели (как `p-motion` с `visible = active`); в вертикальной (`StepItem`) панель получает `p-steppanel-active`, разделитель панели — для каждого элемента, кроме последнего, шаг внутри `StepItem` разделителя не выводит.
- Клавиатурной навигации сверх нативных кнопок нет — как в PrimeNG.

## Отличия

| Отличие | Причина |
|---|---|
| Части — обычные `div`, без хостов `<p-stepper>`, `<p-step>` и т. п. | Хосты в Angular — деталь фреймворка. |
| Значения — `u32` (в PrimeNG — число), `value` — `RwSignal<Option<u32>>` | Двусторонняя привязка сигналами; `None` — нет активного шага. |
| `content` вместо шаблона `#content`: `Callback<StepContext, AnyView>`, где `StepContext { value, active: Signal<bool>, activate: Callback<u32> }` | Аналог `activateCallback`, `value`, `active` шаблона PrimeNG; `activate(n)` ставит значение `n` (у шага — любое, как `updateValue`). Обычное содержимое — дочерние элементы. |
| Разделитель шага определяется реестром шагов в контексте `Stepper` (порядок создания = порядок в DOM) | Вместо `contentChildren`. Шаг, вставленный динамически в середину списка, порядок реестра нарушит: добавлять и убирать можно только в конце. |
| Вертикальная раскладка: неактивные панели остаются в разметке (`display: none`), содержимое всех панелей выводится сразу; раскрытие анимируют классы темы `p-collapsible-*` через `prime_core::collapsible::bind_region_with` | В PrimeNG `p-motion` убирает содержимое неактивной панели; анимация нужна, пока содержимое ещё на месте. |
| Нет `transitionOptions`, `motionOptions` и `@primeuix/motion` | Анимация — классы темы, как у Accordion. |
| Контекст `Stepper` и `StepItem` кладётся в дочернего владельца, а содержимое возвращается как `OwnedView` с этим владельцем | Иначе контекст течёт на соседей, идущих после компонента (горизонтальная панель после вертикального `Stepper` считала бы себя вертикальной); `OwnedView` нужен, чтобы шаги, добавляемые условием уже после монтирования, тоже находили контекст. |
| `xtask gen-theme` для компонентов из `primeng_extra` берёт CSS после `${…_style}` в литерале, если маркера `/* For PrimeNG */` нет | У `stepperstyle.ts` нет маркера, но правило `.p-steppanel .p-motion` нужно для вертикальной анимации. |
| Демонстрация `template`: значки `pi-user`/`pi-id-card` заменены `HomeIcon`/`CheckIcon`, картинка с CDN — «🎉»; `ngModel` — сигналы | В `prime-icons` нет подходящих значков; независимость от CDN. |
| Класс `p-readonly` при `linear` вместо `p-stepper-readonly` из CSS темы | Следуем `stepperstyle.ts`: селектор `.p-stepper.p-stepper-readonly .p-step` в стилях не срабатывает ни у PrimeNG, ни у нас. |

## Что не проверялось

- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Раскладка справа налево и сенсорный ввод.
- Плавность анимации высоты вертикальной панели оценивалась только по классам и итоговому состоянию (`display`), не по кадрам.

## Производительность

См. `benches/` (`stepper-1`, `stepper-1000`, `case-stepper`) и `benches/*budgets.toml`.
