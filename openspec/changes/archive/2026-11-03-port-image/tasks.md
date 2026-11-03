# Tasks

## 1. Основа

- [x] 1.1 Добавить `image` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-image*`
- [x] 1.2 Проверить ключи `aria.*` в `translation_defaults.rs` (`rotateLeft`, `zoomOut`), недостающие добавить

## 2. Крейт

- [x] 2.1 `crates/prime-image`: `state.rs` (`root_classes`, границы и шаги масштаба, `transform`, классы кнопок, `mask_should_close`) с табличными тестами
- [x] 2.2 Компонент `Image`, `ImageLoading`, `ImagePt`: корень, `img`, кнопка просмотра, представления `image`/`indicator`, `on_image_error`, SSR-тесты, rustdoc
- [x] 2.3 Просмотр: `Portal`, маска, панель, поворот и масштаб, `preview`-шаблон, закрытие кнопкой/маской/`Escape`, слой, блокировка прокрутки, ловушка и возврат фокуса, `on_show`/`on_hide`, сброс при закрытии
- [x] 2.4 Браузерные `tests/web.rs` (один последовательный тест просмотра)

## 3. Showcase и e2e

- [x] 3.1 Страница `/image` (`basic`, `preview`, `previewimagesource`, `template`), локальные картинки, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `image.spec.ts` (стили, открытие и закрытие просмотра, поворот и масштаб, клавиша `Escape`); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test image`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-image` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/image.md` (отличия от PrimeNG, что не проверялось); `prime-image` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-image`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: ConfirmPopup.
