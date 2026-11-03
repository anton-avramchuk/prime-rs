# Proposal

## Why

Image — пятый компонент волны 4: картинка с необязательным просмотром в модальном слое (поворот, масштаб, закрытие). Опирается на уже готовые `Portal`, блокировку прокрутки, ловушку фокуса, стек `z-index` и значки, поэтому добавляет мало нового.

## What Changes

- Новый крейт `prime-image` (порт `p-image` из PrimeNG 21.1.10).
- `Image`: корень `div.p-image.p-component` (`p-image-preview` при `preview`) > `img` (`src`, `srcset`, `sizes`, `alt`, `width`, `height`, `loading`, класс и стиль) > при `preview` кнопка `button.p-image-preview-mask` со значком глаза (`aria-label` из перевода `aria.zoomImage`).
- Просмотр: по клику на кнопку в `Portal` выводится `div.p-image-mask.p-overlay-mask[role=dialog]` с панелью `div.p-image-toolbar` (поворот вправо и влево, уменьшение, увеличение, закрытие; кнопки `p-image-action …`, у границ масштаба `p-disabled` и `disabled`) и увеличенной картинкой `img.p-image-original` с `transform: rotate(..deg) scale(..)`. Масштаб от 0.5 до 1.5 с шагом 0.1.
- Закрытие: кнопка, клик по маске (но не после действия на панели или клика по картинке), `Escape` (фокус возвращается на кнопку просмотра). Блокировка прокрутки страницы, слой по стеку `modal`, ловушка фокуса, фокус на кнопке закрытия; при закрытии поворот и масштаб сбрасываются.
- Свойства: `src`, `srcset`, `sizes`, `preview_image_src`, `preview_image_srcset`, `preview_image_sizes`, `alt`, `width`, `height`, `loading` (`ImageLoading`), `preview`, `image_class`, `image_style`, `style_class`, `style`, `pt`; представления `image`, `indicator`, `preview_content`, значки панели (`rotate_right_icon`, `rotate_left_icon`, `zoom_out_icon`, `zoom_in_icon`, `close_icon`) как `ViewFn`; события `on_show`, `on_hide`, `on_image_error`.
- Страница showcase `/image` с демонстрациями `basic`, `preview`, `previewimagesource`, `template`; e2e; бенчмарки и бюджеты; `docs/image.md`.
- Стили: `image` добавляется в `theme.components`.
- **Не входит:** `appendTo` (просмотр всегда в `Portal` у `body`), `showTransitionOptions`/`hideTransitionOptions`/`modalEnterAnimation`/`motionOptions`/`maskMotionOptions` (анимация — классы темы), `Bind`, `pTemplate`, `SafeUrl`.

## Capabilities

### New Capabilities
- `image`: изображение с просмотром в модальном слое, поворотом и масштабом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-image` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-icons` (eye, refresh, undo, searchminus, searchplus, times), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
