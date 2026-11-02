# Toast

Порт `p-toast` и `MessageService` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-toast::Toast`, `ToastPt`, `ToastMessage`, `ToastSeverity`, `ToastPosition`, `MessageService` | `toast/toast.ts`, `toast/style/*`, `api/messageservice.ts`, `api/toastmessage.ts` |
| CSS | `@primeuix/styles/toast` — `cargo xtask gen-theme` (блока `For PrimeNG` нет; позиция контейнера — инлайн-стили из `toaststyle.ts`, их записывает сам компонент) |
| Демонстрации | `apps/showcase/doc/toast/{basic,severity,multiple,position,sticky,life,clear,target,responsive,template,headless}-doc.ts` |

Спека: `openspec/specs/toast`.

## Как пользоваться

Сервис кладётся в контекст один раз (в корне приложения или выше `Toast`), сообщения шлются из любого обработчика:

```rust
let service = provide_message_service();
view! {
    <Toast />
    <Button label="Show" on_click=Callback::new(move |_| {
        service.add(ToastMessage::new(ToastSeverity::Success, "Success", "Message Content"));
    }) />
}
```

В компонентах ниже по дереву сервис берётся `use_message_service()`. Адресация — `key`: `Toast key="a"` показывает только `ToastMessage::…key("a")`; `service.clear()` закрывает всё, `service.clear_key("a")` — только тосты с этим ключом.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-toast.p-component.p-toast-<положение>` > `div.p-toast-message.p-toast-message-<severity>[role=alert]` > `p-toast-message-content` (значок серьёзности — `Check`, `InfoCircle`, `TimesCircle`, `ExclamationTriangle`; `p-toast-message-text` с `p-toast-summary` и `p-toast-detail`; `button.p-toast-close-button`); `data-p`, `aria-live`, `aria-atomic`.
- Семь положений и инлайн-стили контейнера (`position: fixed`, 20px у краёв, 50% у центра).
- Ключи (оба без ключа — совпадение), `preventOpenDuplicates` и `preventDuplicates` с тем же сравнением (`summary`, `detail`, серьёзность).
- `life` (сообщения и `Toast`, по умолчанию 3000 мс), `sticky`, остановка таймера при наведении и повторный запуск при уходе курсора, закрытие кнопкой и `clear`, `on_close` после анимации выхода.
- Анимации `p-toast-message-enter-active` (300 мс) и `p-toast-message-leave-active` (250 мс), `z-index` группы модальных окон (`auto_z_index`, `base_z_index`) пока есть сообщения, `breakpoints` через `<style>` в `head`.

## Отличия

| Отличие | Причина |
|---|---|
| `MessageService` — `Copy`-хэндл в контексте Leptos (`provide_message_service` / `use_message_service`), подписчики вызываются синхронно | В Leptos нет внедрения зависимостей; шина событий заменена списком подписчиков. |
| `Toast` вне `provide_message_service()` паникует с понятным сообщением | Так же ведут себя части `Accordion` вне корня. |
| Нет `@primeuix/motion`, `motionOptions`, `show/hideTransformOptions`, `show/hideTransitionOptions` | Анимации — CSS-классы PrimeNG; запасной таймер (600 мс) убирает сообщение, если `animationend` не придёт. |
| Свой вид — `message` и `headless` (`Callback`), а не `pTemplate`/`ng-template`; `headless` получает функцию закрытия | Аналог `closeFn` PrimeNG. |
| У `ToastMessage` нет `data` и `text`; `severity` — `ToastSeverity` (есть `Custom(String)` для `severity: 'custom'`) | Типизированный аналог. |
| У кнопки закрытия нет атрибута `autofocus` | PrimeNG ставит `autofocus` на кнопку каждого сообщения; в Leptos он красть фокус не нужен. |
| `breakpoints` — пары «ширина — свойства CSS» (`Vec<(String, Vec<(String, String)>)>`) | Типизированный аналог объекта. |
| Демо `multiple` без `Ripple`, `headless` упрощена (карточка с двумя кнопками, без загрузки файла и градиента), `template` с инициалами вместо картинки | `Ripple` не переносится, нет загрузки файлов. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |

## Стоимость

31 аллокация на экземпляр при SSR (пустой `Toast`); вклад в wasm ≈ 330 КБ (бюджет 370 КБ в `benches/budgets.toml`).

## Не проверено

Касание, `rtl`, экранные считыватели, поведение при открытой на сервере странице с уже существующими сообщениями (сервер сообщений не отдаёт). Проверены в Chrome (браузерные тесты и e2e с настоящей мышью): добавление, разметка и все серьёзности, значки, закрытие кнопкой и анимация выхода, `life`/`sticky`/наведение, ключи, `clear`/`clear_key`, оба режима дубликатов, `add_all`, слой, `message`/`headless`, `breakpoints`, положения контейнера и отписка при размонтировании. Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось.
