# Proposal

## Why

Steps — первый компонент волны 5: индикатор шагов мастера (нумерованные пункты в ряд), более простой предшественник `Stepper`. Он остаётся в PrimeNG 21, его стили (`@primeuix/styles/steps`) есть, а в проекте нужен как самый короткий порт для разогрева волны и как образец «навигации по пунктам-ссылкам» (клавиатура с roving `tabindex`), который затем переиспользуют Menubar и PanelMenu.

## What Changes

- Новый крейт `prime-steps` (порт `p-steps` из PrimeNG 21.1.10).
- `StepsItem` (вместо `MenuItem` PrimeNG, без полей маршрутизатора): `id`, `label`, `url`, `target`, `disabled`, `tabindex`, `style`, `command` (`Callback<StepsCommand>` с событием, пунктом и индексом).
- `Steps`: `nav.p-steps.p-component` (`p-readonly`, `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` (`p-steps-item-active`, `p-disabled`, `aria-current="step"` у активного, `id`) > `a.p-steps-item-link[role=link]` (`href`, `target`, `tabindex`, `aria-expanded`, `aria-disabled`) > `span.p-steps-item-number` (номер с единицы) и `span.p-steps-item-label`.
- Поведение: по умолчанию `readonly = true` — клик ничего не делает (`preventDefault`), все пункты кроме активного отключены; в интерактивном режиме клик по доступному пункту вызывает `on_active_index_change(i)` и `command`, переход по ссылке отменяется, если у пункта нет `url`; клавиши ←/→ по пунктам, `Home`/`End`, `Enter`/`Space` — клик, с roving `tabindex` (активный пункт `0`, остальные `-1`; `Tab` возвращает фокус-индикатор на активный пункт).
- Свойства: `model`, `active_index` (двусторонний `RwSignal<usize>` или значение), `readonly` (по умолчанию `true`), `exact`, `style`, `style_class`, `pt`; события `on_active_index_change`.
- Страница showcase `/steps` с демонстрациями `basic`, `controlled`, `interactive`; e2e; бенчмарки и бюджеты; `docs/steps.md`.
- Стили: `steps` добавляется в `theme.components`.
- **Не входит:** `routerLink` и поля маршрутизатора (`queryParams`, `fragment`, …), `RouterModule`-определение активного пункта по адресу (активный пункт — только `active_index`), подсказки пунктов (`tooltipOptions`), HTML-метки (`escape = false`), `Bind`, `pTemplate`. Демонстрация `routing` не переносится: в локальной копии PrimeNG каталога `doc/steps` нет (демонстрации удалены апстримом при чистке), набор демонстраций выведен из списка в `pages/steps/index.ts` (`basic`, `controlled`, `interactive`, `routing`, `accessibility`).

## Capabilities

### New Capabilities
- `steps`: индикатор шагов мастера с необязательной интерактивной навигацией.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-steps` (`publish = false`); зависимости: `prime-core`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-steps`).
