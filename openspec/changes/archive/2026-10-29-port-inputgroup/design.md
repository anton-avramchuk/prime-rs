# Design

## Context

Требования — в `specs/inputgroup`. Исходники PrimeNG 21.1.10: `packages/primeng/src/inputgroup/inputgroup.ts`, `inputgroup/style/inputgroupstyle.ts`, `inputgroupaddon/inputgroupaddon.ts`, `inputgroupaddon/style/inputgroupaddonstyle.ts`, демонстрации `apps/showcase/doc/inputgroup`. Образец — `crates/prime-toolbar` (`docs/toolbar.md`): статическая и реактивная ветви, `macro_rules!` разметки, `pt` по частям. Общей логики и клиентского кода нет.

## Decisions

### D1. Один крейт, два компонента
`InputGroup` и `InputGroupAddon` живут в `prime-inputgroup`: аддон не используется отдельно, а CSS темы у них общий (`@primeuix/styles/inputgroup`).

### D2. Ветви
Статическая — когда `style_class` и `style` статические. Реактивная — замыкания над `Arc<Props>` и `Arc<…Pt>` только для изменяемых атрибутов корня; `children` строятся один раз. Разметка — общий `macro_rules!`.

### D3. Классы и атрибуты
`InputGroup`: `p-inputgroup` + `style_class` + класс из `pt.root`. `InputGroupAddon`: `p-inputgroupaddon` + то же. `style` есть у обоих (в PrimeNG у `InputGroup` его нет, но он безвреден и единообразен; отличие записать в docs). Свойство `fluid` не переносится: в PrimeNG 21.1.10 у `InputGroup` нет такого входа, `instance.fluid` всегда `undefined`.

### D4. CSS
`theme.components` получает `inputgroup`; блок `/*For PrimeNG*/` не нужен (`primeng_extra` не меняется), так как правила относятся к Angular-обёрткам `p-button`, `p-inputwrapper`, `p-inputmask`. Скругления крайних `Button` и `InputText` внутри группы обеспечивают правила `@primeuix/styles/inputgroup`; при расхождении визуала поправить в showcase, а не в генераторе.

### D5. Демонстрации
`basic` (иконка, текст, аддоны вокруг `InputText`), `button` (`Button` рядом с полем), `checkbox` (`Checkbox` в аддоне), `multiple` (несколько аддонов). `floatlabel` и `iftalabel` пропускаются (нет компонентов), отличие записать в `docs/inputgroup.md`.

### D6. Тесты
SSR: структура и порядок, классы, `style`, `pt`, паритет ветвей, вложенность с `InputText`/`Button`. Браузерных тестов нет: клиентского поведения нет.

## Risks / Trade-offs

- [Скругления крайних элементов зависят от DOM-структуры `Button` и `InputText` в Leptos] → проверить e2e и скриншотом страницы `/inputgroup`; при расхождении добавить недостающее правило в CSS showcase и записать в docs.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
