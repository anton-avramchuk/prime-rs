# Tasks

## 1. Тип свойства `Prop<T>`

- [x] 1.1 Реализовать `Prop<T>`/`OptProp<T>` в `prime-core` (конструкторы `From`, `Default`, `get`, `with`, `get_untracked`, `is_static`); проверить юнит-тестами сценарии спеки `core-runtime` (статическое значение не создаёт примитивов, сигнал реагирует, значение по умолчанию) и компиляцией вызовов `value="5"`, `RwSignal`, `Memo`
- [x] 1.2 Задокументировать `Prop` в rustdoc с примерами и проверить `cargo test --doc -p prime-core`; если коэрсии `Prop<Option<T>>` не компилируются — вынести `OptProp<T>` в отдельный тип и записать решение в `design.md`

## 2. Badge

- [x] 2.1 Перевести `Badge` на `Prop`/`OptProp`, убрать `Memo` и `class_memo`, вычислять атрибуты в замыканиях; проверить, что все существующие SSR-тесты `prime-badge` проходят без изменений ожидаемых значений
- [x] 2.2 Добавить тесты обеих веток (статическая и реактивная, смена `value` и `severity`) и измерить `badge-1000` через `cargo bench -p render-bench --bench alloc`; записать значение до и после

## 3. Button

- [x] 3.1 Перевести `Button` на `Prop`/`OptProp` (все свойства, включая `badge`, `severity`, `loading`, `disabled`), убрать лишние `Memo`, вычислять атрибуты в замыканиях; проверить, что все SSR-тесты и браузерные тесты `prime-button` проходят без изменений ожидаемых значений
- [x] 3.2 Измерить `button-1000` и `button-full-1000`; если на динамическом пути один `Memo` состояния даёт заметный выигрыш, оставить его и записать решение в `design.md` (D3); проверить тест «class не перезаписывается при изменении постороннего свойства»

## 4. Бюджеты аллокаций

- [ ] 4.1 Реализовать `cargo xtask alloc-budget` и `benches/alloc-budgets.toml` (`badge`, `button`, `button-full`: максимум аллокаций на экземпляр); проверить юнит-тестами парсинга и превышения и запуском на реальном `target/bench/alloc.json`, включая искусственно заниженный бюджет (ненулевой код выхода)
- [ ] 4.2 Добавить `alloc-budget` в job `apps` workflow `ci.yml` и описание в `docs/benchmarks.md`; проверить синтаксис YAML и прогон CI на GitHub

## 5. Проверка в приложении

- [ ] 5.1 Собрать showcase (`cargo leptos build`) и прогнать e2e (`npx playwright test`) со страницами `/button` и `/badge`; убедиться, что внешний вид и поведение не изменились (в т. ч. страница Loading)
- [ ] 5.2 Добавить в `e2e/tests/memory.spec.ts` сценарий «1000 кнопок», записать значения `heap_bytes`/`dom_nodes` до и после в `docs/benchmarks.md` и оценить стоимость `RenderEffect` атрибутов (D2)

## 6. Интеграция

- [ ] 6.1 Обновить `docs/benchmarks.md` и `docs/button.md` (новые значения аллокаций и шаблон `Prop` и вывод про атрибуты-замыкания для следующих компонентов); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate optimize-reactive-props`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI.
- Следующие компоненты волны 1 (Tag, Chip, Avatar, Divider, Card, …) берут `Prop` как шаблон.
