# TreeSelect

Порт `p-treeselect` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-treeselect::TreeSelect`, `TreeSelectPt`, `TreeSelectDisplay` | `packages/primeng/src/treeselect/treeselect.ts`, `treeselect/style/treeselectstyle.ts` |
| Панель | готовый `prime_tree::Tree` (общий сигнал выбора и раскрытых ключей) |
| CSS | `@primeuix/styles/treeselect`, пресет Aura и блок `/* For PrimeNG */` — `cargo xtask gen-theme` (`treeselect` в `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/treeselect/*-doc.ts` (`basic`, `multiple`, `checkbox`, `filter`, `clearicon`, `disabled`, `invalid`, `template`) |

Спека: `openspec/specs/treeselect`. Значение — `RwSignal<Vec<String>>` ключей выбранных узлов (в `Single` — 0 или 1 ключ); узлы — `prime_tree::TreeNode`.

```rust
let value = RwSignal::new(Vec::<String>::new());
view! { <TreeSelect options=nodes value=value selection_mode=TreeSelectionMode::Multiple display=TreeSelectDisplay::Chip /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-treeselect.p-component.p-inputwrapper` (состояния: `-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`, `-open`, `-clearable`, `-fluid`, `-sm`, `-lg`) > скрытое поле `input[role=combobox][readonly]` > `p-treeselect-label-container` > `p-treeselect-label` (запятая, чипы, подсказка) > значок очистки > `p-treeselect-dropdown` > панель `p-treeselect-overlay` > `p-treeselect-tree-container` (`max-height` 400px) > `Tree`.
- Клик по полю открывает/закрывает панель; `Single` закрывает панель после выбора и возвращает фокус; при открытии раскрываются предки выбранных узлов; клик вне, `resize` и `Escape` закрывают; после раскрытия узла панель перевыравнивается.
- Клавиатура скрытого поля: ↓ открывает и фокусирует первый узел, `Space`/`Enter` открывают, `Escape` закрывает, `Tab` закрывает.
- Фильтр, режимы выбора, `meta_key_selection`, распространение флажков и пустое сообщение работают через `Tree`.

## Отличия

| Отличие | Причина |
|---|---|
| Значение — ключи, а не узлы (в PrimeNG `Single` — узел, остальное — массив узлов) | Модель `Tree` неизменяема и хранит выбор ключами. |
| Нет формы Angular (`ngModel`, реактивные формы), `appendTo`, виртуальной прокрутки, `resetFilterOnHide` (фильтр сбрасывается при каждом закрытии, так как панель пересоздаётся) | Вне объёма порта. |
| Нет скрытых фокус-ловушек панели; `Tab` на поле закрывает панель | Упрощение; фокус внутри панели уходит клавишами дерева и кликом. |
| Нет шаблонов `header`/`footer`/`empty`/иконок; есть `value_view` (замена метки) и `node` (подпись узла) | Нет `pTemplate`. |
| Метка пустого поля без подсказки — слово `empty` под `p-treeselect-label-empty` (скрыта темой) | Так же в PrimeNG. |
| Панель внутри корня (`position: absolute`), как у `Select` | В PrimeNG — отдельный `p-overlay`. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Положение панели у нижнего/правого края окна (общий `align_overlay`).

## Производительность

См. `benches/` (`treeselect-1`, `treeselect-1000`, `case-treeselect`) и `benches/*budgets.toml`.
