# RadioButton

Порт `p-radiobutton` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-radiobutton::RadioButton` | `packages/primeng/src/radiobutton/radiobutton.ts`, `radiobutton/style/radiobuttonstyle.ts` |
| CSS | `@primeuix/styles/radiobutton` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/radiobutton/*-doc.ts` (`group`, `filled`, `sizes`, `invalid`, `disabled`) |

Спека: `openspec/specs/radiobutton`.

## Что совпадает с PrimeNG

- Структура: корень, внутри `input.p-radiobutton-input` (`aria-checked`), затем `div.p-radiobutton-box` с `div.p-radiobutton-icon`.
- Классы корня и их порядок: `p-radiobutton p-component`, `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`.
- Атрибуты `data-p-disabled`, `data-p-checked`, `data-p` на корне.
- Событие `on_click` со значением и исходным событием (`onClick` PrimeNG).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-radiobutton>` | Хост в Angular — деталь фреймворка. |
| Группа — общий `model: RwSignal<Option<String>>`, без `RadioControlRegistry` | Реестр нужен Angular-формам; общий сигнал обновляет всю группу реактивно. |
| Атрибут `value` у `<input>` — собственное значение кнопки | В PrimeNG там `modelValue()` (булево состояние), что ломает нативную отправку формы. |
| Нет `binary`, `autofocus`, интеграции с формами | Вне объёма change. |
| Без `model` кнопка неконтролируемая (`default_checked`), классы корня не отслеживаются | Статическая ветвь не создаёт сигналов. |

## Производительность

SSR: 23 аллокации на экземпляр (бюджет 28); вклад в сырой wasm +168 КБ (бюджет 210 КБ).
Шаблон — как у Checkbox (`docs/checkbox.md`).
