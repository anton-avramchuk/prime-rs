# Tasks

## 1. Основа

- [x] 1.1 Добавить `steps` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-steps*`

## 2. Крейт `prime-steps`

- [x] 2.1 `state.rs` (классы, отключённость, `tabindex`, `aria-disabled`, исход клика, целевой индекс клавиши, клавиши активации) с табличными тестами
- [x] 2.2 Компонент `Steps`, `StepsItem`, `StepsCommand`, `StepsPt`: корень, список, пункты, номера и подписи, SSR-тесты, rustdoc
- [x] 2.3 Клик: `readonly`, отключённые, `on_active_index_change`, `active_index_signal`, `command` с индексом, отмена перехода без `url`
- [x] 2.4 Клавиатура: ←/→/`Home`/`End`, roving `tabindex`, `Tab`, `Enter`/`Space`
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/steps` (`basic`, `controlled`, `interactive`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `steps.spec.ts` (стили, клик, клавиши, кнопки управляемой демонстрации); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test steps`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-steps` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/steps.md` (отличия от PrimeNG, что не проверялось); `prime-steps` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты (SSR по одному: `--exact`; эффекты только под `cfg(wasm32)`), deny, machete, `openspec validate port-steps`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: ContextMenu.
