# fileupload Specification

## Purpose
FileUpload — choosing, validating and uploading files.

## ADDED Requirements

### Requirement: File validation
A file SHALL be accepted if it passes `accept` (a comma-separated list: an exact MIME type, a `type/*` pattern by type, or an `.ext` extension case-insensitively), does not exceed `max_file_size` and is not yet selected (name, type and size match). A rejected file SHALL produce an `error` message from the templates "{0}: Invalid file type, allowed file types: {1}." / "{0}: Invalid file size, maximum upload size is {1}.".

#### Scenario: Type
- **WHEN** `accept = "image/*"`, file `a.pdf` (`application/pdf`)
- **THEN** the file is rejected, message "a.pdf: Invalid file type, allowed file types: image/*."

#### Scenario: Extension
- **WHEN** `accept = ".PDF"`, file `a.pdf`
- **THEN** the file is accepted

#### Scenario: Size
- **WHEN** `max_file_size = 1000`, a 2048-byte file
- **THEN** rejected, the message contains "1000.000 B"

### Requirement: Size and labels
`format_size` SHALL output the size as `{number with three decimals} {unit}` with units B, KB, MB, …, base 1024; `0` — "0 B". In `basic` mode the label SHALL be: the choose label with `auto`; the file name for a single file; "Files" for several (the `fileChosenMessage` template); "No file chosen" without files.

#### Scenario: Units
- **WHEN** 1536 bytes
- **THEN** «1.500 KB»

### Requirement: Markup
`FileUpload` SHALL render `div.p-fileupload.p-fileupload-<mode>.p-component`. In `advanced` — `input[type=file]` (`multiple`, `accept`, `disabled`, `aria-label`), `.p-fileupload-header` with the choose (`p-fileupload-choose-button`), upload (`-upload-button`, if not `auto` and `show_upload_button`) and cancel (`-cancel-button`, if not `auto` and `show_cancel_button`) buttons, `.p-fileupload-content`. The upload button is disabled without files or when the limit is exceeded; cancel — without files or during upload; choose — when `file_limit` is reached or `disabled`. Files SHALL be rendered in `.p-fileupload-file-list` > `.p-fileupload-file` with a thumbnail (for images), `.p-fileupload-file-name`, `.p-fileupload-file-size`, a "Pending" (`warn`) or "Completed" (`success`) icon and a remove button.

#### Scenario: Empty
- **WHEN** there are no files
- **THEN** there are three buttons, upload and cancel are `disabled`, there is no list and no indicator

#### Scenario: Auto upload
- **WHEN** `auto = true`
- **THEN** there are no upload and cancel buttons

### Requirement: Selection and removal
Selection through the field and drag onto the content SHALL add valid files (without `multiple` — replace the current ones), call `on_select`, check `file_limit` (message "Maximum number of files exceeded, limit is N at most.") and with `auto` start the upload. Dragging SHALL highlight the content with the `p-fileupload-highlight` class. The remove button SHALL remove the file and call `on_remove`; cancel SHALL clear the list and call `on_clear`.

#### Scenario: Single file
- **WHEN** without `multiple` a second file is selected
- **THEN** only it is in the list

#### Scenario: Limit
- **WHEN** `file_limit = 1`, two files are selected
- **THEN** a limit exceeded message, the upload button is disabled

### Requirement: Upload
Upload SHALL send `FormData` (field `name`, the file names) with the `method` method to `url` via `XMLHttpRequest` with the headers and `with_credentials`, show progress (0–100) and call `on_before_upload`, `on_send`, `on_progress`. A 2xx response SHALL call `on_upload`, otherwise (and on a network error) — `on_error`; after the response the files SHALL move to the uploaded list, and the pending list SHALL be cleared. With `custom_upload`, `on_upload_handler` SHALL be called instead of sending.

#### Scenario: Server error
- **WHEN** the server responds 404
- **THEN** `on_error` is called, the buttons return to the available state

### Requirement: Basic mode
In `basic` the choose button (`p-fileupload-choose-button`), the input and the label SHALL be rendered in `.p-fileupload-basic-content`; `Space`/`Enter` on the button open the chooser; messages — before the content.

#### Scenario: Label
- **WHEN** one file `a.txt` is selected
- **THEN** the label is "a.txt"
