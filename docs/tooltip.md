# Tooltip

Порт директивы `pTooltip` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-tooltip::tooltip` (директива), `TooltipOptions`, `TooltipContent`, `TooltipPosition`, `TooltipEvent` | `tooltip/tooltip.ts`, `tooltip/style/*` |
| CSS | `@primeuix/styles/tooltip` — `cargo xtask gen-theme` (блока `For PrimeNG` нет) |
| Демонстрации | `apps/showcase/doc/tooltip/{position,event,delay,autohide,options,custom}-doc.ts` |

Спека: `openspec/specs/tooltip`.

## Как пользоваться

В Leptos аналог `pTooltip` — `use:`-директива: она работает на любом элементе и на корне любого компонента проекта и не меняет разметку.

```rust
view! {
    <input use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Top) />
    <Button label="Save" use:tooltip=TooltipOptions::text("Confirm to proceed").show_delay(1000).hide_delay(300) />
    <InputText use:tooltip=TooltipOptions::text("Enter your username").event(TooltipEvent::Focus) />
}
```

`TooltipOptions::text_signal(signal)` делает текст реактивным, `.disabled(signal)` — отключение; `html(..)` и `view(..)` задают разметку и представление. Директиву вызывает Leptos в браузере при построении и гидратации элемента; на сервере она не вызывается.

## Что совпадает с PrimeNG

- Разметка: `div.p-tooltip.p-component.p-tooltip-<сторона>[role=tooltip]` в `body` > `div.p-tooltip-arrow` + `div.p-tooltip-text`; `id`, `style_class`, `position_style`, `fit_content`, `z_index`.
- События `Hover` (мышь и касание), `Focus`, `Both`; `click` скрывает; фокус слушается на `.p-component` внутри элемента, на `input` в `p-inputwrapper` или на самом элементе.
- Позиция: четыре стороны и тот же порядок переноса при нехватке места (`top`: top → bottom → right → left и т. д.), стрелка у края и по центру, смещения `position_top` и `position_left`.
- `show_delay`, `hide_delay`, `life`, `auto_hide` (при `false` подсказку можно навести мышью), `hide_on_escape`, `show_on_ellipsis`, `disabled`.
- Закрытие по Escape, `resize` окна и прокрутке вложенного прокручиваемого предка; слой группы подсказок (`ZGroup::Tooltip`).

## Отличия

| Отличие | Причина |
|---|---|
| Подсказка всегда в `body`; `appendTo` (`target`, элемент) нет | Так же у `Popover`, `Dialog`, `Drawer`. |
| Прокрутка самой страницы подсказку не скрывает | Подсказка лежит в координатах страницы и едет вместе с ней. В PrimeNG `ConnectedOverlayScrollHandler` слушает прокручиваемых предков, а не окно. |
| Появление — CSS-переход прозрачности 250 мс, а не JS-функция `fadeIn` | Результат тот же. |
| Параметры собираются строителем `TooltipOptions`, нет `tooltipOptions`-объекта и отдельных входов | Типизированный аналог входов `pTooltip`, `tooltipPosition`, `showDelay` и др. |
| Содержимое — `Text` / `Html` / `View`; нет `TemplateRef` и флага `escape` | `Html` — аналог `[escape]="false"`, `View` — шаблона. |
| Нет `pTooltipPT`, `pTooltipUnstyled`, `Bind` | `pt`-механизм PrimeNG не переносится. |
| Нет задержки 100 мс внутри `p-dialog` | Наш `Dialog` позиционирует окно до показа подсказки. |
| При повторном применении директивы (`rebuild`) слушатели перепривязываются по новым параметрам, а реактивные сигналы прежнего содержимого продолжают отслеживаться | Следствие того, что Leptos вызывает директиву при каждом построении атрибутов. |

## Стоимость

В SSR директива не вызывается, поэтому стоимость — около 1 аллокации на элемент (построение параметров); вклад в wasm ≈ 151 КБ (бюджет 170 КБ в `benches/budgets.toml`).

## Не проверено

Касание (`touchstart`/`touchend`, закрытие касанием вне подсказки при `auto_hide(false)`) на реальном вводе, `rtl` и экранные считыватели (`aria-describedby` не ставится, как и в PrimeNG). Проверены в Chrome (браузерные тесты и e2e с настоящей мышью и клавиатурой): показ и скрытие по hover, focus и both, `click`, четыре стороны и перенос на другую при нехватке места, смещения и опции вида, задержки и `life`, `auto_hide(false)`, Escape, `resize` и прокрутка вложенного предка, текст/HTML/`View`, реактивные текст и `disabled`, `show_on_ellipsis`, директива на корне `Button` и `InputText`, очистка при размонтировании и повторное применение к тому же элементу. Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось.
