# Design

## Context

Требования — в `specs/image`. Исходники PrimeNG 21.1.10: `image/image.ts` (около 550 строк), `image/style/imagestyle.ts`; стили `@primeuix/styles/image`; демонстрации `apps/showcase/doc/image/{basic,preview,previewimagesource,template}-doc.ts`. Образцы в проекте: `prime-dialog` (`Portal`, блокировка прокрутки, `z-index`, `FocusTrap`, `Escape`, стадии показа и скрытия), `prime-blockui` (маска и общий стек), `prime-imagecompare` (картинки, `Pt`).

Наблюдения по исходнику:
- Корневой класс — `p-image p-component` + `p-image-preview`; `img` без обёртки; кнопка просмотра имеет размер `width × height` (строки с `px`, как в PrimeNG).
- `onImageClick` включает `maskVisible`/`previewVisible`, строит маску и просмотр, блокирует прокрутку. `onMaskClick` закрывает, если не был `previewClick`; действия панели и клик по картинке выставляют `previewClick = true`, поэтому клик по ним маску не закрывает; `handleToolbarClick` гасит всплытие.
- `Escape` на маске (и глобальный `document:keydown.escape`): закрыть; на маске фокус затем возвращается на кнопку просмотра (через 25 мс).
- После показа (`onAnimationStart`): слой `modal` по `zIndex`, `onShow`, фокус на кнопке закрытия через 25 мс. После скрытия: снять слой, `rotate = 0`, `scale = 1`, разблокировать прокрутку, `onHide`.
- Масштаб: `step 0.1`, `min 0.5`, `max 1.5`; кнопки блокируются, когда следующий шаг выходит за предел (`scale ± step <= min`/`>= max`; в плавающей арифметике сравнение делаем с допуском).
- Анимации маски задаются классами `p-overlay-mask-enter-active`/`-leave-active`; для картинки в теме 21 — через `p-motion` с именем `p-image-original`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роли и `aria-*`; тот же просмотр с поворотом, масштабом и закрытием; блокировка прокрутки и слои как у остальных модальных компонентов.

**Non-Goals:** `appendTo`, переходы через `motionOptions`, `Bind`, `pTemplate`, `SafeUrl`.

## Decisions

### D1. Крейт и API
`prime-image` с `Image`, `ImageLoading {Lazy, Eager}`, `ImagePt`. Свойства — `Prop`/`OptProp`, как в остальных крейтах; представления — `ViewFn`; `preview` (шаблон) получает `ImagePreviewContext { class, style, preview_callback }` — как контекст PrimeNG; `image` получает `error_callback`.

### D2. Чистая логика (`state.rs`)
`root_classes(preview, extra)`, `zoom_in_disabled(scale)`, `zoom_out_disabled(scale)`, `zoom_in(scale)`/`zoom_out(scale)` (с округлением до сотых), `transform(rotate, scale)`, `action_classes(kind, disabled)`, `mask_should_close(preview_click)`. Табличные тесты, включая границы масштаба.

### D3. Состояние просмотра
`RwSignal` для `open` (показ запрошен), `rendered` (слой в DOM, пока идёт выход), `rotate`, `scale`, флаг `preview_click` (не сигнал). Структура ядра — по образцу `prime-dialog`: `StoredValue` для слоя `z-index`, ловушки фокуса и признака блокировки; повторное закрытие и двойная блокировка прокрутки защищены флагами (урок BlockUI).

### D4. Показ
`Portal` строится заново при каждом показе (готовые представления не `Send + Sync`). Маска получает классы `p-overlay-mask-enter-active`/`-leave-active`; выход ждёт `animationend` с запасным таймером, как в `Dialog`. Слой — `prime_core::zindex::generate(ZGroup::Modal, ..)`.

### D5. Клавиши и фокус
`Escape` — слушатель `keydown` на маске и на `document` (пока открыто): закрыть и вернуть фокус на кнопку просмотра через `set_timeout(25)`. `FocusTrap` на маске; фокус на кнопке закрытия через `set_timeout(25)`.

### D6. Надписи
`aria-label` кнопок — из `use_prime_config()` (`aria.zoomImage`, `aria.rotateRight`, `aria.rotateLeft`, `aria.zoomIn`, `aria.zoomOut`, `aria.close`); проверить, что ключи есть в `translation_defaults.rs`, недостающие — добавить.

### D7. Ошибка загрузки
`on_error` у `img` вызывает `on_image_error`; в SSR обработчик не вешается.

### D8. CSS
`image` добавляется в `theme.components`; проверить `p-image*` в пресете.

### D9. Тесты
SSR: разметка, `preview`/без, атрибуты `img`, `image`-шаблон, `indicator`, `pt`, `style_class`, `aria-label`. Браузерные `tests/web.rs` (один последовательный тест из-за общего стека `z-index` и счётчика блокировки прокрутки): открытие, состав панели, поворот и масштаб (`transform`, `disabled` на границах), закрытие кнопкой/маской/`Escape`, клик по картинке и панели не закрывает, сброс поворота и масштаба, фокус, блокировка прокрутки, `on_show`/`on_hide`. e2e: стили темы (кнопка просмотра над картинкой, глаз при наведении, маска поверх страницы), полный сценарий просмотра.

### D10. Демонстрации и бенчмарки
Картинки PrimeNG с CDN в демонстрациях не используются: локальный SVG (градиент) в `showcase/assets`; `template` с кастомными `indicator`, `image`, `preview`. Бенч `image-1`/`-1000` и `case-image`.

## Risks / Trade-offs

- [Анимация выхода] Если тема не задаёт `animationend`, нужен запасной таймер, иначе слой не уберётся.
- [Дробный масштаб] Накопленная ошибка плавающей арифметики; округляем до сотых.
- [Общие слушатели] `Escape` на `document` не должен срабатывать у закрытого просмотра; снимаем при закрытии и в `on_cleanup`.

## Open Questions

- Нужен ли `appendTo`: все остальные оверлеи проекта обходятся без него.

## Migration Plan

Миграции нет. Откат — `git revert`.
