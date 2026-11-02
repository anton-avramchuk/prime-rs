# Tasks

## 1. Основа

- [x] 1.1 Добавить `paginator` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился, что классы `p-paginator-*` есть и что `select`/`inputnumber` покрывают вложенные списки и поле

## 2. Крейт

- [x] 2.1 `crates/prime-paginator`: `state.rs` (`page_count`, `page`, `link_bounds`, `links`, `report`, `rows_items`, `page_items`, `clamp_first_after_total`, `next_first_for_rows`, классы) с табличными тестами
- [x] 2.2 Компонент `Paginator`, `PaginatorState`, `RowsPerPage`, `PaginatorPt`: корень, кнопки, ссылки, отчёт, `template_left`/`template_right`, значки, скрытие, SSR-тесты, rustdoc
- [x] 2.3 Поведение: `change_page`, кнопки и ссылки, `on_page_change`, двусторонние `first`/`rows`, отступ назад при уменьшении `total_records`
- [x] 2.4 Списки и поле: переход к странице списком и полем, число строк со «Показать все», синхронизация с `first`/`rows`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/paginator` (`basic`, `currentpagereport`, `template`, `images`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `paginator.spec.ts` (разметка, кнопки и ссылки, окно ссылок, списки, отчёт); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test paginator`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-paginator` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/paginator.md` (отличия от PrimeNG, что не проверялось); `prime-paginator` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-paginator`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Stepper.
