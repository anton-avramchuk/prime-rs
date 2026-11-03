# Confirmation и ConfirmationService

Порт `api/confirmation.ts` и `api/confirmationservice.ts` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Крейт `prime-confirmation` без компонентов и стилей; им пользуются `ConfirmPopup` (`docs/confirmpopup.md`) и позднее `ConfirmDialog`.

| Наш тип | Исходники PrimeNG |
|---|---|
| `Confirmation`, `ConfirmButtonProps`, `ConfirmFocus` | `packages/primeng/src/api/confirmation.ts` |
| `ConfirmationService`, `ConfirmEvent`, `provide_confirmation_service`, `use_confirmation_service` | `packages/primeng/src/api/confirmationservice.ts` |

Спека: `openspec/specs/confirmation`.

Сервис кладётся в контекст один раз (в корне приложения или выше компонента), подтверждения шлются из любого обработчика:

```rust
let service = provide_confirmation_service();
service.confirm(Confirmation::new().key("delete").message("Sure?").accept(Callback::new(|()| { /* ... */ })));
service.close();
```

## Что совпадает с PrimeNG

- Все поля `Confirmation`: `key`, `message`, `header`, `icon`, `target`, `accept`, `reject`, подписи и значки кнопок, `accept_visible`/`reject_visible`, классы и свойства кнопок, `default_focus`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal`. Не заданные поля — `None`; значения по умолчанию (кнопки видимы, закрытие по `Escape` и клику) применяет компонент, как и в PrimeNG (`!== false`).
- `confirm` и `close` доходят до всех подписчиков в порядке подписки; подтверждение показывает компонент с тем же `key`.

## Отличия

| Отличие | Причина |
|---|---|
| `ConfirmationService` — `Copy`-хэндл в контексте Leptos (`provide_confirmation_service` / `use_confirmation_service`), подписчики вызываются синхронно | В Leptos нет внедрения зависимостей; `Subject` заменён списком подписчиков (как `MessageService`). |
| `accept` и `reject` — `Callback<()>`, а не `Function` с эмиттерами `acceptEvent`/`rejectEvent` | Нет `EventEmitter`. |
| `acceptButtonProps` и `rejectButtonProps` — структура `ConfirmButtonProps` (`label`, `severity`, `outlined`, `text`, `size`), а не произвольный объект | Типизированное подмножество свойств `Button`, нужное компонентам. |
| Нет `closeButtonProps` и `ConfirmationService.onAccept` | Относятся к `ConfirmDialog`; добавятся вместе с ним. |
| Значки — `ViewFn` | Нет PrimeIcons. |
| `target` — `HtmlElement` (из события — `Confirmation::target_event`) | `EventTarget` в PrimeNG приводится к элементу. |

## Что не проверялось

- Одновременная работа нескольких сервисов в разных поддеревьях контекста.
