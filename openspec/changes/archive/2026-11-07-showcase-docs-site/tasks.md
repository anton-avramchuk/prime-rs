# Tasks

## 1. Генерация

- [x] 1.1 `xtask gen-docs`: разбор демо (`<Demo>` → `fn Component`) и таблиц свойств из `crates/prime-*/src/lib.rs`, модульные тесты на фикстурах
- [x] 1.2 Markdown `docs/*.md` → HTML (`pulldown-cmark`), CSS-классы компонента из темы
- [x] 1.3 Вывод `showcase/src/generated/*`, `--check`

## 2. Оболочка

- [x] 2.1 `showcase/src/doc.rs`: `DocPage`, карточка примера с кнопкой Code, таблицы API, оглавление, соседние страницы; стили в `shell`
- [x] 2.2 Меню по разделам с поиском; список компонентов в одном месте
- [x] 2.3 SSR-тест `DocPage`

## 3. Страницы

- [x] 3.1 Перевод страниц на `DocPage` (описание и строка импорта на страницу); выборочная проверка AutoComplete, Button, Table
- [x] 3.2 Остальные страницы по разделам меню

## 4. Проверки

- [x] 4.1 `e2e/tests/docs.spec.ts`; существующие e2e проходят
- [x] 4.2 `ci.yml`: `cargo xtask gen-docs --check`; fmt, clippy, deny, machete, `openspec validate showcase-docs-site`
- [x] 4.3 `docs/showcase.md`: как запустить и обновить документацию
