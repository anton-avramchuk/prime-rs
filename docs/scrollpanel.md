# ScrollPanel

Порт `p-scrollpanel` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-scrollpanel::ScrollPanel` | `packages/primeng/src/scrollpanel/scrollpanel.ts`, `scrollpanel/style/scrollpanelstyle.ts` |
| CSS | `@primeuix/styles/scrollpanel` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/scrollpanel/*-doc.ts` (`basic`, `custom`) |

Спека: `openspec/specs/scrollpanel`.

## Что совпадает с PrimeNG

- Разметка и классы: `p-scrollpanel p-component` → `p-scrollpanel-content-container` → `p-scrollpanel-content`, затем полосы `p-scrollpanel-bar p-scrollpanel-bar-x` и `p-scrollpanel-bar-y` с `role="scrollbar"`, `tabindex="0"`, `aria-orientation`, `aria-valuenow`, `aria-controls`, `data-pc-group-section="bar"`.
- Формулы размера и положения полос (длина не меньше 10%, смещение пропорционально прокрутке), атрибут `data-p-scrollpanel-hidden` и класс `p-scrollpanel-hidden`, расчёт высоты корня при `max-height`.
- Пересчёт на `scroll`, `mouseenter` содержимого и `resize` окна; перетаскивание полос с `p-scrollpanel-grabbed` на полосе и `body`.
- Клавиатура: стрелки на сфокусированной полосе прокручивают на `step` через 40 мс, `keyup` отменяет таймер, то есть клавишу нужно удерживать (в Playwright — `keyboard.down`, а не `press`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-scrollpanel>`; правило `.p-scrollpanel { display: block }` из `scrollpanelstyle.ts` не нужно | `div` уже блочный; хост в Angular — деталь фреймворка. |
| Содержимое — `children` или слот `content_template`, взаимоисключающие | Вместо `<ng-content>` и шаблона `#content`; в Angular при шаблоне `<ng-content>` скрыт тем же `@if`. |
| Методы `refresh()` и `scroll_top(value)` — через ручку `ScrollPanelRef` (свойство `panel_ref`) | Нет `@ViewChild`. |
| При перетаскивании горизонтальной полосы `data-p-scrollpanel-grabbed="true"` | В PrimeNG там `"false"` (опечатка в `onXBarMouseDown`), класс `p-scrollpanel-grabbed` ставится в обоих случаях. |
| `aria-valuenow` обновляется напрямую в DOM из обработчика `scroll` | Без change detection; значение то же. |
| `style` полос ведёт клиентский пересчёт; `pt.bar_*.style` применяется только при первой отрисовке и затем затирается | Иначе Leptos и `moveBar` перезаписывали бы друг друга. Задавайте внешний вид полос классами. |
| Высота корня при `max-height` фиксируется один раз после монтирования; реактивный `style` с `max-height` перезапишет её | Как и в PrimeNG, расчёт разовый; сигнал, меняющий `style`, возвращает значение из `style`. |
| Свойство `step` читается в момент нажатия клавиши, а не при отрисовке | Допускает сигнал без перерисовки. |
| Нет `BlockableUI`, синонимов шаблонов, `appendTo` | Интерфейс и устаревшие имена Angular. |
| Положение при перетаскивании округляется до целых пикселей | `scrollTop` в `web-sys` без `unstable_apis` — `i32`. |

## Производительность

SSR: 26 аллокаций на экземпляр с одним абзацем внутри (бюджет 29); вклад в сырой wasm +184 КБ (бюджет 200 КБ). Заметная часть — клиентский код: форматирование `f64` для `style` полос, слушатели `web-sys` и `getComputedStyle`; на сервере его нет.
Шаблон разметки — как у Toolbar (`docs/toolbar.md`), клиентская часть — `crates/prime-scrollpanel/src/client.rs`.

## Тесты

SSR — `tests/ssr.rs`; браузерные — `tests/web.rs` (`cargo test -p prime-scrollpanel --target wasm32-unknown-unknown --test web`, нужны `wasm-bindgen-cli` и `chromedriver`); e2e — `e2e/tests/scrollpanel.spec.ts`. В браузерных тестах CSS темы не подключается: нужное для прокрутки минимум-правил задано в тесте.
