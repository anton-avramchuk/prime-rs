# SplitButton

Порт `p-splitbutton` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-splitbutton::SplitButton`, `SplitButtonPt` | `packages/primeng/src/splitbutton/splitbutton.ts`, `splitbutton/style/splitbuttonstyle.ts` |
| CSS | `@primeuix/styles/splitbutton` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет); кнопки и меню берут стили `button` и `menu` |
| Демонстрации | `apps/showcase/doc/splitbutton/*-doc.ts` (`basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`) |

Спека: `openspec/specs/splitbutton`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-splitbutton.p-component` (`p-splitbutton-raised`, `-rounded`, `-outlined`, `-text`, `-sm`/`-lg`, `data-p-severity`) > основная кнопка `button.p-button.p-splitbutton-button` > кнопка раскрытия `button.p-button.p-splitbutton-dropdown.p-button-icon-only` > всплывающее меню.
- `severity`, `text`, `outlined`, `plain`, `size` передаются обеим кнопкам; `raised` и `rounded` действуют классами корня.
- Клик по основной кнопке: `on_click`, затем закрытие меню. Клик по стрелке: `on_dropdown_click`, затем переключение меню; ↓ и ↑ на стрелке делают то же и отменяют действие по умолчанию.
- Меню выравнивается по всему компоненту (в PrimeNG `currentTarget` — хост), а не по стрелке; `aria-expanded` стрелки следует за показом меню (`on_menu_show`/`on_menu_hide`), `aria-controls` указывает на меню, `aria-haspopup="true"`.
- `content` заменяет содержимое основной кнопки; `dropdown_icon` — стрелку; `disabled` блокирует обе кнопки, `button_disabled`/`menu_button_disabled` — по одной.

## Отличия

| Отличие | Причина |
|---|---|
| Меню — плоский `prime_menu::Menu` с группами, а не `TieredMenu`; демонстрация `nested` не перенесена | `TieredMenu` ещё не портирован (последний компонент волны 4). Когда он появится, внутренний `Menu` заменится без изменения API `SplitButton`. |
| Кнопка раскрытия нарисована самим `SplitButton`, а не `Button` | `Button` не принимает реактивные `aria-expanded`/`aria-controls`; классы и `data-p` берутся из функций `prime-button`, чтобы не разойтись с ним. Ripple на ней подключается так же, как у `Button`. |
| Корень — `<div>`, а не хост `<p-splitbutton>` | Хост в Angular — деталь фреймворка. |
| Значки — `ViewFn` (`icon`, `dropdown_icon`), а не классы `pi pi-…`; `content` — `ViewFn` | Нет PrimeIcons и `ng-template`. |
| Нет `tooltip`/`tooltipOptions`, `buttonProps`/`menuButtonProps`, `appendTo`, `dir`, `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` | Подсказку можно добавить снаружи директивой `use:tooltip`; меню всегда внутри корня (как у остальных оверлеев проекта); анимация — классы темы. |
| `button_aria_label`/`expand_aria_label` вместо полей `buttonProps.ariaLabel`/`menuButtonProps.ariaLabel`/`expandAriaLabel` | Плоские строковые свойства. |
| В демонстрациях значки `pi-check`, `pi-cog` заменены `CheckIcon`, `RefreshIcon`; логотип в `template` — текстом; `reversedkeys` пропущена | В `prime-icons` нет `cog`; в PrimeNG `reversedkeys` — текст про токены темы, не про компонент. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Положение меню при нехватке места снизу — как у `Menu`, подробности в `docs/menu.md`.

## Производительность

См. `benches/` (`splitbutton-1`, `splitbutton-1000`, `case-splitbutton`) и `benches/*budgets.toml`.
