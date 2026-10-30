# Inplace

Порт `p-inplace` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-inplace::Inplace`, `InplacePt` | `inplace/inplace.ts`, `inplace/style/inplacestyle.ts` |
| CSS | `@primeuix/styles/inplace` — `cargo xtask gen-theme` (блока `/* For PrimeNG */` у компонента нет) |
| Демонстрации | `apps/showcase/doc/inplace/{basic,input,image,lazy}-doc.ts` |

Спека: `openspec/specs/inplace`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-inplace.p-component[aria-live=polite]` > `div.p-inplace-display[tabindex=0][role=button]` (с `p-disabled` и `data-p-disabled`) или `div.p-inplace-content` с содержимым и, при `closable`, кнопкой закрытия со значком `TimesIcon`.
- Клик по виду и `Enter` активируют; `Enter` отменяет действие по умолчанию. `prevent_click` отключает клик и закрытие, но не `Enter` (как в PrimeNG). При `disabled` ничего не происходит.
- Содержимое получает функцию закрытия (`closeCallback`); она подчиняется тем же правилам, что кнопка закрытия.
- Содержимое создаётся при каждом показе и удаляется при возврате к виду (как `*ngIf`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>` без хоста `<p-inplace>` | Хост в Angular — деталь фреймворка. |
| `display: ViewFn` и `content: Callback<Callback<()>, AnyView>` вместо шаблонов `display`, `content` и селекторов `pInplaceDisplay` / `pInplaceContent` | Вместо `ng-template` и проекции содержимого. |
| `close_icon` — `ViewFn`, а не имя класса шрифта; нет шаблона `closeicon` | Иконки проекта — SVG-компоненты. |
| `on_activate` и `on_deactivate` — `Callback<()>` без события | Событие нужно в основном для `preventDefault`; изменение состояния не привязано к событию (закрытие бывает из колбэка содержимого). |
| `active` — двусторонний `RwSignal<bool>` | В PrimeNG — обычное свойство, которое компонент меняет сам. |
| Нет `Ripple` у кнопки закрытия | `Ripple` не портирован как отдельная директива. |
| В демонстрации `lazy` — обычная `<table>` вместо `Table`; в `input` нет `pAutoFocus`; в `image` вместо значка `pi-image` — `EyeIcon` | `Table` не портирован; в `prime-icons` нет `image`; у `InputText` нет `node_ref`. |
| Клик, закрывший содержимое, не активирует новый вид | Тот же клик продолжает всплывать к только что созданному виду; цель, уже убранная из документа, игнорируется. В PrimeNG событие уже обработано Angular до замены. |

## Стоимость

32 аллокации на экземпляр при SSR (вид), вклад в wasm ≈ 500 КБ (бюджет 550 КБ в `benches/budgets.toml`; в основном общий `Button` и иконка).

## Не проверено

Касание и экранные считыватели: проверены разметка, клик, `Enter`, закрытие и колбэки в Chrome (e2e и браузерные тесты). Фокус при переключении не переносится — как в PrimeNG.
