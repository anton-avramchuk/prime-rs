# Design

## Context

Требования — в `specs/panelmenu`. Исходники PrimeNG 21.1.10: `panelmenu/panelmenu.ts` (`PanelMenuSub` — список пунктов уровня, `PanelMenuList` — дерево внутри панели, `PanelMenu` — панели-заголовки), `panelmenu/style/panelmenustyle.ts`; демонстрации `apps/showcase/doc/panelmenu/{basic,multiple,controlled,command,template}-doc.ts` (`router-doc` и `accessibility-doc` не переносятся). Образцы в проекте — `prime-accordion` (панели и коллапс), `prime-tieredmenu` (модель пунктов, фокус, поиск), `prime-core::collapsible`.

Отличия от прежних меню: нет всплывающих уровней и слоёв `z-index`, нет клика вне и `resize`; «видимые» пункты — это раскрытые ветки, активный путь — набор раскрытых ключей, а не единственная цепочка.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, клавиши, раскрытие и анимация.

**Non-Goals:** `routerLink`, `Tooltip`/`Badge`, HTML-метки, `Ripple`, `Bind`, `pTemplate`, `transitionOptions`.

## Decisions

### D1. Состояние раскрытия
Набор раскрытых ключей — `RwSignal<HashSet<String>>` (ключ = `key` пункта или позиция вида `0_1_2`, как в исходнике). Переключение: при `multiple = false` снимает раскрытие с сестёр с тем же родителем; при `true` только переключает. Чистая функция `toggle_key(set, key, parent_key_of_siblings, multiple)` с табличными тестами. Начальное множество — из `expanded` пунктов.

### D2. Модель дерева и фокус
Собственная небольшая плоская модель в крейте (`nodes`, `visible(expanded)`, `next/prev/first/last`, `arrow_left/right`) по `flatItems`/`onArrow*Key` исходника; `prime-tieredmenu::model` не переиспользуется, так как его «видимость» завязана на всплывающие пути. Если окажется, что `Node`/`build`/`item_at` подходят без правок — взять их (проверить при apply), иначе не усложнять. Чистая логика без `web_sys` — с табличными тестами.

### D3. Разметка
Рекурсивная функция уровня; корневой пункт рисуется как панель с заголовком (и своим деревом внутри `content`), вложенные — как `li.p-panelmenu-item`. Коллапс-области — `prime_core::collapsible::bind_region_with`, `CollapseIds` для `aria-controls`. Кнопки заголовка в PrimeNG — `div` с `a`; `ButtonPt` не нужен.

### D4. Фокус и клавиатура
Единый обработчик `keydown`/`focus`/`blur` на корневом списке активной панели; сфокусированный ключ — сигнал; при переходе на заголовок другой панели фокус переносится в её список (`focus_no_scroll`). `scroll_into_view` для сфокусированного `li` — под `cfg(wasm32)`; эффекты — только под `cfg(wasm32)` (иначе паника executor в nextest).

### D5. Управление
`PanelMenuRef` (`Copy`) со ссылкой на множество ключей и модель: `expand_all`, `collapse_all`, `toggle_all`.

### D6. Тесты
SSR: разметка, ARIA, начальное раскрытие, скрытые/отключённые/разделители, шаблоны, `pt`. Браузерные `tests/web.rs` одним последовательным тестом: клик, одиночный и множественный режим, команда, отключённый, клавиатура, `expand_all`/`collapse_all`. e2e: стили темы, раскрытие/сворачивание, клавиатура.

### D7. Демонстрации и бенчмарки
`basic`, `multiple`, `controlled`, `command` (Toast), `template`. Бенч `panelmenu-1`/`-1000`, `case-panelmenu`.

## Risks / Trade-offs

- [Двойное раскрытие по клику] Клик по заголовку и по ссылке внутри не должен срабатывать дважды → обработчик на `-header-content`/`-item-content`, как в исходнике.
- [Схожесть с Accordion] Не тянем `prime-accordion` целиком: его значение — список строк по панелям, здесь — вложенные ключи.
- [Дублирование модели с TieredMenu] Осознанно; вынос — после Tree.

## Open Questions

- Как `multiple = false` ведёт себя для вложенных веток — по `activeItemPath.filter(parentKey !== …)` исходника это «одна на уровень»; сверить при apply.

## Migration Plan

Миграции нет. Откат — `git revert`.
