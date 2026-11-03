# Tasks

## 1. Основа

- [x] 1.1 Добавить `carousel` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-carousel*`
- [x] 1.2 Проверить переводы `aria.slide`, `slideNumber`, `pageLabel`, `prevPageLabel`, `nextPageLabel`

## 2. Крейт `prime-carousel`

- [x] 2.1 `model.rs`: `Track` и чистые функции (`total_dots`, `is_circular`, `recalc`, `step`, `settle`, `after_transition_shift`, `first_index`/`last_index`, `nav_disabled`, `responsive`, `swipe`, `autoplay_next`, `translate`, подстановка в `aria`-строки) с табличными тестами
- [x] 2.2 Компонент `Carousel<T>`, `CarouselResponsive`, `CarouselOrientation`, `CarouselPt`: корень, навигаторы, окно, элементы, клоны, индикаторы, header/footer, SSR-тесты, rustdoc
- [x] 2.3 Листание: кнопки, точки, `page` (двусторонний), `on_page`, `transition`/`transitionend`, `p-items-hidden`, скачок с клона в бесконечном режиме
- [x] 2.4 Адаптивность на `resize`, ширина элемента инлайн-стилем; автопрокрутка и её остановка; свайп; клавиатура индикаторов
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/carousel` (`basic`, `responsive`, `circular`, `vertical`), локальные данные товаров, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `carousel.spec.ts` (стили, листание, бесконечный режим, автопрокрутка, адаптивность, вертикальный режим, свайп); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test carousel`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-carousel` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/carousel.md` (отличия от PrimeNG, что не проверялось); `prime-carousel` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты (SSR по одному: `--exact`; эффекты только под `cfg(wasm32)`), deny, machete, `openspec validate port-carousel`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: TieredMenu.
