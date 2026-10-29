# Knob

Порт `p-knob` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-knob::Knob` | `packages/primeng/src/knob/knob.ts`, `knob/style/knobstyle.ts` |
| CSS | `@primeuix/styles/knob` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/knob/*-doc.ts` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`) |

Спека: `openspec/specs/knob`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-knob`, внутри `<svg viewBox="0 0 100 100" role="slider">` с `path.p-knob-range`, `path.p-knob-value` и `text.p-knob-text` (`x=50`, `y=57`, `text-anchor=middle`); классы корня `p-knob p-component`, `p-disabled`.
- Геометрия: центр (50, 50), радиус 40, шкала от −120° до +120° относительно верха (углы `4π/3` и `−π/3`), разрыв внизу; дуга значения идёт от нуля шкалы (при `min > 0` — от `min`) с флагами `large-arc` и `sweep` как в `valuePath`.
- Значение по углу: `mapRange`, округление `Math.round((v − min) / step) · step + min`; точки в разрыве внизу игнорируются.
- Клавиши: стрелки ±1 (шаг `step` не учитывается), `PageUp`/`PageDown` ±10, `Home`/`End`; результат ограничен `min..max`.
- `tabindex` равен `-1` при `readonly` и `disabled`; `aria-valuemin/max/now`, `required`, `aria-label`, `aria-labelledby`; `name` у `<text>`.
- Шаблон текста: заменяется только первое `{value}`; значение без значения показывает `min`.
- Цвета по умолчанию — переменные темы `--p-knob-range-background`, `--p-knob-value-background`, `--p-knob-text-color` (то, во что `$dt(...).variable` превращается в PrimeNG).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-knob>` | Хост в Angular — деталь фреймворка. |
| `value: RwSignal<f64>` вместо `ngModel`; без него внутреннее состояние (`default_value`, по умолчанию `min`) | Двусторонняя привязка без форм-инфраструктуры. |
| Координаты указателя — `clientX/Y` минус `getBoundingClientRect()` SVG | PrimeNG берёт `offsetX/offsetY`, которые относительны текущей цели события: у `mousemove` на `window` и у клика по `path` значение считается неверно, особенно при выходе указателя за шкалу. |
| `mouseup` и `touchend` снимают оба слушателя `window` | В PrimeNG `onMouseUp` присваивает `null` ссылке на слушатель `mouseup`, не снимая его, и тот остаётся на `window`. |
| Нажатие мыши ставит фокус на SVG | `mousedown` с `preventDefault` в PrimeNG не даёт SVG получить фокус, и клавиши работают только после Tab. |
| Результат округления по шагу ограничивается `min..max` | В PrimeNG при `max − min`, не кратном `step`, значение может выйти за `max`. |
| `click` сразу после перетаскивания пропускается | В PrimeNG он заново применяет то же значение и повторно вызывает `onChange`. |
| Значение вне `min..max` рисуется прижатым к краю шкалы, текст и `aria-valuenow` показывают исходное | В PrimeNG дуга уходит за конец шкалы. |
| Координаты в `d` округляются до трёх знаков | Короче HTML, не зависит от шума `f64`; 1/1000 единицы `viewBox` неразличима глазом. |
| Нет `styleClass` отдельной логикой, интеграции с Angular Forms | Есть `style_class`; формы вне объёма change. |

Касание покрыто только чистыми функциями (`value_from_angle`) и чтением кода; на сенсорном устройстве и синтетическими `TouchEvent` оно не проверялось (в `wasm-bindgen-test` они неудобны).

## Производительность

SSR: 54 аллокации на экземпляр (бюджет 60); вклад в сырой wasm +265 КБ (бюджет 305 КБ).
