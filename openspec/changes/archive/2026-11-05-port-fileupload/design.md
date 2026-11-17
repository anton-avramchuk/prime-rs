# Design

## Context

Requirements are in `specs/fileupload`. PrimeNG 21.1.10 sources: `fileupload/fileupload.ts` (`FileUpload`, `FileContent`), `style/fileuploadstyle.ts`, `@primeuix/styles/fileupload`. Models: `prime-button`, `prime-message`, `prime-progressbar`, `prime-badge` (ready-made parts), `prime-drawer`/`prime-colorpicker` (crate structure, tests).

## Goals / Non-Goals

**Goals:** the same DOM, the same checks and labels, real sending through XHR.

**Non-Goals:** Angular templates (`file`, `content`), IE11, `Bind`, `getBlockableElement`.

## Decisions

### D1. Pure logic (`files.rs`)
`FileInfo {name, mime, size}`; `format_size(bytes, units)`, `is_type_valid(info, accept)`, `validate(info, accept, max_size) -> Result<(), Rejection>`, `is_duplicate`, `limit_exceeded(auto, selected, uploaded, limit)`, `choose_disabled`, `basic_label`. Table tests on the TS formulas.

### D2. State
`FileUploadRef` (Copy): `files`, `uploaded` — `RwSignal<Vec<Entry>, LocalStorage>` (`Entry {file: web_sys::File, info, object_url}`), `messages`, `progress`, `uploading`, `uploaded_count`, `highlight`, a reference to `input`. Methods `clear`, `upload`, `choose`.

### D3. Sending
`XmlHttpRequest` + `FormData`; `progress`/`load`/`error` closures — `Closure::forget` (one leak per upload). A 2xx response — `on_upload`, otherwise `on_error`; after the response `uploaded += files`, `files.clear`. Image URL objects are released on removal/clear and unmount.

### D4. Selection and drag and drop
The field's `change` and the content's `drop` read the `FileList`; the field value is reset after handling. `dragover` sets the highlight and `preventDefault`.

### D5. Tests
Logic — tables. SSR: mode markup, button states, `auto`. Browser `tests/web.rs` as one sequential test: a file is put into the input through `DataTransfer`; selection, validation, limit, removal, `clear`, `custom_upload`, sending to a non-existent address (404/network error), drop. e2e: `setInputFiles`, server response mock `page.route`.

## Risks / Trade-offs

- [Drag and drop] The `drop` event is emulated in e2e with `DataTransfer`.
- [XHR closure leak] One small closure per upload.

## Open Questions

- None.

## Migration Plan

No migration. Rollback — `git revert`.
