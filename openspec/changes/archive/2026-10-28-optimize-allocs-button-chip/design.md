# Design

## Context

`render-bench`: Button 19, полная Button 45, Chip 10, removable Chip 35 аллокаций на экземпляр. Основные статьи: `Arc<Props>` + `Arc<Pt>` (2), `NodeRef` (арена + сигнал), `.into_any()` на каждый блок, `Pt::new()` и временные строки в `style_of` Chip.

## Decisions

### D1. `Arc` только в реактивной ветви
`Props` и `Pt` живут на стеке до развилки. Статическая ветвь заимствует их; реактивная оборачивает в `Arc` для замыканий. Обработчик клика статической кнопки получает `off = props.is_disabled()` вместо `Arc<Props>`.

### D2. `NodeRef` только на клиенте
Нужен для автофокуса и ripple (`setup_client`), которые существуют лишь на `wasm32`. Атрибут подключается спредом `{..node}`: на `wasm32` это `node_ref(...)`, на нативной цели — `()`.

### D3. Без `.into_any()`
Индикатор Button — `EitherOf4`; метка, бейдж, изображение, иконка, метка и элемент удаления Chip — `Option<impl IntoView>`/`Either`.

### D4. Стиль Chip
`Pt::merge_style_parts(&[...])` собирает части в одну строку; без `style`, `pt.style` и в видимом состоянии возвращается `None` без аллокаций.

## Risks / Trade-offs

- [Две формы обработчика клика] → покрыто SSR-, паритет- и браузерными тестами.

## Migration Plan

Внутренний рефакторинг; откат — `git revert`.

## Open Questions

Нет.
