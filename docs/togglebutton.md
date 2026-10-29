# ToggleButton

Порт `p-togglebutton` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-togglebutton::ToggleButton` | `packages/primeng/src/togglebutton/togglebutton.ts`, `togglebutton/style/togglebuttonstyle.ts` |
| CSS | `@primeuix/styles/togglebutton`, пресет Aura и блок `/* For PrimeNG (iconPos) */` (`cargo xtask gen-theme`, `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/togglebutton/*-doc.ts` (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`) |

Спека: `openspec/specs/togglebutton`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-togglebutton[role=button]`, внутри `span.p-togglebutton-content` с иконкой (`span.p-togglebutton-icon` и `-left`/`-right`) и `span.p-togglebutton-label`.
- Классы корня и их порядок: `p-togglebutton p-component`, `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` / `p-togglebutton-lg p-inputfield-lg`, `p-togglebutton-fluid`.
- Атрибуты `aria-pressed`, `tabindex` (`0`, при `disabled` `-1`), `data-pc-name`, `data-p-checked`, `data-p-disabled`, `data-p` (`checked invalid <size>`) на корне и `data-p` на содержимом.
- `toggle` как в PrimeNG: клик и клавиши Enter и Space (по `event.code`, с `preventDefault`); `disabled` и `allow_empty = false` у включённой кнопки блокируют переключение.
- Метки по умолчанию «Yes» и «No», пустая метка выводится как пробел; слоты `icon_template` и `content_template` получают текущее `checked`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-togglebutton>` | Хост в Angular — деталь фреймворка. |
| `checked: RwSignal<bool>` вместо `ngModel`; без него внутренний сигнал с `default_checked` | Двусторонняя привязка без форм-инфраструктуры; в PrimeNG состояние тоже хранит сам компонент. |
| `on_icon`/`off_icon` — `ViewFn` в `span.p-togglebutton-icon`, а не класс PrimeIcons | В `prime-icons` нет шрифта; оболочка нужна для CSS (`order` у `-right`). |
| `icon_pos` — собственный `IconPos` (`Left`, `Right`) | Зависимость от `prime-button` ради перечисления не нужна. |
| `icon_template`, `content_template`: `Callback<bool, AnyView>` | Слоты Leptos. |
| Нет `autofocus`, ripple, интеграции с Angular Forms | Вне объёма change; правило `.ng-invalid.ng-dirty` в CSS безвредно. |
| В демонстрации `customized` `CheckIcon` и `TimesIcon` вместо `pi-lock` и `pi-lock-open` | В `prime-icons` нет замков. |
| Без статической ветви: разметка всегда реактивная | Любое нажатие меняет `class`, `aria-pressed`, `data-*` и метку. |

`xtask gen-theme` принимает маркер `/* For PrimeNG */` с необязательным пояснением в скобках (в `togglebuttonstyle.ts` — `(iconPos)`).

## Производительность

SSR: 23 аллокации на экземпляр (бюджет 26); вклад в сырой wasm +169 КБ (бюджет 200 КБ).
