# BlockUI

Порт `p-blockui` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-blockui::BlockUI`, `BlockTarget` | `packages/primeng/src/blockui/blockui.ts`, `blockui/style/blockuistyle.ts` |
| CSS | `@primeuix/styles/blockui` и общие правила `p-overlay-mask*` из `base.css` — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/blockui/{basic,document}-doc.ts` |

Спека: `openspec/specs/blockui`.

## Что совпадает с PrimeNG

- Классы маски: `p-blockui p-blockui-mask`, у документа `p-blockui-mask-document`, при показе `p-overlay-mask p-overlay-mask-enter-active`, при снятии `p-overlay-mask-leave-active`; инлайн `display: flex`; `aria-busy` — `true` при показе и `false` при выходе.
- Цель (`target`): маска — последний ребёнок целевого элемента, ему на время показа ставится `position: relative`; тема растягивает маску (`position: absolute`). Без цели маска лежит в `body` (`position: fixed`), прокрутка страницы блокируется через `prime_dom::block_body_scroll` (общий счётчик с Dialog и Drawer).
- Слой: при `auto_z_index` (по умолчанию да) маска получает `z-index` группы `Modal` со сдвигом `base_z_index`. Как и в PrimeNG (`ZIndexUtils`), стек слоёв общий: новый слой выше предыдущего, а базовое значение учитывается, только когда выше нет других оверлеев.
- Выход: после снятия блокировки играет анимация, по `animationend` (либо запасному таймеру 400 мс) маска убирается, слой и блокировка прокрутки освобождаются; повторное включение во время выхода возвращает маску в состояние «заблокировано».

## Отличия

| Отличие | Причина |
|---|---|
| Пока `blocked = false`, в DOM ничего нет; маска выводится только в браузере (`Portal`) и до гидратации не отдаётся сервером | В PrimeNG хост остаётся в разметке (с `display: none` после первого снятия); здесь как у Drawer и Dialog. `aria-busy` тоже существует только на показанной маске. |
| `target` — `BlockTarget` из `NodeRef` любого элемента вместо `BlockableUI` (`getBlockableElement()`) | В Rust нет интерфейсов Angular; цель — элемент на странице. |
| Инлайн-значение `position` у цели возвращается после выхода маски | PrimeNG оставляет `position: relative` навсегда. |
| Содержимое — `children` (`ChildrenFn`, строится заново при каждом показе) вместо шаблона `content` | Нет `ng-template`. |
| Если `target` задан, но его элемента ещё нет в DOM, маска не выводится | PrimeNG бросает исключение. Для начальной блокировки цели `blocked` нужно включать после монтирования. |
| Нет `Bind`, `pTemplate`; `pt` — один `Pt` на корень маски | `pt` — простые добавки классов, стиля и атрибутов. |

## Что не проверялось

- Блокировка фокуса: маска закрывает указатель, но не перехватывает клавиатуру — как и в PrimeNG, Tab может уйти на элементы под маской.
- Несколько одновременно включённых масок на разных целях: слои и счётчик прокрутки проверены на одном и последовательных сценариях.

## Производительность

См. `benches/` (`blockui-1`, `blockui-1000`, `case-blockui`) и `benches/*budgets.toml`: на сервере маска не выводится, измеряется только обвязка (28 аллокаций на экземпляр).
