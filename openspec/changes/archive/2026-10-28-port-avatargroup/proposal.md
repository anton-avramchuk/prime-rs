# Proposal

## Why

AvatarGroup объединяет несколько Avatar в перекрывающуюся группу. В PrimeNG это отдельный пакет (`primeng/avatargroup`), поэтому по правилу «один компонент — один крейт» он выделяется в свой крейт; при порте Avatar он был намеренно отложен (`docs/avatar.md`).

## What Changes

- Новый крейт `prime-avatargroup` с компонентом `AvatarGroup`: дочернее содержимое, `style`, `style_class`, `pt` (`root`).
- Разметка и классы как в PrimeNG 21.1.10: корень `<div class="p-avatar-group p-component">`; расположение и перекрытие аватаров задаёт CSS темы (`.p-avatar-group`).
- Демонстрация `avatargroup` на странице `/avatar` в showcase, e2e, бенчмарки, бюджеты.
- **Не входит:** автоматическое сворачивание лишних аватаров в «+N» (в PrimeNG этого тоже нет — «+2» делается вручную).

## Capabilities

### New Capabilities
- `avatargroup`: компонент AvatarGroup — разметка, классы, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-avatargroup` (`publish = false`); зависимость только `prime-core`.
- Изменяются `showcase` (страница Avatar), `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`, `docs/avatar.md`.
