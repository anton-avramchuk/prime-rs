# Steps

Порт `p-steps` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-steps::Steps`, `StepsItem`, `StepsCommand`, `StepsPt` | `packages/primeng/src/steps/steps.ts`, `steps/style/stepsstyle.ts` |
| CSS | `@primeuix/styles/steps` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | В этой копии PrimeNG каталога `apps/showcase/doc/steps` нет (апстрим удалил демонстрации, страница `pages/steps/index.ts` на них ссылается); набор `basic`, `controlled`, `interactive` выведен из списка на странице и кода компонента |

Спека: `openspec/specs/steps`.

```rust
let active = RwSignal::new(0usize);
view! { <Steps model=items active_index_signal=active readonly=false /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `nav.p-steps.p-component` (`p-readonly`, `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` (`p-steps-item-active`, `p-disabled`, `aria-current="step"`) > `a.p-steps-item-link[role=link]` (`aria-expanded`, `aria-disabled`, `tabindex`) > `span.p-steps-item-number` и `span.p-steps-item-label`.
- По умолчанию `readonly = true`: клик ничего не делает, неактивные пункты отключены. В интерактивном режиме клик по доступному пункту сообщает индекс (`on_active_index_change`), вызывает `command` пункта с событием, пунктом и индексом и отменяет переход по ссылке, если у пункта нет `url`.
- Клавиатура: ←/→/`Home`/`End` переносят фокус с roving `tabindex`, `Tab` возвращает `tabindex="0"` активному пункту, `Enter`/`Space` — клик.
- Правила `tabindex`: у отключённого `-1`, у активного собственный (или `0`), у остальных собственный (или `-1`).

## Отличия

| Отличие | Причина |
|---|---|
| Нет `routerLink` и определения активного пункта по адресу маршрутизатора; активный шаг — только `active_index` | Нет маршрутизатора Angular. |
| Текущий шаг — `active_index` (значение или сигнал) и необязательный двусторонний `active_index_signal` вместо `activeIndex`/`activeIndexChange` | Идиома Leptos. |
| Пункты нумеруются по индексу; в PrimeNG `track item.label` склеивал пункты с одинаковой подписью | Ошибка исходника. |
| Нет подсказок пунктов (`tooltipOptions`), HTML-подписей (`escape = false`), `Bind`, `pTemplate` | Не переносятся. |
| `exact` принимается, но поведения не имеет | В PrimeNG влияет только на ссылки маршрутизатора. |
| Демонстрация `routing` не перенесена | Нет маршрутизатора. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Сравнение с эталонными скриншотами PrimeNG: демонстраций в локальной копии нет.

## Производительность

См. `benches/` (`steps-1`, `steps-1000`, `case-steps`) и `benches/*budgets.toml`.
