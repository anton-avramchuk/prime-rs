# Design

## Context

Требования — в `specs/splitbutton`. Исходники PrimeNG 21.1.10: `splitbutton/splitbutton.ts` (около 300 строк), `splitbutton/style/splitbuttonstyle.ts`; стили `@primeuix/styles/splitbutton`; демонстрации `apps/showcase/doc/splitbutton/*-doc.ts`. Образцы в проекте: `prime-button` (`Button`, `ButtonState`, функции классов), `prime-menu` (`Menu` с `popup`, `MenuRef::show_at`/`toggle`/`hide`, `on_show`/`on_hide`), `prime-panel` (кнопка-переключатель со своей разметкой, потому что `ButtonPt` не поддерживает реактивные атрибуты).

Наблюдения по исходнику:
- Шаблон: основная кнопка (`pButton`, `label`/`icon`/`iconPos`, либо шаблон `content`), кнопка раскрытия (`pButton`, значок `ChevronDown` или `dropdownIcon`), `p-tieredmenu` в режиме `popup`.
- Корневые классы: `p-splitbutton p-component` + модификаторы по `raised`, `rounded`, `outlined`, `text`, `size` (`small → sm`, иначе `lg`); у хоста `data-p-severity`.
- `onDefaultButtonClick`: `onClick.emit(event)`, затем `menu.hide()`. `onDropdownButtonClick`: `onDropdownClick.emit(event)`, затем `menu.toggle({ currentTarget: hostElement, relativeAlign: appendTo == 'self' })` — меню выравнивается по хосту, не по стрелке. `onDropdownButtonKeydown`: ↓/↑ → то же, что клик, и `preventDefault`.
- `aria-expanded` кнопки раскрытия — `isExpanded` (меняют `onShow`/`onHide` меню), `aria-controls` — `ariaId` меню.
- `raised` и `rounded` доходят до кнопок только через CSS корня (`p-splitbutton-raised` / `-rounded`), а `severity`, `text`, `outlined`, `size` передаются самим `pButton`.
- Меню — `TieredMenu`; в проекте `TieredMenu` ещё нет (последний компонент волны 4), есть плоский `Menu` с группами.

## Goals / Non-Goals

**Goals:** тот же DOM, классы и `aria-*`; то же поведение клика, клавиш и закрытия меню; меню привязано к корню.

**Non-Goals:** вложенные подменю (до `TieredMenu`), `tooltip`/`tooltipOptions`, `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `Ripple`, `@primeuix/motion`, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и API
`prime-splitbutton` с `SplitButton`, `SplitButtonPt { root, button, dropdown, menu }`. Модель — `Prop<Vec<MenuItem>>` из `prime-menu`. `severity: OptProp<ButtonSeverity>`, `size: OptProp<Size>`, `icon: Option<ViewFn>`, `icon_pos: Prop<IconPos>`, `content: Option<ViewFn>`.

### D2. Основная кнопка
`Button` с `style_class="p-splitbutton-button"` и общими `severity`/`text`/`outlined`/`plain`/`size`/`icon`/`icon_pos`/`label`/`disabled`/`tabindex`/`autofocus`/`aria_label`; при `content` — `children` кнопки вместо метки (метка и значок не выводятся). `disabled = disabled || button_disabled`. `on_click` обёртка: пользовательский обработчик, затем `menu.hide()`.

### D3. Кнопка раскрытия
Своя разметка `<button type="button">` с классами и `data-p*` из `prime_button::{ButtonState, root_classes, root_data_p}` (иконка без метки даёт `p-button-icon-only`) плюс `p-splitbutton-dropdown`; значок — `ChevronDownIcon` с классом `p-button-icon` или `dropdown_icon`. Реактивные `aria-expanded`/`aria-controls` задаются атрибутами-замыканиями. Причина своей разметки — `Button` не принимает реактивные произвольные атрибуты.

### D4. Меню
`Menu popup=true menu_ref=… model=… style=menu_style style_class=menu_style_class id=menu_id on_show on_hide` внутри корня. `MenuRef` создаётся в `SplitButton`; открытие — `menu_ref.show_at(root_element)` (корень получаем через `NodeRef`), закрытие — `menu_ref.hide()`. `on_show`/`on_hide` меню меняют `expanded: RwSignal<bool>` и вызывают `on_menu_show`/`on_menu_hide`.

### D5. Чистая логика (`state.rs`)
`root_classes(raised, rounded, outlined, text, size)` — классы корня; `is_toggle_key(code)` (`ArrowDown`/`ArrowUp`). Табличные тесты.

### D6. CSS
`splitbutton` добавляется в `theme.components`. Правила `.p-splitbutton*` и общие стили кнопки уже в теме; проверить, что `p-splitbutton-raised`, `-rounded`, `-button`, `-dropdown` есть.

### D7. Тесты
SSR: разметка, модификаторы, `severity` на обеих кнопках, `content`, `disabled`/`button_disabled`/`menu_button_disabled`, `aria-*` кнопки раскрытия, меню закрыто. Браузерные `tests/web.rs`: клик по основной кнопке (событие, закрытие меню), клик по стрелке (событие, меню открыто, `aria-expanded`), ↓/↑, закрытие кликом вне, положение панели под корнем, команда пункта, события показа/скрытия. e2e: настоящая мышь и клавиши, положение панели, скриншотные пары.

### D8. Демонстрации и бенчмарки
Демонстрации `basic` (с `Toast`/`MessageService` для `command` пунктов), `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`. Значки PrimeIcons (`pi-check`, `pi-cog`) заменяются из `prime-icons`. Бенч `splitbutton-1`/`-1000` и `case-splitbutton`.

## Risks / Trade-offs

- [Меню без вложенности] `nested` не переносится до `TieredMenu`; фиксируется в docs.
- [Своя кнопка раскрытия] Классы и `data-p` берём из функций `prime-button`, чтобы не разойтись с `Button` при изменениях.
- [Положение панели] `Menu` выравнивается относительно предка с `position: relative`; корень `.p-splitbutton` его даёт темой — проверяется в e2e.

## Open Questions

- Нужен ли `appendTo`: нет, меню внутри корня, как у остальных оверлеев проекта.

## Migration Plan

Миграции нет. Откат — `git revert`.
