# image Specification

## Purpose
Image показывает картинку и, по настройке, открывает её в модальном слое с поворотом и масштабом.

## Requirements

### Requirement: Разметка
`Image` крейта `prime-image` SHALL выводить `div.p-image.p-component` (`p-image-preview` при `preview`; `style_class`, `style`, `pt`) с `img`, у которого заданные `src`, `srcset`, `sizes`, `alt`, `width`, `height`, `loading` (`lazy` или `eager`), класс `image_class` и стиль `image_style` выведены атрибутами, а незаданные опущены. Представление `image` SHALL заменять `img` и получать обратный вызов ошибки. При `preview` SHALL выводиться `button.p-image-preview-mask[type=button]` с `aria-label` из перевода `aria.zoomImage` и размерами `width × height`, содержащая значок глаза `p-image-preview-icon`; представление `indicator` SHALL заменять значок.

#### Scenario: Без просмотра
- **WHEN** отрисован `Image` с `src` и `alt`, `preview` не задан
- **THEN** выведен `img` с этими атрибутами, кнопки просмотра нет

#### Scenario: С просмотром
- **WHEN** задан `preview = true`
- **THEN** у корня класс `p-image-preview`, после `img` выведена кнопка `p-image-preview-mask` со значком

### Requirement: Открытие просмотра
Клик по кнопке просмотра SHALL открывать в `Portal` маску `div.p-image-mask.p-overlay-mask[role=dialog][aria-modal=true]` со слоем `modal` из общего стека `z-index`, блокировать прокрутку страницы, ставить ловушку фокуса и после показа переносить фокус на кнопку закрытия, затем вызывать `on_show`. Внутри маски SHALL быть панель `div.p-image-toolbar` и картинка `img.p-image-original`, чей `src` — `preview_image_src` или, если не задан, `src` (аналогично `srcset` и `sizes` из `preview_image_srcset`/`preview_image_sizes`). Представление `preview_content` SHALL заменять картинку и получать класс `p-image-original`, стиль с `transform` и обратный вызов клика по картинке.

#### Scenario: Открытие
- **WHEN** пользователь кликает по кнопке просмотра
- **THEN** в `body` выведена маска с панелью и увеличенной картинкой, прокрутка страницы заблокирована, фокус на кнопке закрытия

#### Scenario: Другой источник
- **WHEN** задан `preview_image_src`
- **THEN** увеличенная картинка берёт его, а не `src`

### Requirement: Панель действий
Панель SHALL содержать кнопки в порядке: поворот вправо (`p-image-rotate-right-button`), поворот влево (`p-image-rotate-left-button`), уменьшение (`p-image-zoom-out-button`), увеличение (`p-image-zoom-in-button`), закрытие (`p-image-close-button`); у каждой класс `p-image-action`, `type=button` и `aria-label` из переводов `aria.rotateRight`, `aria.rotateLeft`, `aria.zoomOut`, `aria.zoomIn`, `aria.close`; значки заменяются свойствами `rotate_right_icon`, `rotate_left_icon`, `zoom_out_icon`, `zoom_in_icon`, `close_icon`. Повороты SHALL менять угол на ±90°, масштаб — на ±0.1 в пределах от 0.5 до 1.5; картинка SHALL получать `transform: rotate(<угол>deg) scale(<масштаб>)`. Кнопка уменьшения (увеличения) SHALL получать `disabled` и класс `p-disabled`, когда следующий шаг не оставляет масштаба внутри предела. Клики по панели SHALL не закрывать просмотр.

#### Scenario: Поворот и масштаб
- **WHEN** нажаты «повернуть вправо» и «увеличить»
- **THEN** у картинки `transform: rotate(90deg) scale(1.1)`, просмотр остаётся открытым

#### Scenario: Предел масштаба
- **WHEN** масштаб доведён кнопкой «увеличить» до верхнего предела
- **THEN** кнопка «увеличить» отключена и имеет класс `p-disabled`

### Requirement: Закрытие
Просмотр SHALL закрываться кнопкой закрытия, кликом по маске (но не по панели и не по увеличенной картинке) и клавишей `Escape`; при `Escape` фокус SHALL возвращаться на кнопку просмотра. После окончания анимации выхода SHALL сниматься слой, ловушка фокуса и блокировка прокрутки, поворот и масштаб — сбрасываться (0° и 1), маска — удаляться из DOM, и вызываться `on_hide`. Повторное закрытие и закрытие незавершённого показа SHALL не разблокировать прокрутку дважды.

#### Scenario: Клик по маске
- **WHEN** открыт просмотр и пользователь кликает по маске вне картинки и панели
- **THEN** просмотр закрывается, прокрутка разблокирована, вызван `on_hide`

#### Scenario: Клик по картинке
- **WHEN** пользователь кликает по увеличенной картинке
- **THEN** просмотр остаётся открытым

#### Scenario: Сброс
- **WHEN** просмотр с поворотом на 90° закрыт и открыт снова
- **THEN** картинка снова без поворота и в масштабе 1

### Requirement: Ошибка загрузки и стили
При ошибке загрузки основной картинки SHALL вызываться `on_image_error` с событием. Стили темы SHALL подключаться набором `image` (`cargo xtask gen-theme`).

#### Scenario: Ошибка
- **WHEN** `img` сообщает об ошибке загрузки
- **THEN** вызывается `on_image_error`
