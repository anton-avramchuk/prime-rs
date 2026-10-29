# InputGroup

Порт `p-inputgroup` и `p-inputgroup-addon` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-inputgroup::InputGroup` | `packages/primeng/src/inputgroup/inputgroup.ts`, `inputgroup/style/inputgroupstyle.ts` |
| `prime-inputgroup::InputGroupAddon` | `packages/primeng/src/inputgroupaddon/inputgroupaddon.ts`, `inputgroupaddon/style/inputgroupaddonstyle.ts` |
| CSS | `@primeuix/styles/inputgroup` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/inputgroup/*-doc.ts` (`basic`, `button`, `checkbox`, `multiple`) |

Спека: `openspec/specs/inputgroup`.

## Что совпадает с PrimeNG

- Корень `div.p-inputgroup`, аддон `div.p-inputgroupaddon`; внутри проецируемое содержимое без дополнительных обёрток.
- У хостов нет `role` и `data-p`.

## Отличия

| Отличие | Причина |
|---|---|
| Корни — `<div>`, а не хосты `<p-inputgroup>` и `<p-inputgroup-addon>` | Хост в Angular — деталь фреймворка. |
| У `InputGroup` есть `style` | В PrimeNG его нет у `InputGroup` (только у аддона); для единообразия с остальными компонентами. |
| Нет блока `/*For PrimeNG*/` из `inputgroupstyle.ts` | Правила для Angular-обёрток `p-button`, `p-inputwrapper`, `p-inputmask`, которых в Leptos нет. |
| Нет `fluid` | В PrimeNG 21.1.10 у `InputGroup` такого входа нет: `instance.fluid` всегда `undefined`, класс `p-inputgroup-fluid` не выводится. |
| Нет демонстраций `floatlabel` и `iftalabel` | FloatLabel и IftaLabel ещё не портированы. |
| В `basic` и `button` вместо InputNumber, Select и Menu — `InputText` и обычные кнопки; иконки SVG из `prime-icons` (`search`, `star-fill`, `check`, `times`) вместо шрифта PrimeIcons (`user`, `map`, `clock`) | Этих компонентов ещё нет в порте, а иконок `user`, `map`, `clock` нет в наборе. |

## Производительность

SSR: 19 аллокаций на экземпляр с двумя аддонами и полем (бюджет 21); вклад в сырой wasm +93 КБ (бюджет 107 КБ).
Шаблон — как у Toolbar (`docs/toolbar.md`).
