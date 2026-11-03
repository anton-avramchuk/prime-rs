# Image

Порт `p-image` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-image::Image`, `ImagePt`, `ImageLoading`, `ImagePreviewContext` | `packages/primeng/src/image/image.ts`, `image/style/imagestyle.ts` |
| CSS | `@primeuix/styles/image` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/image/*-doc.ts` (`basic`, `preview`, `previewimagesource`, `template`) |

Спека: `openspec/specs/image`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-image.p-component` (`p-image-preview`) > `img` и `button.p-image-preview-mask` со значком глаза `p-image-preview-icon` (`aria-label` — перевод `aria.zoomImage`, размер — `width × height`). Просмотр: `div.p-image-mask.p-overlay-mask[role=dialog][aria-modal]` > `div.p-image-toolbar` с пятью кнопками `p-image-action …` (поворот вправо и влево, уменьшение, увеличение, закрытие) и `img.p-image-original`.
- Поворот на ±90°, масштаб с шагом 0.1; картинка получает `transform: rotate(..deg) scale(..)`. Кнопки уменьшения и увеличения блокируются (`disabled` и `p-disabled`) по тому же условию, что в PrimeNG (`scale ± 0.1` не оставляет масштаб строго внутри 0.5–1.5), поэтому достижимы масштабы от 0.6 до 1.4.
- Закрытие кнопкой, кликом по маске и `Escape` (фокус возвращается на кнопку просмотра). Клик по увеличенной картинке и по панели не закрывает просмотр. Прокрутка страницы блокируется, маска получает слой группы `Modal`, внутри — ловушка фокуса, после показа фокус на кнопке закрытия. При закрытии поворот и масштаб сбрасываются.
- Свои представления: `indicator`, `image` (получает обратный вызов ошибки), `preview_content` (получает класс, стиль и обратный вызов клика по картинке) и значки панели; события `on_show`, `on_hide`, `on_image_error`.

## Отличия

| Отличие | Причина |
|---|---|
| После действия панели первый клик по маске закрывает просмотр. В PrimeNG флаг `previewClick`, выставленный кнопкой панели, остаётся (клик гасится в `handleToolbarClick`, до маски не доходит), и первый клик по маске после поворота или масштаба ничего не делает | Очевидная ошибка исходника: панель и так останавливает всплытие, флаг нужен только для клика по картинке. |
| `Escape` обрабатывается одним слушателем на `window` и только у верхнего слоя (просмотр над диалогом не закроет диалог) | В PrimeNG два обработчика (на маске и на `document`); здесь общий порядок слоёв, как у `Dialog`. |
| Нет `appendTo`: просмотр всегда в `Portal` у `body` | Остальные оверлеи проекта тоже без него. |
| Нет `showTransitionOptions`, `hideTransitionOptions`, `modalEnterAnimation`, `modalLeaveAnimation`, `motionOptions`, `maskMotionOptions` | Анимация задана классами темы (`p-overlay-mask-*-active`, `p-image-original-*-active`); выход ждёт `animationend` маски с запасным таймером 400 мс. |
| Значки — `ViewFn`, а не `ng-template`; представления вместо `pTemplate` | Нет `ng-template`. |
| `width` и `height` числом получают `px` у кнопки просмотра; процент и `rem` берутся как есть | В PrimeNG всегда `height + 'px'` (для `50%` стиль был бы неверным). |
| Нет `SafeUrl` | Адреса — строки. |
| Просмотр не отдаётся сервером и появляется после гидратации | Маска нужна только после клика. |
| В демонстрациях фотографии с CDN заменены встроенными SVG-картинками | Независимость от сети. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод (щипок для масштаба в PrimeNG тоже нет).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Просмотр над другим модальным слоем проверен только на уровне порядка `z-index`.

## Производительность

См. `benches/` (`image-1`, `image-1000`, `case-image`) и `benches/*budgets.toml`.
