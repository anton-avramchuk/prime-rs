# Proposal

## Why

BlockUI — второй компонент волны 4: полупрозрачная маска, блокирующая либо весь документ, либо выбранный элемент (панель, карточку), с произвольным содержимым по центру (например, индикатор загрузки). Он нужен как основа для состояний «идёт загрузка» и использует уже готовые общие части: слои `ZGroup::Modal`, счётчик блокировки прокрутки страницы и `Portal`.

## What Changes

- Новый крейт `prime-blockui` (порт `p-blockui` из PrimeNG 21.1.10).
- `BlockUI`: пока `blocked = false`, в DOM ничего нет; при `blocked = true` в целевой элемент (`target`) либо в `body` (документ) выводится маска `div.p-blockui.p-blockui-mask.p-overlay-mask.p-overlay-mask-enter-active` (в режиме документа ещё `p-blockui-mask-document`) с `display: flex`, `aria-busy="true"` и содержимым `children`; при снятии играет анимация выхода (`p-overlay-mask-leave-active`, `aria-busy="false"`), затем маска убирается.
- Свойства: `blocked` (значение или сигнал), `target` (`BlockTarget` из `NodeRef` любого элемента; без него блокируется документ), `auto_z_index` (да), `base_z_index` (0), `style`, `style_class`, `pt`, `children`.
- Поведение: в режиме цели целевому элементу временно ставится `position: relative` (прежнее значение возвращается после выхода маски); в режиме документа блокируется прокрутка страницы (`prime_dom::block_body_scroll`, счётчик с Dialog/Drawer); при `auto_z_index` маска получает слой группы `Modal` со сдвигом `base_z_index`; повторное включение во время выхода возвращает маску в состояние «заблокирован»; при размонтировании слой и прокрутка освобождаются.
- Страница showcase `/blockui` с демонстрациями `basic` (маска над панелью) и `document` (маска над документом на три секунды); e2e; бенчмарки и бюджеты; `docs/blockui.md`.
- Стили: `blockui` добавляется в `theme.components`.
- **Не входит:** проверка `BlockableUI` (в Rust цель — любой `NodeRef`), выбор через шаблон `content` (содержимое — `children`), `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `blockui`: маска, блокирующая документ или выбранный элемент.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-blockui` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
