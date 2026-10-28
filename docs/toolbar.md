# Toolbar

Порт `p-toolbar` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-toolbar::Toolbar` | `packages/primeng/src/toolbar/toolbar.ts`, `toolbar/style/toolbarstyle.ts` |
| CSS | `@primeuix/styles/toolbar` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/toolbar/*-doc.ts` (`basic`, `custom`) |

Спека: `openspec/specs/toolbar`.

## Что совпадает с PrimeNG

- Корень `div.p-toolbar.p-component` с `role="toolbar"` и `aria-labelledby`.
- Области `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` в этом порядке и только при заданном слоте; проецируемое содержимое стоит перед ними.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-toolbar>` | Хост в Angular — деталь фреймворка. |
| Слоты — `ViewFn` (`start_template`, `center_template`, `end_template`), содержимое — `children` | Вместо шаблонов `#start`, `#center`, `#end` и `<ng-content>`. |
| Нет синонимов шаблонов `left` и `right` | Устаревшие имена `start` и `end`. |
| Нет `BlockableUI` | Интерфейс блокировки Angular. |
| Демонстрация `basic`: кнопка «Save» вместо SplitButton, `InputText` с иконкой поиска вместо IconField, иконки SVG из `prime-icons` (плюс, корзина, загрузка) вместо шрифта PrimeIcons (плюс, печать, загрузка) | SplitButton, IconField и InputIcon ещё не портированы; иконки печати нет в наборе. |

## Производительность

SSR: 23 аллокации на экземпляр с тремя областями (бюджет 26), из них около половины — три слота `ViewFn`; вклад в сырой wasm +96 КБ (бюджет 110 КБ).
Шаблон — как у Card (`docs/card.md`).
