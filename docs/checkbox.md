# Checkbox

Порт `p-checkbox` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-checkbox::Checkbox` | `packages/primeng/src/checkbox/checkbox.ts`, `checkbox/style/checkboxstyle.ts` |
| CSS | `@primeuix/styles/checkbox` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/checkbox/*-doc.ts` (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`) |

Спека: `openspec/specs/checkbox`.

## Что совпадает с PrimeNG

- Структура: корень, внутри `input.p-checkbox-input`, затем `div.p-checkbox-box` с иконкой `check` или `minus` (`p-checkbox-icon`).
- Классы корня и их порядок: `p-checkbox p-component`, `p-checkbox-checked p-highlight`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`.
- Атрибуты `data-p-highlight`, `data-p-checked`, `data-p-disabled` на корне и `data-p` на корне и боксе.
- Режимы `binary` и набор значений; `indeterminate` не считается отмеченным и сбрасывается после изменения.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-checkbox>` | Хост в Angular — деталь фреймворка. |
| `checked: RwSignal<bool>` (binary) и `model: RwSignal<Vec<String>>` (набор) вместо `ngModel` | Двусторонняя привязка без форм-инфраструктуры; значения набора — строки. |
| Нет `trueValue`/`falseValue` произвольных типов | Только `bool`. |
| `icon_template: ViewFn` вместо `checkboxIcon` (класс) и шаблона `#icon` | Слоты Leptos. |
| `pt.icon` задаёт только класс, `data-p` у иконки нет | Иконка — отдельный компонент `prime-icons`, не принимает произвольные атрибуты. |
| Без сигналов флажок неконтролируемый (`default_checked`), классы корня не отслеживаются | Статическая ветвь не создаёт сигналов. |
| Нет интеграции с формами | Вне объёма change. |

## Производительность

SSR: 34 аллокации на экземпляр с иконкой (бюджет 42); вклад в сырой wasm +201 КБ (бюджет 250 КБ).
Шаблон — как у InputText (`docs/inputtext.md`) и Button (`docs/button.md`); замыкания реактивной ветви собираются через `rx(props, pt, fn)` над общими `Arc`.
