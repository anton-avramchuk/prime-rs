# ToggleSwitch

Порт `p-toggleswitch` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-toggleswitch::ToggleSwitch` | `packages/primeng/src/toggleswitch/toggleswitch.ts`, `toggleswitch/style/toggleswitchstyle.ts` |
| CSS | `@primeuix/styles/toggleswitch` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/toggleswitch/*-doc.ts` (`basic`, `preselection`, `template`, `invalid`, `disabled`) |

Спека: `openspec/specs/toggleswitch`.

## Что совпадает с PrimeNG

- Структура: корень со `style="position: relative"`, внутри `input.p-toggleswitch-input[role=switch]` (`aria-checked`), затем `div.p-toggleswitch-slider` с `div.p-toggleswitch-handle`.
- Классы корня и их порядок: `p-toggleswitch p-component`, `p-toggleswitch-checked`, `p-disabled`, `p-invalid`.
- Атрибуты `data-p-checked`, `data-p-disabled` на корне и `data-p` на корне, дорожке и ручке.
- Клик обрабатывается на корне (как `@HostListener('click')`), затем фокус возвращается на `<input>`; `readonly` и `disabled` блокируют переключение.
- Слот ручки получает текущее `checked`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-toggleswitch>` | Хост в Angular — деталь фреймворка. |
| `checked: RwSignal<bool>` вместо `ngModel` | Двусторонняя привязка без форм-инфраструктуры. |
| Нет `trueValue`/`falseValue`, `autofocus`, `size`, интеграции с формами | В PrimeNG `size` не влияет на классы; остальное вне объёма change. |
| `handle_template: Callback<bool, AnyView>` вместо шаблона `#handle` | Слоты Leptos. |
| Без `checked` переключатель неконтролируемый (`default_checked`), классы корня не отслеживаются | Статическая ветвь не создаёт сигналов. |
| Нативное состояние `<input>` выравнивается после клика явно | Клик по дорожке не переключает `<input>` нативно, клик по `<input>` переключает до обработчика корня. |

## Производительность

SSR: 33 аллокации на экземпляр (бюджет 40); вклад в сырой wasm +167 КБ (бюджет 210 КБ).
Шаблон — как у Checkbox (`docs/checkbox.md`).
