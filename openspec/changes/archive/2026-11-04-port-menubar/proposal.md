# Proposal

## Why

Menubar — второе меню волны 5 (после ContextMenu): горизонтальная панель с выпадающими подменю, на узком окне сворачивающаяся в кнопку-«гамбургер». Логика путей, фокуса, поиска и положения подменю уже есть в `prime-tieredmenu`; новое — горизонтальный корневой уровень (стрелки ← → ходят по корню, ↓ раскрывает группу), слоты `start`/`end`, мобильная кнопка и автоскрытие.

## What Changes

- Новый крейт `prime-menubar` (порт `p-menubar` из PrimeNG 21.1.10). Модель пунктов — `prime_menu::MenuItem`; чистая логика берётся из `prime-tieredmenu`, при необходимости расширяется функциями горизонтального корня.
- `Menubar`: корень `div.p-menubar.p-component` (`p-menubar-mobile` на узком окне, `p-menubar-mobile-active` при открытой кнопке; `id`, `style`, `style_class`) > `div.p-menubar-start` (слот `start`) + `a.p-menubar-button[role=button]` со значком «бар» (только при непустой модели; `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) + `ul.p-menubar-root-list[role=menubar]` > `li.p-menubar-item` > `div.p-menubar-item-content` > `a.p-menubar-item-link` (`p-menubar-item-icon`, `p-menubar-item-label`, `p-menubar-submenu-icon`) + вложенный `ul.p-menubar-submenu`; разделители `p-menubar-separator`; слот `end` в `div.p-menubar-end`. Атрибуты `aria-*`/`data-p-*` — как у `TieredMenu`.
- Поведение: клик по группе раскрывает/сворачивает её, лист вызывает `command` и закрывает всё; наведение раскрывает группы при `auto_display`; `auto_hide` закрывает подменю через `auto_hide_delay` (100 мс) после ухода курсора; клик вне закрывает подменю (и мобильную панель); `resize` закрывает.
- Клавиатура: ← → по корню, ↓ раскрывает группу корня и входит в неё, ↑ ↓ → ← внутри подменю как у `TieredMenu`, `Home`/`End`, `Enter`/`Space`, `Escape`, `Tab`, поиск по началу подписи. Кнопка меню: `Enter`/`Space` переключают.
- Мобильный режим (`breakpoint`, по умолчанию `960px`): кнопка показывает/скрывает корневой список (слой `Menu`), подменю раскрываются кликом, наведение путь не меняет.
- Свойства: `model`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `auto_display`, `auto_hide`, `auto_hide_delay`, `breakpoint`, `id`, `aria_label`, `aria_labelledby`; представления `start`, `end`, `item`, `submenu_icon`, `menu_icon`; события `on_focus`, `on_blur`.
- Страница `/menubar` (`basic`, `command`, `template`), e2e, бенчмарки и бюджеты, `docs/menubar.md`, `menubar` в `theme.components` (и в `primeng_extra`, если в исходнике стилей есть блок `For PrimeNG`).
- **Не входит:** `routerLink`, `Tooltip`/`Badge` пунктов, HTML-метки, `Ripple`, `Bind`, `pTemplate`, `appendTo`, анимационные опции, демонстрация `router`.

## Capabilities

### New Capabilities
- `menubar`: горизонтальное меню с подменю, клавиатурой и мобильным режимом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-menubar` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-menu`, `prime-tieredmenu`, `prime-icons` (angleright, angledown, bars).
- Возможно, правка `prime-tieredmenu::model` (функции горизонтального корня) — без изменения поведения `TieredMenu` и `ContextMenu`, их тесты остаются зелёными.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
