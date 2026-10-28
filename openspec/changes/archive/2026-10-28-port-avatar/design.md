# Design

## Context

Требования — в `specs/avatar`. Исходники PrimeNG 21.1.10: `packages/primeng/src/avatar/avatar.ts`, `avatar/style/avatarstyle.ts`, демонстрации `apps/showcase/doc/avatar`. Хост `<p-avatar>` несёт классы, `aria-label`, `aria-labelledby`, `data-p`; внутри `ng-content` и один из трёх блоков (метка, иначе иконка, иначе изображение). `data-p` строится из формы и размера, у которых есть значения по умолчанию (`square`, `normal`), поэтому по умолчанию он равен `square normal`. Шаблон — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Avatar с теми же классами, `data-p`, порядком и приоритетом блоков; ветви без `RenderEffect` для статики; бюджеты.
**Non-Goals:** `AvatarGroup` (отдельный change), шрифтовые иконки.

## Decisions

### D1. Корень — `<div>`
Как у Chip: хост `<p-avatar>` — деталь Angular.

### D2. Перечисления с умолчаниями
`AvatarSize` (`Normal`, `Large`, `XLarge`) и `AvatarShape` (`Square`, `Circle`) реализуют `Default` (`Normal`, `Square`); свойства — `Prop<AvatarSize>`, `Prop<AvatarShape>`, значение по умолчанию статическое. Значения попадают в `data-p` всегда, как в PrimeNG.

### D3. Блоки
`label_of`, `icon_of`, `image_of` возвращают `Option<AnyView>` с приоритетом метка → иконка → изображение; `p-avatar-image` на корне определяется по `image.is_some()` (как `!= null` в PrimeNG).

### D4. Состояние и ветви
Чистые функции `write_classes`/`data_p` над `AvatarState`; ветвь по `all_static!(label, image, size, shape, style, style_class, aria_label, aria_labelledby)`.

## Risks / Trade-offs

- [Внешний вид зависит от CSS темы `avatar`] → CSS уже сгенерирован, подключается оболочкой showcase.
- [Расхождение ветвей] → параметризованный тест паритета.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
