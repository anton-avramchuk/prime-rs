# Proposal

## Why

FileUpload — file upload: choosing with a button or drag and drop, type/size/count checks, a list with previews, a progress indicator, sending to a server or a custom handler. The third component of wave 6. Checks and labels are pure logic, the shell is built on the ready-made Button, Message, ProgressBar and Badge.

## What Changes

- New crate `prime-fileupload` (port of `p-fileupload` from PrimeNG 21.1.10).
- Pure module `files.rs`: `FileInfo`, `format_size`, type check by `accept` (MIME, `image/*` wildcard, extension), size, duplicates, `file_limit`, the `basic` mode label.
- `FileUpload` in `advanced` and `basic` modes: `div.p-fileupload.p-fileupload-<mode>.p-component`; in `advanced` — hidden `input[type=file]` field, `.p-fileupload-header` (choose, upload, cancel buttons), `.p-fileupload-content` (progress indicator, error messages, lists of pending and uploaded files with image previews, size, a "Pending"/"Completed" icon and a remove button); in `basic` — a choose button and a label of the selection.
- Behaviour: choosing through the field, drag onto the content (highlight `p-fileupload-highlight`), `multiple`, `accept`, `max_file_size`, `file_limit`, `auto`, `custom_upload` (a handler instead of sending), sending via `XMLHttpRequest` with `FormData`, headers, `with_credentials`, progress; events `on_select`, `on_before_upload`, `on_send`, `on_progress`, `on_upload`, `on_error`, `on_clear`, `on_remove`, `on_remove_uploaded_file`; `FileUploadRef` (`clear`, `upload`, `choose`).
- Page `/fileupload` (`basic`, `advanced`, `auto`, `custom`), e2e, benchmarks and budgets, `docs/fileupload.md`, `fileupload` in `theme.components`.
- **Out of scope:** Angular file and content templates, IE11, `Bind`, Angular forms, `getBlockableElement`, shared button properties `chooseButtonProps` (replaced by `choose_style_class`).

## Capabilities

### New Capabilities
- `fileupload`: choosing, validating and uploading files.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-fileupload` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-button`, `prime-message`, `prime-progressbar`, `prime-badge`, `prime-icons`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
