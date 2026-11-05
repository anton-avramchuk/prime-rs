# Tasks

## 1. Основа

- [x] 1.1 `fileupload` в `theme.components`, `cargo xtask gen-theme`; есть `p-fileupload-file-badge`, `p-fileupload-highlight`
- [x] 1.2 Свериться с `onFileSelect`, `validate`, `uploader`, `isFileLimitExceeded`, `basicFileChosenLabel`, `onDrop` исходника

## 2. Крейт `prime-fileupload`

- [x] 2.1 Модуль `files.rs` с табличными тестами
- [x] 2.2 Компонент `FileUpload`, `FileUploadPt`, `FileUploadRef`: режимы `advanced` и `basic`, SSR-тесты, rustdoc
- [x] 2.3 Выбор, перетаскивание, проверки, лимит, удаление, очистка
- [x] 2.4 Отправка через XHR, прогресс, `custom_upload`, `auto`
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/fileupload`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `fileupload.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-fileupload`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/fileupload.md`; `prime-fileupload` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-fileupload`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; коммит и архив локально.
