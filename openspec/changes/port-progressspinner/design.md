# Design

## Context

Требования — в `specs/progressspinner`. Исходники PrimeNG 21.1.10: `packages/primeng/src/progressspinner/progressspinner.ts`, `progressspinner/style/progressspinnerstyle.ts`, демонстрации `apps/showcase/doc/progressspinner` (`basic`, `custom`). CSS темы получим через `cargo xtask gen-theme`; блок `/* For PrimeNG */` в `progressspinnerstyle.ts` отсутствует. Шаблон крейта — `prime-toolbar` (две ветви, `macro_rules!`, `Props`).

## Decisions

### D1. Значения атрибутов — строки
`stroke_width`, `fill` и `animation_duration` — `Prop<String>` с теми же значениями по умолчанию, что в Angular (`"2"`, `"none"`, `"2s"`): значения попадают в SVG/CSS как есть и совпадают с вводом демонстраций (`strokeWidth="8"`, `animationDuration=".5s"`). Числа не вводятся, чтобы не терять формы вроде `.5s`.

### D2. Ветви
Статическая ветвь — когда все свойства статические; разметка в `macro_rules!`, меняются только выражения атрибутов. `animation-duration` собирается в строку `style` у `svg` (`animation-duration: 2s`); `pt.spin` добавляет свой стиль через `merge_style`.

### D3. Атрибуты корня
`role="progressbar"` и `aria-busy="true"` статические; `aria-label` — `OptProp<String>`, не выводится при `None`. `data-p` нет: в `progressspinnerstyle.ts` его не задаёт.

### D4. SVG в `view!`
Элементы `svg` и `circle` из `leptos::svg` с атрибутами `viewBox`, `cx`, `cy`, `r`, `stroke-miterlimit`. Проверяется, что SSR-разметка совпадает с ожидаемой строкой (имя `viewBox` с заглавной буквой), и что на гидрации узлы совпадают (e2e).

## Risks / Trade-offs

- [Регистр `viewBox` при гидрации] → SSR-тест на подстроку `viewBox=`, e2e на отсутствие ошибок гидрации в консоли.
- [`pt.spin` и `animation-duration` делят `style`] → порядок: базовые значения, затем pt (как у остальных компонентов).

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
