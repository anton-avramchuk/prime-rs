# Tasks

## 1. Основа

- [x] 1.1 `galleria` в `theme.components`, `cargo xtask gen-theme`; есть `p-galleria-thumbnail-item`, `p-items-hidden`
- [x] 1.2 Свериться с `GalleriaThumbnails` (`onAfterContentChecked`, `navForward/Backward`, `onItemClick`, клавиши), `GalleriaItem`, `GalleriaContent` (автосмена) исходника

## 2. Крейт `prime-galleria`

- [x] 2.1 Модель `model.rs` с табличными тестами
- [x] 2.2 Компонент `Galleria`, `GalleriaPt`, `GalleriaImage`: разметка, индикаторы, подпись, миниатюры, SSR-тесты, rustdoc
- [x] 2.3 Навигация: кнопки кадра, индикаторы, миниатюры, клавиши, `circular`, касание
- [x] 2.4 Автосмена, адаптивные `num_visible`
- [x] 2.5 Полноэкранный режим: `Portal`, маска, закрытие, прокрутка, фокус, выход
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/galleria`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `galleria.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-galleria`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/galleria.md`; `prime-galleria` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-galleria`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; коммит и архив локально.
