# Design

## Context

Требования — в `specs/toast`. Исходники PrimeNG 21.1.10: `toast/toast.ts`, `toast/style/toaststyle.ts`, `api/messageservice.ts`, `api/toastmessage.ts`; демонстрации `apps/showcase/doc/toast/*-doc.ts`. Образцы в проекте: `prime-message` (серьёзности, значки, `data-p`), `prime-dialog` (анимация выхода по `animationend` с запасным таймером, `<style>` с `@media` в `head`, слои `ZGroup::Modal`), `prime-core::zindex`.

Наблюдения по исходнику:
- Toast — два уровня: `Toast` (контейнер, подписки на сервис, список сообщений) и `ToastItem` (одно сообщение: таймер, видимость, анимация, закрытие).
- `MessageService` — две шины (`messageObserver`, `clearObserver`); `Toast` подписывается в `onInit`. `canAdd`: `this.key === message.key` (оба `undefined` совпадают), затем `preventOpenDuplicates` (по списку открытых) и `preventDuplicates` (по архиву всех когда-либо добавленных). Сравнение дубликатов — `summary`, `detail`, `severity`.
- `ToastItem`: `sticky` → видим без таймера; иначе `visible=true` и `setTimeout(life || toast.life || 3000)`; `mouseenter` гасит таймер, `mouseleave` запускает заново (если не идёт закрытие); `clearAll` и кнопка закрытия ставят `visible=false`; после выхода (`onAfterLeave`) сообщение удаляется из списка и вызывается `onClose`.
- Контейнер позиционируется инлайн-стилями из `toaststyle.ts` (`position: fixed`, 20px у краёв, 50% у центра) — в CSS темы их нет.
- Анимация сообщения: `pMotionName="p-toast-message"` → классы `p-toast-message-enter-active` / `-leave-active`; есть ли для них `@keyframes` в теме — проверяется при генерации.
- Контейнеру `z-index` ставится при первой анимации (`ZIndexUtils.set('modal', …)`), очищается, когда сообщений не осталось.
- Адаптивность: `<style>` в `head` с правилами `.p-toast[<uuid>] { … !important }` по `breakpoints`.

## Goals / Non-Goals

**Goals:** тот же DOM и поведение таймеров, ключей, дубликатов и закрытия; сервис, доступный из любого обработчика приложения.

**Non-Goals:** `@primeuix/motion`, `pTemplate`, `data`/`text`, `Ripple`, `Bind`.

## Decisions

### D1. Крейт
`prime-toast` с `Toast`, `ToastMessage`, `ToastSeverity`, `ToastPosition`, `MessageService`, `ToastPt`. Зависит от `prime-core`, `prime-dom`, `prime-icons`, `web-sys`.

### D2. Сервис
`MessageService` — `Copy`-хэндл на `StoredValue<Vec<(usize, Callback<ServiceEvent>)>>` (подписчики) и счётчик подписок. `provide_message_service() -> MessageService` создаёт и кладёт в контекст; `use_message_service()` берёт из контекста и паникует с понятным сообщением вне него (как части `Accordion`). События: `ServiceEvent::Add(Vec<ToastMessage>)` и `ServiceEvent::Clear(Option<String>)`. `subscribe(callback) -> usize`, `unsubscribe(id)`; `Toast` подписывается в теле компонента (в браузере и на сервере — на сервере события не приходят) и отписывается в `on_cleanup`. Вызовы подписчиков идут синхронно, вне реактивного отслеживания.

### D3. Чистая логика (`state.rs`)
- `ToastSeverity`, `ToastPosition`, `severity_class`, `message_classes(severity)`, `root_classes(position, style_class)`.
- `position_style(position) -> String` — инлайн-стили контейнера.
- `matches_key(toast_key, message_key) -> bool`.
- `contains_message(collection, message) -> bool` (по `summary`, `detail`, `severity`).
- `can_add(toast_key, message, open, archive, prevent_open, prevent_all) -> bool`.
- `lifetime(message_life, toast_life) -> u32` (`message || toast || 3000`, нулевые не считаются заданными).
- `breakpoints_css(attr, &[(bp, &[(prop, value)])])`.
Табличные тесты на каждую.

### D4. Элемент списка
Сообщение — внутренний `Item { key: u64, message: ToastMessage, visible: RwSignal<bool>, closing: StoredValue<bool> }`; список — `RwSignal<Vec<Item>>`. Рендер — `<For>` по `key` с компонентом `ToastItem`. Таймер — `set_timeout_with_handle` в `StoredValue<Option<TimeoutHandle>, LocalStorage>`; снимается при `mouseenter`, закрытии и размонтировании; запасной таймер выхода (700 мс) на случай, когда `animationend` не придёт. По окончании выхода элемент удаляется из списка, вызывается `on_close`, а при пустом списке освобождается `z-index`.

### D5. `clear`
`clear()` и `clear_key(k)` ставят `visible=false` всем (подходящим) элементам: они плавно уходят, как в PrimeNG (`clearAllTrigger`).

### D6. Слой
При первом появлении элемента в пустом списке: `z-index` контейнера = `zindex::generate(ZGroup::Modal, base + base_z_index)`, при опустошении — `revert`. Контейнер может уже иметь `z-index` (если `auto_z_index` выключен — не трогаем).

### D7. Представления
`message: Option<Callback<ToastMessage, AnyView>>` заменяет значок и тексты (но не кнопку закрытия); `headless: Option<Callback<(ToastMessage, Callback<()>), AnyView>>` заменяет всё внутри сообщения и получает функцию закрытия.

### D8. CSS
`toast` добавляется в `theme.components`; блока `For PrimeNG` нет (позиция — инлайн-стили, D3). Проверить, что классы `p-toast-message-*`, значки и анимации в CSS есть.

### D9. Тесты
SSR: пустой контейнер, положения, `data-p`, `style_class`, сервис без браузера. Браузерные `tests/web.rs` (один последовательный прогон — таймеры и контекст общие): сообщения и разметка, серьёзности и значки, кнопка закрытия и анимация выхода, `life` и `sticky`, наведение, ключи, `clear`/`clear_key`, дубликаты (оба режима), несколько сообщений сразу, `message`/`headless`, `breakpoints`, слой, `on_close`, отписка при размонтировании. e2e: настоящая мышь. Крейт добавляется в три места `ci.yml`.

### D10. Бенчмарки
Сценарии `toast-1`/`-1000` (пустой `Toast`) и `case-toast`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке).

## Risks / Trade-offs

- [Сервис в контексте] `Toast` вне `provide_message_service()` паникует с понятным сообщением; в showcase сервис кладётся на уровне оболочки приложения.
- [Анимация] если `@keyframes` для `p-toast-message-*` нет, остаётся запасной таймер; решается при генерации темы.
- [Запасной таймер] при анимации длиннее таймера сообщение исчезнет раньше времени; 700 мс при анимации ≈ 300 мс.

## Open Questions

- Закрыт: в `@primeuix/styles/toast` есть `.p-toast-message-enter-active` (300 мс) и `.p-toast-message-leave-active` (250 мс) с `@keyframes`.

## Migration Plan

Миграции нет. Откат — `git revert`.
