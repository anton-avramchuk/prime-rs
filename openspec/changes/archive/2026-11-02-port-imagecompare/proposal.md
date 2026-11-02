# Proposal

## Why

ImageCompare — первый компонент волны 4: сравнение двух изображений ползунком (обрезка верхнего по `clip-path`). Он самый маленький из оставшихся компонентов (около 150 строк в PrimeNG), не зависит от других крейтов и позволяет запустить волну 4 с лёгкого шага.

## What Changes

- Новый крейт `prime-imagecompare` (порт `p-imagecompare` из PrimeNG 21.1.10).
- `ImageCompare`: корень `div.p-imagecompare` > левое представление (`left`, обычно `<img>`) > правое представление (`right`, обычно `<img>`) > `input[type=range].p-imagecompare-slider` (`min=0`, `max=100`, `value=50`).
- Свойства: `left` и `right` (`ViewFn`), `tabindex`, `aria_label`, `aria_labelledby` (на корне), `style`, `style_class`, `pt` (`root`, `slider`).
- Поведение: ввод в ползунок задаёт `clip-path: polygon(0 0, V% 0, V% 100%, 0 100%)` у элемента, стоящего непосредственно перед ползунком (правое изображение); в контейнере с `dir="rtl"` — `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)`. До первого ввода применяется правило темы (`50%`).
- Страница showcase `/imagecompare` с демонстрациями `basic` и `responsive`; e2e; бенчмарки и бюджеты; `docs/imagecompare.md`.
- Стили: `imagecompare` добавляется в `theme.components`.
- **Не входит:** наблюдатель `MutationObserver` за атрибутом `dir` (направление вычисляется при вводе — результат тот же, потому что в PrimeNG оно тоже применяется только при вводе), `Bind`, `pTemplate`, демонстрация `accessibility` (текст).

## Capabilities

### New Capabilities
- `imagecompare`: сравнение двух изображений с помощью ползунка.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-imagecompare` (`publish = false`); зависимости: `prime-core`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
