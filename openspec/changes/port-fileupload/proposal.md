# Proposal

## Why

FileUpload — загрузка файлов: выбор кнопкой или перетаскиванием, проверка типа/размера/числа, список с превью, индикатор прогресса, отправка на сервер или собственный обработчик. Третий компонент волны 6. Проверки и подписи — чистая логика, оболочка строится на готовых Button, Message, ProgressBar и Badge.

## What Changes

- Новый крейт `prime-fileupload` (порт `p-fileupload` из PrimeNG 21.1.10).
- Чистый модуль `files.rs`: `FileInfo`, `format_size`, проверка типа по `accept` (MIME, подстановка `image/*`, расширение), размера, дубликатов, `file_limit`, подпись режима `basic`.
- `FileUpload` в режимах `advanced` и `basic`: `div.p-fileupload.p-fileupload-<mode>.p-component`; в `advanced` — скрытое поле `input[type=file]`, `.p-fileupload-header` (кнопки выбора, загрузки, отмены), `.p-fileupload-content` (индикатор прогресса, сообщения об ошибках, списки ожидающих и загруженных файлов с превью изображений, размером, значком «Pending»/«Completed» и кнопкой удаления); в `basic` — кнопка выбора и подпись выбранного.
- Поведение: выбор через поле, перетаскивание на содержимое (подсветка `p-fileupload-highlight`), `multiple`, `accept`, `max_file_size`, `file_limit`, `auto`, `custom_upload` (обработчик вместо отправки), отправка через `XMLHttpRequest` с `FormData`, заголовками, `with_credentials`, прогрессом; события `on_select`, `on_before_upload`, `on_send`, `on_progress`, `on_upload`, `on_error`, `on_clear`, `on_remove`, `on_remove_uploaded_file`; `FileUploadRef` (`clear`, `upload`, `choose`).
- Страница `/fileupload` (`basic`, `advanced`, `auto`, `custom`), e2e, бенчмарки и бюджеты, `docs/fileupload.md`, `fileupload` в `theme.components`.
- **Не входит:** шаблоны файла и содержимого Angular, IE11, `Bind`, формы Angular, `getBlockableElement`, общие свойства кнопок `chooseButtonProps` (заменены `choose_style_class`).

## Capabilities

### New Capabilities
- `fileupload`: выбор, проверка и загрузка файлов.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-fileupload` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-button`, `prime-message`, `prime-progressbar`, `prime-badge`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
