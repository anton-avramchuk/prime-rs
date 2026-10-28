# Design

## Context

Требования — в `specs/textarea`. Исходники PrimeNG 21.1.10: `packages/primeng/src/textarea/textarea.ts`, `textarea/style/textareastyle.ts`, демонстрации `apps/showcase/doc/textarea`. В PrimeNG это директива на `<textarea>`; шаблон компонента и модель значения — `docs/inputtext.md`.

## Decisions

### D1. Компонент вместо директивы
`<Textarea />` отрисовывает `<textarea>`; значение — необязательный `RwSignal<String>` (как у InputText). Без `value` поле неконтролируемое, `p-filled` не отслеживается.

### D2. Значение в SSR
Текст `<textarea>` — это его дочерний узел, а не атрибут `value`: начальное значение выводится дочерним текстом, обновления на клиенте идут через `prop:value`.

### D3. Автоподгонка высоты
`auto_resize` создаёт клиентский `Effect` (только `wasm32`, только если включено): `style.height = auto`, затем `scrollHeight` в пикселях; при достижении `max-height` — `overflow-y: scroll` и высота равна `max-height`, иначе `overflow: hidden`. `on_resize` вызывается после каждой подгонки. Логика — чистая функция над числами и покрыта юнит-тестом.

### D4. Классы
Порядок как в `textareastyle.ts`; `p-textarea-resizable` ставится при `auto_resize`. Вариант — `variant` либо `PrimeConfig.input_variant`.

### D5. Ветви
Статическая — при `all_static!` и `value.is_none()`; иначе реактивная. Разметка — `macro_rules!`; `Arc` только в реактивной ветви и обработчиках.

### D6. Тесты
SSR: структура, атрибуты, классы, значение дочерним текстом, `pt`, паритет. Браузер: ввод пишет в сигнал, сигнал меняет поле, автоподгонка меняет высоту и вызывает `on_resize`, события.

## Risks / Trade-offs

- [Подгонка зависит от вёрстки] → проверяется в браузере по изменению высоты при росте текста.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
