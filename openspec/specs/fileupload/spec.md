# fileupload Specification

## Purpose
FileUpload — выбор, проверка и загрузка файлов.

## Requirements

### Requirement: Проверка файлов
Файл SHALL приниматься, если он проходит `accept` (список через запятую: MIME-тип точно, шаблон `тип/*` по типу, либо расширение `.ext` без учёта регистра), не превышает `max_file_size` и ещё не выбран (совпадение имени, типа и размера). Отклонённый файл SHALL давать сообщение `error` из шаблонов «{0}: Invalid file type, allowed file types: {1}.» / «{0}: Invalid file size, maximum upload size is {1}.».

#### Scenario: Тип
- **WHEN** `accept = "image/*"`, файл `a.pdf` (`application/pdf`)
- **THEN** файл отклонён, сообщение «a.pdf: Invalid file type, allowed file types: image/*.»

#### Scenario: Расширение
- **WHEN** `accept = ".PDF"`, файл `a.pdf`
- **THEN** файл принят

#### Scenario: Размер
- **WHEN** `max_file_size = 1000`, файл 2048 байт
- **THEN** отклонён, в сообщении «1000.000 B»

### Requirement: Размер и подписи
`format_size` SHALL выводить размер как `{число с тремя знаками} {единица}` с единицами B, KB, MB, …, основание 1024; `0` — «0 B». В режиме `basic` подпись SHALL быть: метка выбора при `auto`; имя файла при одном файле; «Files» при нескольких (шаблон `fileChosenMessage`); «No file chosen» без файлов.

#### Scenario: Единицы
- **WHEN** 1536 байт
- **THEN** «1.500 KB»

### Requirement: Разметка
`FileUpload` SHALL выводить `div.p-fileupload.p-fileupload-<mode>.p-component`. В `advanced` — `input[type=file]` (`multiple`, `accept`, `disabled`, `aria-label`), `.p-fileupload-header` с кнопками выбора (`p-fileupload-choose-button`), загрузки (`-upload-button`, если не `auto` и `show_upload_button`) и отмены (`-cancel-button`, если не `auto` и `show_cancel_button`), `.p-fileupload-content`. Кнопка загрузки недоступна без файлов или при превышении лимита; отмены — без файлов или во время загрузки; выбора — при достижении `file_limit` или `disabled`. Файлы SHALL выводиться в `.p-fileupload-file-list` > `.p-fileupload-file` с миниатюрой (для изображений), `.p-fileupload-file-name`, `.p-fileupload-file-size`, значком «Pending» (`warn`) или «Completed» (`success`) и кнопкой удаления.

#### Scenario: Пустой
- **WHEN** файлов нет
- **THEN** есть три кнопки, у загрузки и отмены `disabled`, списка и индикатора нет

#### Scenario: Автозагрузка
- **WHEN** `auto = true`
- **THEN** кнопок загрузки и отмены нет

### Requirement: Выбор и удаление
Выбор через поле и перетаскивание на содержимое SHALL добавлять валидные файлы (без `multiple` — заменять текущие), вызывать `on_select`, проверять `file_limit` (сообщение «Maximum number of files exceeded, limit is N at most.») и при `auto` запускать загрузку. Перетаскивание SHALL подсвечивать содержимое классом `p-fileupload-highlight`. Кнопка удаления SHALL убирать файл и вызывать `on_remove`; отмена SHALL очищать список и вызывать `on_clear`.

#### Scenario: Один файл
- **WHEN** без `multiple` выбран второй файл
- **THEN** в списке только он

#### Scenario: Лимит
- **WHEN** `file_limit = 1`, выбраны два файла
- **THEN** сообщение о превышении лимита, кнопка загрузки недоступна

### Requirement: Загрузка
Загрузка SHALL отправлять `FormData` (поле `name`, имена файлов) методом `method` на `url` через `XMLHttpRequest` с заголовками и `with_credentials`, показывать прогресс (0–100) и вызывать `on_before_upload`, `on_send`, `on_progress`. Ответ 2xx SHALL вызывать `on_upload`, иначе (и при сетевой ошибке) — `on_error`; файлы после ответа SHALL переходить в список загруженных, список ожидающих — очищаться. При `custom_upload` SHALL вызываться `on_upload_handler` вместо отправки.

#### Scenario: Ошибка сервера
- **WHEN** сервер отвечает 404
- **THEN** вызван `on_error`, идёт возврат кнопок в доступное состояние

### Requirement: Режим basic
В `basic` SHALL выводиться кнопка выбора (`p-fileupload-choose-button`), поле и подпись в `.p-fileupload-basic-content`; `Space`/`Enter` на кнопке открывают выбор; сообщения — перед содержимым.

#### Scenario: Подпись
- **WHEN** выбран один файл `a.txt`
- **THEN** подпись «a.txt»
