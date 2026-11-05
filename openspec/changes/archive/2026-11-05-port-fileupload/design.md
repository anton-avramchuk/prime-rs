# Design

## Context

Требования — в `specs/fileupload`. Исходники PrimeNG 21.1.10: `fileupload/fileupload.ts` (`FileUpload`, `FileContent`), `style/fileuploadstyle.ts`, `@primeuix/styles/fileupload`. Образцы: `prime-button`, `prime-message`, `prime-progressbar`, `prime-badge` (готовые части), `prime-drawer`/`prime-colorpicker` (структура крейта, тесты).

## Goals / Non-Goals

**Goals:** тот же DOM, те же проверки и подписи, реальная отправка через XHR.

**Non-Goals:** шаблоны Angular (`file`, `content`), IE11, `Bind`, `getBlockableElement`.

## Decisions

### D1. Чистая логика (`files.rs`)
`FileInfo {name, mime, size}`; `format_size(bytes, units)`, `is_type_valid(info, accept)`, `validate(info, accept, max_size) -> Result<(), Rejection>`, `is_duplicate`, `limit_exceeded(auto, selected, uploaded, limit)`, `choose_disabled`, `basic_label`. Табличные тесты на формулах TS.

### D2. Состояние
`FileUploadRef` (Copy): `files`, `uploaded` — `RwSignal<Vec<Entry>, LocalStorage>` (`Entry {file: web_sys::File, info, object_url}`), `messages`, `progress`, `uploading`, `uploaded_count`, `highlight`, ссылка на `input`. Методы `clear`, `upload`, `choose`.

### D3. Отправка
`XmlHttpRequest` + `FormData`; замыкания `progress`/`load`/`error` — `Closure::forget` (одна утечка на загрузку). Ответ с кодом 2xx — `on_upload`, иначе `on_error`; после ответа `uploaded += files`, `files.clear`. Объекты URL изображений освобождаются при удалении/очистке и размонтировании.

### D4. Выбор и перетаскивание
`change` поля и `drop` содержимого читают `FileList`; значение поля сбрасывается после обработки. `dragover` ставит подсветку и `preventDefault`.

### D5. Тесты
Логика — таблицы. SSR: разметка режимов, состояния кнопок, `auto`. Браузерные `tests/web.rs` одним последовательным тестом: файл подставляется в поле через `DataTransfer`; выбор, валидация, лимит, удаление, `clear`, `custom_upload`, отправка на несуществующий адрес (ошибка 404/сеть), drop. e2e: `setInputFiles`, мок ответа сервера `page.route`.

## Risks / Trade-offs

- [Перетаскивание] Событие `drop` в e2e эмулируется `DataTransfer`.
- [Утечка замыканий XHR] Одно небольшое замыкание на загрузку.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
