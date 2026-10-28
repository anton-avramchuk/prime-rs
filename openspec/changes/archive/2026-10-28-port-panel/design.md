# Design

## Context

Требования — в `specs/panel` и `specs/collapsible`. Исходники PrimeNG 21.1.10: `packages/primeng/src/panel/panel.ts`, `panel/style/panelstyle.ts`, демонстрации `apps/showcase/doc/panel`. Fieldset (`fieldset/fieldset.ts`) использует ту же пару `pMotion="p-collapsible"` + ARIA-область, поэтому логика сворачивания общая. CSS анимации уже в `crates/prime-theme/css/base.css`: `.p-collapsible-enter-active` / `.p-collapsible-leave-active` запускают keyframes `p-animate-collapsible-expand|collapse` (меняют `grid-template-rows` `0fr`↔`1fr`, 0.2 s, `overflow: hidden`), CSS `panel` добавится через `cargo xtask gen-theme` (`upstream.toml`).

## Decisions

### D1. Общий модуль `prime-core::collapsible`
Не отдельный крейт: логика мала, не тянет зависимостей, кроме `leptos` и `web-sys` (уже есть у `prime-core`). Состав:
- `CollapseState { collapsed: Signal<bool>, toggle() }` — над внешним `Option<RwSignal<bool>>` или внутренним `RwSignal` из начального значения;
- `CollapseIds::new(id)` — `{id}_header`, `{id}_content`, `{id}-titlebar`; `id` — заданный пользователем либо уникальный из счётчика (в SSR и на клиенте детерминированно, как у уже существующих компонентов; сверить с практикой Checkbox/RadioButton при apply);
- чистые функции `aria_expanded`, `aria_hidden`, `region_tabindex` для атрибутов;
- `animate(el: &HtmlElement, expanding: bool, done: impl FnOnce())` — клиентская часть (D3), `cfg(target_arch = "wasm32")`, на SSR пустая.
Fieldset (следующий change) берёт этот модуль без изменений.

### D2. Двусторонний `collapsed`
Свойство `collapsed: Option<RwSignal<bool>>` (как `value` у InputText). Без сигнала — внутреннее состояние; начальное значение — отдельное свойство `default_collapsed: bool` (аналог одностороннего `[collapsed]` в Angular), чтобы не смешивать «сигнал» и «литерал» в одном типе. Класс корня и ARIA вычисляются от `collapsed` реактивно.

### D3. Анимация
Контейнер `p-panel-content-container` всегда в DOM. Состояния:
- развёрнут: обычное отображение (`display: grid` задаёт CSS темы);
- свёрнут: `display: none` через `hidden`-стиль (в SSR — сразу, без анимации).
На клиенте при смене состояния (не при первом рендере): раскрытие — снять `display:none`, добавить `p-collapsible-enter-active`; сворачивание — добавить `p-collapsible-leave-active`, после `animationend` снять класс и поставить `display:none`. `animationend` слушается один раз; запасной таймер (`set_timeout` 400 ms) вызывает завершение, если событие не пришло (`prefers-reduced-motion`, скрытая вкладка). Завершение вызывает `on_after_toggle(collapsed)`. Первый запуск эффекта пропускается флагом, чтобы монтирование не анимировалось.

### D4. Переключение по заголовку и кнопке
`toggler = header`: обработчик клика на шапке; кнопка в этом режиме клик тоже получает, но обработчик шапки срабатывает на всплытии, а у кнопки в режиме `header` собственного переключения нет — двойного вызова нет (в PrimeNG та же логика через проверку `toggler`). Enter/Space на кнопке: нативная `<button>` уже превращает их в `click`, поэтому отдельный `keydown` не нужен; по умолчанию браузера событие переключает единожды (в PrimeNG `keydown` нужен из-за `role=button` на `p-button`; проверяется e2e). `on_before_toggle` получает `MouseEvent` и прежнее `collapsed`.

### D5. Переключатель — собственная разметка кнопки
`Button` не подходит: `aria-expanded` меняется реактивно, а `ButtonPt` принимает только статические атрибуты. Поэтому `<button type="button">` рисует сам Panel с теми же классами и `data-p`, что у `p-button` PrimeNG: `p-button p-component p-button-icon-only p-button-secondary p-button-rounded p-button-text p-panel-toggle-button`, внутри `span.p-button-icon` с `MinusIcon`/`PlusIcon` (`prime-icons`). Зависимости от `prime-button` нет; `pt.toggle_button` применяется к этой кнопке.

### D6. Слоты
`ViewFn` вместо `ng-template`: `header_template`, `icons_template`, `header_icons_template` (`Callback<bool, AnyView>`, получает `collapsed`), `footer_template`. Содержимое — `children`. Вместо `<p-header>`/`<p-footer>` — только слоты.

### D7. Доступ к скрытому содержимому
`tabindex="-1"` у фокусируемых потомков: клиентский `Effect` при смене `collapsed` перебирает `input, button, select, a, textarea, [tabindex]` внутри `p-panel-content`, как PrimeNG. В SSR свёрнутая панель скрыта `display:none`, фокус недоступен и без этого.

### D8. Ветви и классы
Статическая ветвь — не `toggleable` и все свойства статические; иначе реактивная. Разметка — `macro_rules!`; `Arc` только в реактивной ветви. Порядок классов — как в `panelstyle.ts`. `data-p` — по `dataP` (`toggleable`), единым способом с `prime-core::data_p`.

## Risks / Trade-offs

- [Анимация `grid-template-rows` зависит от CSS темы] → проверяется e2e по смене класса и итоговой видимости, не по пикселям.
- [Двойное переключение при `toggler = header`] → отдельный сценарий в браузерном тесте.
- [Модуль в `prime-core` растёт] → в нём только логика сворачивания; при появлении Accordion выделяется, если потребуется.
- [`default_collapsed` расходится с Angular `[collapsed]`] → фиксируется в `docs/panel.md` в таблице отличий.

## Migration Plan

Миграции нет: новый крейт и модуль. Откат — `git revert`.

## Open Questions

Нет.
