# Proposal

## Why

CascadeSelect — поле выбора значения из вложенной структуры: панель с корневым списком, из которого вбок раскрываются списки вложенных групп. Оболочка (метка, очистка, кнопка, панель внутри корня) такая же, как у `Select`/`TreeSelect`; новое — каскад списков, путь активных групп и клавиатура с переходами между уровнями.

## What Changes

- Новый крейт `prime-cascadeselect` (порт `p-cascadeselect` из PrimeNG 21.1.10).
- Модель: `CascadeOption` — `label`, `value` (по умолчанию `label`), `disabled`, `children`. Значение — `RwSignal<Option<String>>` (значение листа).
- `CascadeSelect`: корень `div.p-cascadeselect.p-component.p-inputwrapper` (`p-cascadeselect-clearable|mobile|open|fluid|sm|lg`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`) > скрытое поле `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `aria-activedescendant`) > `span.p-cascadeselect-label` (подпись выбранного листа либо `placeholder`) > значок очистки > `div.p-cascadeselect-dropdown` (стрелка либо спиннер загрузки) > панель `div.p-cascadeselect-overlay` > `div.p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` > `li.p-cascadeselect-option[role=treeitem]` (`-option-group`, `-option-active`, `-option-selected`, `p-focus`, `p-disabled`; `aria-level|setsize|posinset|label|selected`, `id`) > `div.p-cascadeselect-option-content` > `span.p-cascadeselect-option-text` + `span.p-cascadeselect-group-icon` (у групп) и вложенный `ul.p-cascadeselect-list.p-cascadeselect-overlay.p-cascadeselect-option-list[role=group]` у активной группы.
- Поведение: клик по полю открывает/закрывает панель; клик по группе раскрывает её список, повторный — сворачивает; клик по листу выбирает его значение и закрывает панель; наведение раскрывает группы (до первого клика — корневые, после — любые); при открытии раскрывается путь к выбранному; клик вне и `resize` закрывают; `show_clear` очищает; на узком окне (`breakpoint`, `960px`) наведение путь не меняет.
- Клавиатура на скрытом поле: ↓ ↑ по списку уровня (с открытием группы под фокусом), → раскрывает группу и входит в неё, ← закрывает группу и возвращает на неё, `Alt+↑` закрывает, `Home`/`End`, `Enter`/`Space`, `Escape`, `Tab`, `Backspace` (очистка при `show_clear`), поиск по началу подписи.
- Свойства: `options`, `value`, `placeholder`, `show_clear`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `loading`, `breakpoint`, `focus_on_hover`, `select_on_focus`, `auto_option_focus`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `panel_style_class`; события `on_change`, `on_group_change`, `on_show`, `on_hide`, `on_clear`, `on_focus`, `on_blur`; представления `value_view` и `option`.
- Страница `/cascadeselect` (`basic`, `clearicon`, `disabled`, `invalid`, `loading`, `template`), e2e, бенчмарки и бюджеты, `docs/cascadeselect.md`, `cascadeselect` в `theme.components` и `primeng_extra`.
- **Не входит:** произвольные объекты опций (`optionLabel`/`optionValue`/`optionGroupLabel`/`optionGroupChildren`/`dataKey`), формы Angular, `appendTo`, `Ripple`, `Bind`, `pTemplate`, шаблоны `header`/`footer`/иконок, отзеркаливание вложенного списка у края окна (`position()`), `onBeforeShow`/`onBeforeHide`, сообщения для скринридеров (`aria-live`).

## Capabilities

### New Capabilities
- `cascadeselect`: выбор значения из каскада вложенных списков.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-cascadeselect` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-icons` (chevrondown, angleright, times, spinner).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
