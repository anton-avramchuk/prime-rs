# FileUpload

Порт `p-fileupload` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-fileupload::FileUpload`, `FileUploadPt`, `FileUploadRef`, `FileUploadMode`, `UploadEvent`, `UploadError` | `packages/primeng/src/fileupload/fileupload.ts` (`FileUpload`, `FileContent`), `fileupload/style/fileuploadstyle.ts` |
| Проверка типа/размера, дубликаты, лимит, размеры, подписи `basic` | `prime-fileupload::files` — чистый порт `validate`, `isFileTypeValid`, `isFileLimitExceeded`, `formatSize`, `basicFileChosenLabel`, табличные тесты |
| CSS | `@primeuix/styles/fileupload` (`fileupload` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/fileupload/*-doc.ts` (`basic`, `advanced`, `auto`, `custom`) |

Спека: `openspec/specs/fileupload`. Выбранные и загруженные файлы доступны через `FileUploadRef` (`files()`, `uploaded_files()`, `choose()`, `upload()`, `clear()`).

```rust
view! { <FileUpload name="demo[]" url="/api/upload" multiple=true accept="image/*" max_file_size=1_000_000 on_upload=Callback::new(|e: UploadEvent| log(e.status)) /> }
```

## Что совпадает с PrimeNG

- Проверки: `accept` (MIME, `тип/*`, расширение без учёта регистра), `max_file_size`, дубликаты по имени, типу и размеру, `file_limit` со своими сообщениями; шаблоны сообщений настраиваются (`invalid_file_*_message_*`).
- Разметка и классы `advanced` и `basic`, значки «Pending»/«Completed», превью изображений (`blob:`), подсветка `p-fileupload-highlight` при перетаскивании.
- Отправка `FormData` через `XMLHttpRequest` с заголовками, `with_credentials`, прогрессом; `custom_upload` с обработчиком; `auto`.
- События: `on_select`, `on_before_upload`, `on_send`, `on_progress`, `on_upload`, `on_error`, `on_clear`, `on_remove`, `on_remove_uploaded_file`.

## Отличия

| Отличие | Причина |
|---|---|
| Поле `input[type=file]` одно (рядом с кнопкой), выбор — `click()` по нему | В PrimeNG их два (корневое и внутри кнопки); выбор тот же. |
| Превью выводится только для изображений (`<img>` у остальных не создаётся) | В PrimeNG `<img>` без `src` у не-изображений. |
| Ответ не 2xx и сетевая ошибка вызывают `on_error`, файлы остаются в списке | Так ведёт себя `HttpClient`; ветка «не 2xx» внутри `Response` в PrimeNG недостижима. |
| Нет шаблонов `file`, `content`, `fileLabel`, `chooseicon` и т. п., вместо них — `header` и `empty`; нет `chooseButtonProps` | В Leptos нет `ng-template`; кнопки настраиваются классами и метками. |
| `Space`/`Enter` на кнопке `basic` обрабатывает сама кнопка (`click`), а не отдельный `keydown` | Нативное поведение `<button>`. |
| Нет IE11, `Bind`, форм Angular, `getBlockableElement` | Вне объёма порта. |
| Замыкания `XMLHttpRequest` освобождаются через `forget` (один небольшой блок на загрузку) | Простота; слушатели привязаны к одноразовому объекту запроса. |

## Что не проверялось

- Загрузка на реальный сервер с успешным ответом в браузерных тестах (там — ошибка сети); успех проверяется в e2e с подменой ответа.
- Выбор файлов системным диалогом и перетаскивание из ОС (эмуляция событий).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`fileupload-1`, `fileupload-1000`, `case-fileupload`) и `benches/*budgets.toml`.
