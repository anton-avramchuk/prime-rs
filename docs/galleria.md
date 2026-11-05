# Galleria

Порт `p-galleria` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-galleria::Galleria`, `GalleriaPt`, `GalleriaImage`, `GalleriaPosition`, `GalleriaResponsive` | `packages/primeng/src/galleria/galleria.ts` (`Galleria`, `GalleriaContent`, `GalleriaItem`, `GalleriaThumbnails`), `galleria/style/galleriastyle.ts` |
| Окно миниатюр, соседние индексы, адаптивное число видимых, классы | `prime-galleria::model` — чистый порт `onAfterContentChecked`, `getMedianItemIndex`, `calculatePosition` и др., табличные тесты |
| CSS | `@primeuix/styles/galleria` (`galleria` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/galleria/*-doc.ts` (`basic`, `autoplay`, `caption`, `controlled`, `thumbnail`, `indicator`, `fullscreen`) |

Спека: `openspec/specs/galleria`. Активный индекс — `RwSignal<usize>`, видимость в полноэкранном режиме — `RwSignal<bool>`.

```rust
let active = RwSignal::new(0);
view! { <Galleria value=images active_index=active num_visible=5 circular=true show_item_navigators=true /> }
```

## Что совпадает с PrimeNG

- Формулы смещения ленты миниатюр (`shifted_items`), медианный индекс, число страниц, доступность кнопок; классы `p-galleria-thumbnail-item-current/-active/-start/-end`.
- Навигация: кнопки кадра, индикаторы (клик, `Enter`/`Space`, наведение), миниатюры (клик, `Enter`/`Space`, `ArrowLeft`/`ArrowRight`/`Home`/`End`, `Tab`), кнопки страниц, `circular`.
- Автосмена с остановкой по действию пользователя (`should_stop_autoplay_by_click`).
- Позиции миниатюр и индикаторов, `show_indicators_on_item`, `show_item_navigators_on_hover`, вертикальные миниатюры.
- Полноэкранный режим: маска `p-galleria-mask` в `body`, кнопка закрытия, закрытие кликом по маске, блокировка прокрутки, ловушка фокуса, анимация выхода.

## Отличия

| Отличие | Причина |
|---|---|
| Данные — `Vec<GalleriaImage>` (`item_image_src`, `thumbnail_image_src`, `alt`, `title`), шаблоны — `Callback<GalleriaImage, AnyView>`; без шаблона кадр и миниатюра выводятся как `<img>` | В Leptos нет `ng-template`; типизированная модель вместо `any`. |
| Смещение ленты — функция `(активный индекс, число видимых)`, отдельного шага `step` нет | В PrimeNG `step()` всегда перекрывается пересчётом при смене индекса. |
| Миниатюры получают `flex: 1 0 (100/n)%` встроенным стилем, а не динамическим `<style>` с медиа-запросами | Адаптивность считается в Rust по ширине окна (`resize`). |
| Нет системного полного экрана (`requestFullscreen`, `fullscreenchange`), `Bind`, форм Angular, опций анимации | Вне объёма порта. |
| Нет жестов смахивания миниатюр | Касание не проверялось на устройстве. |
| Нет `Escape` в полноэкранном режиме | Так же в PrimeNG. |

## Что не проверялось

- Касание и смахивание на реальном устройстве.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Раскладка справа налево.

## Производительность

См. `benches/` (`galleria-1`, `galleria-1000`, `case-galleria`) и `benches/*budgets.toml`.
