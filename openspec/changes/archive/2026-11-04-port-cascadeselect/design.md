# Design

## Context

Требования — в `specs/cascadeselect`. Исходники PrimeNG 21.1.10: `cascadeselect/cascadeselect.ts` (`CascadeSelectSub` — список уровня, `CascadeSelect` — состояние, путь, клавиатура), `style/cascadeselectstyle.ts`; демонстрации `apps/showcase/doc/cascadeselect/*-doc.ts`. Образцы — `prime-treeselect` (оболочка, панель внутри корня, окно-слушатели) и `prime-menubar` (путь активных пунктов, фокус, поиск, `matchMedia`).

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, путь и клавиши.

**Non-Goals:** произвольные объекты опций, формы Angular, `appendTo`, отзеркаливание списка у края окна, шаблоны `header`/`footer`/иконок.

## Decisions

### D1. Модель
`CascadeOption { label, value: Option<String>, disabled, children }`; значение листа — `value` либо `label`. `build(options) -> Vec<Node>` с ключами `0`, `0_1`, `0_1_2` (как `createProcessedOptions`), в узле: `parent_key`, `level`, `index`, `group`, `valid`, `label`, `value`.

### D2. Чистая логика (`state.rs`)
`State { active: Vec<String>, focus: Option<String>, dirty: bool }` и `Menu { nodes }` с функциями по исходнику: `show` (путь к значению и начальный фокус), `option_change`, `option_click` (результат `Closed`/`Group`/`Select(value)`), `hover`, `arrow_down/up/left/right`, `home`/`end`, `enter`, `escape`/`tab`, `search`, `visible(focus)` — список уровня фокуса. Фокус хранится ключом; индекс среди видимых вычисляется. Все функции — чистые, с табличными тестами.

### D3. Оболочка
Как у `TreeSelect`: `open`, `focused`, ссылки на корень, панель и поле; эффекты открытия (`z-index` группы `Overlay`, `align_overlay(Relative)`, слушатели клика вне и `resize`) — под `cfg(wasm32)`. Панель выводится только при открытии. После смены пути панель перевыравнивается (`set_timeout(0)`). `matchMedia(breakpoint)` даёт `mobile`.

### D4. Вложенные списки
Рекурсивная функция уровня; вложенный список выводится только у активной группы. Положение вложенных списков задаёт CSS темы (`left: 100%`). Чтение сигналов при построении — только `*_untracked`.

### D5. Тесты
Таблицы для модели; SSR: закрытое поле, метка, плейсхолдер, состояния, очистка, загрузка, `pt`; браузерные `tests/web.rs` одним последовательным тестом: открытие, каскад, выбор листа, очистка, клик вне, наведение, клавиатура, мобильный класс; после клавиши `sleep`.

## Risks / Trade-offs

- [Фокус и путь] В PrimeNG фокус хранит индекс уровня — у нас ключ, поэтому переходы по уровням (`←`) считаются от ключа родителя; поведение проверяется таблицей.
- [Положение вложенных списков] Без `position()` список может выйти за правый край окна.

## Open Questions

- Что именно делает `Tab` на листе: `onOptionChange` без выбора (сверено: выбирает путь, значение не меняется) — оставляем.

## Migration Plan

Миграции нет. Откат — `git revert`.
