# scrolltop Specification

## Purpose
ScrollTop показывает кнопку возврата к началу страницы или прокручиваемого родителя после того, как прокрутка превысила порог.

## Requirements

### Requirement: Разметка ScrollTop
Компонент `ScrollTop` крейта `prime-scrolltop` SHALL, пока он виден, выводить `<button type="button">` компонента `Button` с классом `p-scrolltop`, скруглением (`rounded`, по умолчанию включено), значком в `.p-scrolltop-icon` (по умолчанию `ChevronUpIcon`) и `aria-label` из локали `aria.scrollTop`, если не задан `aria_label`. При `target = Parent` кнопка SHALL получать класс `p-scrolltop-sticky`. `style_class` SHALL добавляться к классам, `style` — задавать стиль, `pt` — добавлять классы, стиль и атрибуты кнопке. Пока прокрутка не превысила порог, при `target = Window` компонент SHALL не выводить ничего.

#### Scenario: Скрыт по умолчанию
- **WHEN** компонент с `target = Window` отрисован на сервере
- **THEN** разметка кнопки отсутствует

#### Scenario: Родитель
- **WHEN** `target = Parent`
- **THEN** кнопка имеет класс `p-scrolltop-sticky`

#### Scenario: Значок и подпись
- **WHEN** кнопка показана без `icon` и `aria_label`
- **THEN** внутри `.p-scrolltop-icon` лежит `ChevronUpIcon`, `aria-label` равен «Scroll Top»

### Requirement: Видимость ScrollTop
При `target = Window` компонент SHALL подписываться на `scroll` окна, при `Parent` — на `scroll` родительского элемента. Если прокрутка (`scrollY` окна или `scrollTop` родителя) строго больше `threshold` (по умолчанию 400), кнопка SHALL показываться; иначе скрываться. Прокрутка SHALL проверяться и сразу после подписки. Подписка SHALL сниматься при уничтожении компонента.

#### Scenario: Ниже порога
- **WHEN** окно прокручено на 400 или меньше
- **THEN** кнопки нет

#### Scenario: Выше порога
- **WHEN** окно прокручено на 401 или больше
- **THEN** кнопка показана

#### Scenario: Возврат
- **WHEN** кнопка показана и прокрутка вернулась к порогу или ниже
- **THEN** кнопка скрывается

#### Scenario: Родитель
- **WHEN** `target = Parent`, `threshold = 100`, родитель прокручен на 150
- **THEN** кнопка показана

#### Scenario: Страница уже прокручена
- **WHEN** компонент монтируется при окне, прокрученном на 800
- **THEN** кнопка показана без нового события `scroll`

### Requirement: Действие ScrollTop
Клик по кнопке SHALL прокручивать цель (окно или родителя) к началу (`top: 0`) с поведением `behavior` (`Smooth` по умолчанию).

#### Scenario: Прокрутка окна
- **WHEN** окно прокручено на 800 и пользователь нажимает кнопку
- **THEN** окно прокручивается к началу, после чего кнопка скрывается

#### Scenario: Клавиатура
- **WHEN** кнопка в фокусе и пользователь нажимает `Enter` или `Space`
- **THEN** выполняется то же действие, что и при клике

### Requirement: Положение и анимация ScrollTop
При `target = Window` кнопка SHALL иметь `position: fixed`, при `Parent` — `position: sticky`; z-index SHALL выдаваться из группы `Overlay` при показе и возвращаться при скрытии. Появление и исчезновение SHALL анимироваться классами `p-scrolltop-enter-from`/`-active`/`-to` и `p-scrolltop-leave-from`/`-active`/`-to`; после окончания исчезновения кнопка SHALL скрываться.

#### Scenario: Окно
- **WHEN** кнопка показана при `target = Window`
- **THEN** её `position` равен `fixed`, а z-index больше базы группы `Overlay`

#### Scenario: Освобождение z-index
- **WHEN** кнопка скрыта
- **THEN** выданный z-index возвращён менеджеру

### Requirement: Стоимость отрисовки ScrollTop
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `scrolltop-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `scrolltop` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ScrollTop не превышают бюджеты
