# FileUpload

Port of `p-fileupload` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-fileupload::FileUpload`, `FileUploadPt`, `FileUploadRef`, `FileUploadMode`, `UploadEvent`, `UploadError` | `packages/primeng/src/fileupload/fileupload.ts` (`FileUpload`, `FileContent`), `fileupload/style/fileuploadstyle.ts` |
| Type/size checks, duplicates, limit, sizes, `basic` labels | `prime-fileupload::files` — a pure port of `validate`, `isFileTypeValid`, `isFileLimitExceeded`, `formatSize`, `basicFileChosenLabel`, table tests |
| CSS | `@primeuix/styles/fileupload` (`fileupload` in `theme.components`) |
| Demos | `apps/showcase/doc/fileupload/*-doc.ts` (`basic`, `advanced`, `auto`, `custom`) |

Spec: `openspec/specs/fileupload`. Selected and uploaded files are available through `FileUploadRef` (`files()`, `uploaded_files()`, `choose()`, `upload()`, `clear()`).

```rust
view! { <FileUpload name="demo[]" url="/api/upload" multiple=true accept="image/*" max_file_size=1_000_000 on_upload=Callback::new(|e: UploadEvent| log(e.status)) /> }
```

## What matches PrimeNG

- Checks: `accept` (MIME, `type/*`, extension case-insensitively), `max_file_size`, duplicates by name, type and size, `file_limit` with its own messages; message templates are configurable (`invalid_file_*_message_*`).
- Markup and classes of `advanced` and `basic`, "Pending"/"Completed" icons, image previews (`blob:`), the `p-fileupload-highlight` highlight on drag.
- Sending `FormData` via `XMLHttpRequest` with headers, `with_credentials`, progress; `custom_upload` with a handler; `auto`.
- Events: `on_select`, `on_before_upload`, `on_send`, `on_progress`, `on_upload`, `on_error`, `on_clear`, `on_remove`, `on_remove_uploaded_file`.

## Differences

| Difference | Reason |
|---|---|
| There is a single `input[type=file]` field (next to the button), choosing is a `click()` on it | PrimeNG has two (a root one and one inside the button); the choosing is the same. |
| A preview is rendered only for images (no `<img>` is created for the rest) | In PrimeNG an `<img>` without `src` for non-images. |
| A non-2xx response and a network error call `on_error`, files stay in the list | That is how `HttpClient` behaves; the "non-2xx" branch inside `Response` in PrimeNG is unreachable. |
| No `file`, `content`, `fileLabel`, `chooseicon` etc. templates, `header` and `empty` instead; no `chooseButtonProps` | Leptos has no `ng-template`; buttons are customised with classes and labels. |
| `Space`/`Enter` on the `basic` button is handled by the button itself (`click`), not a separate `keydown` | Native `<button>` behaviour. |
| No IE11, `Bind`, Angular forms, `getBlockableElement` | Outside the port scope. |
| `XMLHttpRequest` closures are released through `forget` (one small block per upload) | Simplicity; the listeners are bound to a one-shot request object. |

## What was not verified

- Upload to a real server with a successful response in the browser tests (there — a network error); success is verified in e2e with a mocked response.
- Choosing files via the system dialog and drag from the OS (event emulation).
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.

## Performance

See `benches/` (`fileupload-1`, `fileupload-1000`, `case-fileupload`) and `benches/*budgets.toml`.
