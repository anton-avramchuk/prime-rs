# InputText

Порт `pInputText` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-inputtext::InputText` | `packages/primeng/src/inputtext/inputtext.ts`, `inputtext/style/inputtextstyle.ts` |
| CSS | `@primeuix/styles/inputtext` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/inputtext/*-doc.ts` (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`) |

Спека: `openspec/specs/inputtext`.

## Что совпадает с PrimeNG

- Классы `p-inputtext p-component`, `p-filled`, `p-inputtext-sm`/`-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid` в порядке `inputtextstyle.ts`.
- `data-p`: `invalid`, `fluid`, `filled` (при варианте `filled`), размер.
- Вариант: свойство `variant`, иначе `PrimeConfig.input_variant`.

## Отличия

| Отличие | Причина |
|---|---|
| Компонент `<InputText />`, а не директива на `<input>` | В Leptos нет директив на произвольный элемент. Чаще нужные атрибуты `<input>` — явные свойства. |
| `value: RwSignal<String>` вместо `ngModel` | Двусторонняя привязка без форм-инфраструктуры. |
| Без `value` поле неконтролируемое, `p-filled` не отслеживается | Статическому полю не нужен сигнал; в PrimeNG `p-filled` влияет только на плавающие подписи. |
| Нет интеграции с формами, `KeyFilter`, `FloatLabel`, `IconField`, `InputGroup`, контекста `Fluid` | Вне объёма change. |

## Модель значения (образец для полей ввода)

- `value` — необязательный `RwSignal<String>`; поле читает его (`prop:value` плюс атрибут `value` для SSR) и пишет в него при `input`, затем вызывает `on_input`.
- `on_input`/`on_change` получают исходное `web_sys::Event`, `on_focus`/`on_blur` — `FocusEvent`.
- Статическая ветвь (`all_static!` и нет `value`) не создаёт замыканий и обработчиков значения.

## Производительность

SSR: 13 аллокаций на экземпляр (бюджет 16); вклад в сырой wasm +175 КБ (бюджет 225 КБ) — основная часть это обработчики событий `web-sys`.
Шаблон — как у Button (`docs/button.md`).
