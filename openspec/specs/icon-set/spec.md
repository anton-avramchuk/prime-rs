# icon-set Specification

## Purpose
Набор SVG-иконок PrimeNG в крейте prime-icons: подключение по feature-флагам, доступность, воспроизводимая генерация.

## Requirements

### Requirement: Иконки PrimeNG в одном крейте
Система SHALL предоставлять крейт `prime-icons`, содержащий 53 SVG-иконки из `packages/primeng/src/icons` (без `baseicon` и `blank`) с теми же именами (например, `CheckIcon`, `ChevronDownIcon`), данными контура и `viewBox`, что в PrimeNG.

#### Scenario: Совпадение с PrimeNG
- **WHEN** иконка `check` отрисована
- **THEN** её SVG содержит тот же `d` и `viewBox`, что и исходник PrimeNG

### Requirement: Подключение иконок по feature-флагам
Каждая иконка SHALL быть доступна только при включённом собственном feature-флаге, чтобы неиспользуемые иконки не попадали в сборку.

#### Scenario: Иконка не включена
- **WHEN** feature иконки не включён
- **THEN** её тип недоступен, а размер wasm не включает её данные

### Requirement: Общее поведение иконок
Все иконки SHALL поддерживать размер, режим вращения (`spin`), пользовательский класс и a11y-атрибуты: по умолчанию `aria-hidden="true"`, при заданной подписи `role="img"` и `aria-label`.

#### Scenario: Декоративная иконка
- **WHEN** иконка отрисована без подписи
- **THEN** у `svg` есть `aria-hidden="true"` и нет `role`

#### Scenario: Иконка с подписью
- **WHEN** задан `aria-label`
- **THEN** у `svg` есть `role="img"` и `aria-label`, а `aria-hidden` отсутствует

### Requirement: Воспроизводимая генерация
Код иконок SHALL создаваться командой генерации из исходников PrimeNG, а повторный запуск без изменений в исходниках SHALL не менять файлы.

#### Scenario: Повторная генерация
- **WHEN** команда генерации запускается второй раз без изменений
- **THEN** `git diff` пуст
