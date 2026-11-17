# Design

## Context

Requirements are in `specs/toast`. PrimeNG 21.1.10 sources: `toast/toast.ts`, `toast/style/toaststyle.ts`, `api/messageservice.ts`, `api/toastmessage.ts`; demos `apps/showcase/doc/toast/*-doc.ts`. Reference implementations in the project: `prime-message` (severities, icons, `data-p`), `prime-dialog` (exit animation via `animationend` with a fallback timer, `<style>` with `@media` in `head`, `ZGroup::Modal` layers), `prime-core::zindex`.

Observations about the source:
- Toast has two levels: `Toast` (container, service subscriptions, message list) and `ToastItem` (a single message: timer, visibility, animation, closing).
- `MessageService` has two buses (`messageObserver`, `clearObserver`); `Toast` subscribes in `onInit`. `canAdd`: `this.key === message.key` (both `undefined` match), then `preventOpenDuplicates` (against the open list) and `preventDuplicates` (against the archive of everything ever added). Duplicate comparison — `summary`, `detail`, `severity`.
- `ToastItem`: `sticky` → visible without a timer; otherwise `visible=true` and `setTimeout(life || toast.life || 3000)`; `mouseenter` stops the timer, `mouseleave` restarts it (unless closing is in progress); `clearAll` and the close button set `visible=false`; after exit (`onAfterLeave`) the message is removed from the list and `onClose` is called.
- The container is positioned with inline styles from `toaststyle.ts` (`position: fixed`, 20px at the edges, 50% at the center) — they are not in the theme CSS.
- Message animation: `pMotionName="p-toast-message"` → classes `p-toast-message-enter-active` / `-leave-active`; whether the theme has `@keyframes` for them is checked during generation.
- The container `z-index` is set on the first animation (`ZIndexUtils.set('modal', …)`) and cleared when no messages are left.
- Responsiveness: a `<style>` in `head` with rules `.p-toast[<uuid>] { … !important }` per `breakpoints`.

## Goals / Non-Goals

**Goals:** the same DOM and behavior of timers, keys, duplicates and closing; a service available from any app handler.

**Non-Goals:** `@primeuix/motion`, `pTemplate`, `data`/`text`, `Ripple`, `Bind`.

## Decisions

### D1. Crate
`prime-toast` with `Toast`, `ToastMessage`, `ToastSeverity`, `ToastPosition`, `MessageService`, `ToastPt`. Depends on `prime-core`, `prime-dom`, `prime-icons`, `web-sys`.

### D2. Service
`MessageService` is a `Copy` handle over `StoredValue<Vec<(usize, Callback<ServiceEvent>)>>` (subscribers) and a subscription counter. `provide_message_service() -> MessageService` creates it and puts it into context; `use_message_service()` takes it from context and panics with a clear message outside it (like the `Accordion` parts). Events: `ServiceEvent::Add(Vec<ToastMessage>)` and `ServiceEvent::Clear(Option<String>)`. `subscribe(callback) -> usize`, `unsubscribe(id)`; `Toast` subscribes in the component body (in the browser and on the server — on the server no events arrive) and unsubscribes in `on_cleanup`. Subscribers are called synchronously, outside reactive tracking.

### D3. Pure logic (`state.rs`)
- `ToastSeverity`, `ToastPosition`, `severity_class`, `message_classes(severity)`, `root_classes(position, style_class)`.
- `position_style(position) -> String` — container inline styles.
- `matches_key(toast_key, message_key) -> bool`.
- `contains_message(collection, message) -> bool` (by `summary`, `detail`, `severity`).
- `can_add(toast_key, message, open, archive, prevent_open, prevent_all) -> bool`.
- `lifetime(message_life, toast_life) -> u32` (`message || toast || 3000`, zeros do not count as set).
- `breakpoints_css(attr, &[(bp, &[(prop, value)])])`.
Table-driven tests for each.

### D4. List item
A message is an internal `Item { key: u64, message: ToastMessage, visible: RwSignal<bool>, closing: StoredValue<bool> }`; the list is `RwSignal<Vec<Item>>`. Rendering — `<For>` by `key` with a `ToastItem` component. The timer is `set_timeout_with_handle` in `StoredValue<Option<TimeoutHandle>, LocalStorage>`; it is cleared on `mouseenter`, on closing and on unmount; a fallback exit timer (700 ms) covers the case when `animationend` never fires. When exit finishes the item is removed from the list, `on_close` is called, and when the list is empty the `z-index` is released.

### D5. `clear`
`clear()` and `clear_key(k)` set `visible=false` on all (matching) items: they leave smoothly, as in PrimeNG (`clearAllTrigger`).

### D6. Layer
When the first item appears in an empty list: container `z-index` = `zindex::generate(ZGroup::Modal, base + base_z_index)`, on emptying — `revert`. The container may already have a `z-index` (if `auto_z_index` is off, we do not touch it).

### D7. Views
`message: Option<Callback<ToastMessage, AnyView>>` replaces the icon and texts (but not the close button); `headless: Option<Callback<(ToastMessage, Callback<()>), AnyView>>` replaces everything inside the message and receives the close function.

### D8. CSS
`toast` is added to `theme.components`; there is no `For PrimeNG` block (position — inline styles, D3). Check that the `p-toast-message-*` classes, icons and animations are present in the CSS.

### D9. Tests
SSR: empty container, positions, `data-p`, `style_class`, the service without a browser. Browser `tests/web.rs` (one sequential run — timers and context are shared): messages and markup, severities and icons, close button and exit animation, `life` and `sticky`, hover, keys, `clear`/`clear_key`, duplicates (both modes), several messages at once, `message`/`headless`, `breakpoints`, layer, `on_close`, unsubscribe on unmount. e2e: a real mouse. The crate is added in three places of `ci.yml`.

### D10. Benchmarks
Scenarios `toast-1`/`-1000` (empty `Toast`) and `case-toast`; first `cargo bench -p render-bench --bench alloc`, for wasm — a temporary large budget (replace by a unique line).

## Risks / Trade-offs

- [Service in context] `Toast` outside `provide_message_service()` panics with a clear message; in the showcase the service is put at the app shell level.
- [Animation] if there are no `@keyframes` for `p-toast-message-*`, the fallback timer remains; decided during theme generation.
- [Fallback timer] if the animation is longer than the timer, the message disappears early; 700 ms against an animation of ≈ 300 ms.

## Open Questions

- Closed: `@primeuix/styles/toast` has `.p-toast-message-enter-active` (300 ms) and `.p-toast-message-leave-active` (250 ms) with `@keyframes`.

## Migration Plan

No migration. Rollback — `git revert`.
