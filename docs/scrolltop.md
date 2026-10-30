# ScrollTop

Порт `p-scrolltop` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-scrolltop::ScrollTop`, `ScrollTopTarget`, `ScrollTopBehavior`, `ScrollTopPt` | `scrolltop/scrolltop.ts`, `scrolltop/style/scrolltopstyle.ts` |
| CSS | `@primeuix/styles/scrolltop` — `cargo xtask gen-theme` |
| Демонстрации | `apps/showcase/doc/scrolltop/{basic,element}-doc.ts` |

Спека: `openspec/specs/scrolltop`.

## Что совпадает с PrimeNG

- Корень — кнопка `Button` с классами `p-scrolltop` (и `p-scrolltop-sticky` для `Parent`), по умолчанию скруглённая; значок — `ChevronUpIcon` с классом `p-scrolltop-icon`, `aria-label` из локали.
- Кнопка показывается, когда прокрутка строго больше порога (по умолчанию 400); `Window` — `position: fixed`, `Parent` — `sticky`.
- Показ и скрытие анимируются классами `p-scrolltop-enter-*` / `leave-*` из темы (300 мс); z-index выдаётся из группы `Overlay` при показе и возвращается при скрытии.
- Клик прокручивает окно или родителя к началу (`top = 0`, поведение `Smooth` по умолчанию).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<button>` без хоста `<p-scrolltop>` | Хост в Angular — деталь фреймворка. |
| Начальная проверка при монтировании | В PrimeNG, пока не придёт первое `scroll`, кнопки нет, даже если страница уже прокручена. |
| При `Window` кнопки нет в DOM, пока она скрыта; при `Parent` она всегда в DOM со `display: none` | Родителя можно найти только по элементу самого компонента. |
| Анимация — классами и таймером 300 мс | Нет `@primeuix/motion` / `pMotion`. |
| Нет `buttonProps` и `icon` как имени класса | `severity`, `raised`, `rounded` — свойства; значок — `ViewFn`. |

## Стоимость

29 аллокаций на экземпляр при SSR (в SSR `Window`-вариант скрыт, поэтому измеряется «пустой» экземпляр), вклад в wasm ≈ 542 КБ (в основном общий `Button`; бюджет 600 КБ в `benches/budgets.toml`).

## Не проверено

Касание и инерционная прокрутка на мобильных устройствах, экранные считыватели. Проверены порог, положение, клик и клавиши `Enter`/`Space` в Chrome (e2e и браузерные тесты).
