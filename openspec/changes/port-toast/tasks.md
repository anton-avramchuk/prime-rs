# Tasks

## 1. Основа

- [x] 1.1 Добавить `toast` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что классы `p-toast-message-*`, значки и `@keyframes` для `p-toast-message-enter-active` / `-leave-active` есть (иначе — решить вопрос из design.md)

## 2. Крейт

- [x] 2.1 `crates/prime-toast`: `state.rs` (`ToastSeverity`, `ToastPosition`, `message_classes`, `root_classes`, `position_style`, `matches_key`, `contains_message`, `can_add`, `lifetime`, `breakpoints_css`) с табличными тестами
- [x] 2.2 `ToastMessage` (строитель) и `MessageService` (`provide_message_service`, `use_message_service`, `add`, `add_all`, `clear`, `clear_key`, подписка/отписка) с тестами без браузера
- [x] 2.3 Компонент `Toast` и `ToastPt`: контейнер, элементы списка, значки, кнопка закрытия, `message`/`headless`, SSR-тесты, rustdoc
- [x] 2.4 Поведение: добавление и ключи, таймеры (`life`, `sticky`, наведение), закрытие и анимация выхода, `clear`/`clear_key`, дубликаты, слой, `breakpoints`, `on_close`, отписка; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/toast` (`basic`, `severity`, `multiple`, `position`, `sticky`, `life`, `clear`, `target`, `responsive`, `template`, `headless`), служба сообщений в оболочке, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `toast.spec.ts` (показ, серьёзности, закрытие кнопкой, самозакрытие, sticky, наведение, clear, ключи, положения); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test toast`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-toast` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне)

## 5. Интеграция

- [x] 5.1 `docs/toast.md` (отличия от PrimeNG, что не проверялось); `prime-toast` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-toast`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Menu.
