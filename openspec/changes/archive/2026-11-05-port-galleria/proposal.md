# Proposal

## Why

Galleria — галерея изображений: крупный кадр, миниатюры с прокруткой, индикаторы, подписи, автоматическая смена и полноэкранный режим. Второй компонент волны 6. Логика окна миниатюр (смещение, медианный элемент, постраничная навигация, адаптивное число видимых) — самостоятельная чистая модель; оболочка строится на готовых `Portal`/`FocusTrap`/`block_body_scroll`, как у Drawer.

## What Changes

- Новый крейт `prime-galleria` (порт `p-galleria` из PrimeNG 21.1.10).
- Чистая модель `model.rs`: окно миниатюр (`median_index`, `total_pages`, смещение по активному индексу, доступность кнопок, видимость миниатюр), следующий/предыдущий индекс кадра, выбор `num_visible` по ширине окна (`responsive_options`), классы позиций.
- `Galleria`: `div.p-galleria.p-component` (`role=region`; в полноэкранном режиме — внутри `div.p-galleria-mask.p-overlay-mask` в `Portal`, `role=dialog`, `aria-modal`) > кнопка закрытия, `header`, `.p-galleria-content` > `.p-galleria-items-container` (`.p-galleria-items` > кнопки навигации, `.p-galleria-item`, `.p-galleria-caption`; список индикаторов) и `.p-galleria-thumbnails` (`.p-galleria-thumbnails-content` > кнопки, `-viewport` > `-thumbnail-items` > `-thumbnail-item` > `.p-galleria-thumbnail`), `footer`.
- Данные — `Vec<GalleriaImage>` (`item_image_src`, `thumbnail_image_src`, `alt`, `title`); свои шаблоны — `item`, `thumbnail`, `caption`, `indicator`, `header`, `footer` как `Callback`/`ViewFn`.
- Поведение: активный индекс — `RwSignal<usize>`; кнопки кадра (`show_item_navigators`, `circular`), индикаторы (клик, `Enter`/`Space`, наведение при `change_item_on_indicator_hover`), миниатюры (клик, `Enter`/`Space`, стрелки/`Home`/`End`, кнопки страниц, касание), автосмена (`auto_play`, `transition_interval`, остановка по клику при `should_stop_autoplay_by_click`), позиции миниатюр и индикаторов, `show_indicators_on_item`, `show_item_navigators_on_hover`, вертикальные миниатюры, адаптивные `num_visible`, полноэкранный режим (`visible`, маска, закрытие по маске и кнопке, блокировка прокрутки, ловушка фокуса).
- Страница `/galleria` (`basic`, `autoplay`, `caption`, `controlled`, `thumbnail`, `responsive`, `advanced`), e2e, бенчмарки и бюджеты, `docs/galleria.md`, `galleria` в `theme.components`.
- **Не входит:** `fullscreenchange` документа и системный полный экран (`requestFullscreen`), `Bind`, формы Angular, `maskMotionOptions`/`motionOptions`/`showTransitionOptions`, `numVisibleLimit` заменено ограничением `min(num_visible, len)`.

## Capabilities

### New Capabilities
- `galleria`: галерея изображений с миниатюрами, индикаторами, автосменой и полноэкранным режимом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-galleria` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
