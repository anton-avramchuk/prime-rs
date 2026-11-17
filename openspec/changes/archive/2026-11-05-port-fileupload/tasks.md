# Tasks

## 1. Foundation

- [x] 1.1 `fileupload` in `theme.components`, `cargo xtask gen-theme`; `p-fileupload-file-badge`, `p-fileupload-highlight` exist
- [x] 1.2 Check against the source's `onFileSelect`, `validate`, `uploader`, `isFileLimitExceeded`, `basicFileChosenLabel`, `onDrop`

## 2. Crate `prime-fileupload`

- [x] 2.1 Module `files.rs` with table tests
- [x] 2.2 The `FileUpload` component, `FileUploadPt`, `FileUploadRef`: `advanced` and `basic` modes, SSR tests, rustdoc
- [x] 2.3 Selection, drag and drop, checks, limit, removal, clear
- [x] 2.4 Sending via XHR, progress, `custom_upload`, `auto`
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/fileupload`, menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `fileupload.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-fileupload`, budgets

## 5. Integration

- [x] 5.1 `docs/fileupload.md`; `prime-fileupload` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-fileupload`

## Workflow follow-up

- Wave 6 — CI once at the end of the wave; commit and archive locally.
