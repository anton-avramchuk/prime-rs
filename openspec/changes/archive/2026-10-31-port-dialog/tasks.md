# Tasks

## 1. Основа

- [x] 1.1 Добавить `dialog` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что `@keyframes` для `p-dialog-enter-active` / `p-dialog-leave-active` попали в CSS (иначе — решить вопрос из design.md)

## 2. Крейт

- [x] 2.1 `crates/prime-dialog`: `state.rs` (`DialogPosition`, `mask_classes`, `root_classes`, `mask_inline_style`, `root_inline_style`, `drag_position`, `drag_accept`, `resize_to`, `breakpoints_css`, `is_topmost`, `mask_z`) с табличными тестами
- [x] 2.2 Компонент `Dialog` и `DialogPt`: разметка (маска > окно), `Portal`, заголовок и кнопки, подвал, `headless`, SSR-тесты, rustdoc
- [x] 2.3 Показ и закрытие: `visible`/`rendered`/`leaving`, анимации, маска (`modal`, `dismissable_mask`), Escape верхнего окна, `block_scroll`, `FocusTrap`, фокус при показе, `z-index`, события; браузерные `tests/web.rs`
- [x] 2.4 Указатель и размер: перетаскивание (`keep_in_viewport`, `min_x`, `min_y`, `on_drag_end`), изменение размера (`on_resize_init`, `on_resize_end`), максимизация (`on_maximize`), `breakpoints`; браузерные тесты

## 3. Showcase и e2e

- [x] 3.1 Страница `/dialog` (`basic`, `position`, `modal`, `withoutmodal`, `maximizable`, `responsive`, `longcontent`, `template`, `headless`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `dialog.spec.ts` (открытие, закрытие кнопкой, маской, Escape, модальность, перетаскивание и размер настоящей мышью, максимизация, позиции, Tab-цикл); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test dialog`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-dialog` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 5. Интеграция

- [x] 5.1 `docs/dialog.md` (отличия от PrimeNG, что не проверялось); `prime-dialog` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-dialog`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Tooltip.
