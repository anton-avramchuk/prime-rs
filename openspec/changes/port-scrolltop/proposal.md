# Proposal

## Why

ScrollTop — кнопка «наверх», которая появляется после прокрутки на заданную глубину. Это следующий компонент волны 2 (после InputOtp); он опирается на уже портированный Button и на подписку на `scroll` по образцу Slider/Select.

## What Changes

- Новый крейт `prime-scrolltop` с компонентом `ScrollTop` (порт `p-scrolltop` из PrimeNG 21.1.10): круглая кнопка `Button` с классами `p-scrolltop` (+ `p-scrolltop-sticky` при `target = Parent`) и значком `p-scrolltop-icon` (по умолчанию `ChevronUpIcon`).
- Свойства: `target` (`Window` по умолчанию | `Parent`), `threshold` (400), `behavior` (`Smooth` по умолчанию | `Auto`), `icon`, `aria_label`, `severity`, `raised`, `rounded` (по умолчанию `true`), `style`, `style_class`, `pt`.
- Видимость: подписка на `scroll` окна или родительского элемента; кнопка показывается при превышении порога и скрывается при возврате. Появление и исчезновение анимируются классами `p-scrolltop-enter-*`/`p-scrolltop-leave-*` из темы, z-index — группа `Overlay`; позиция `fixed` для окна и `sticky` для родителя.
- Клик прокручивает цель к началу (`scroll({ top: 0, behavior })`).
- Страница showcase `/scrolltop` с демонстрациями `basic` и `element` (`target = Parent`); e2e с настоящей прокруткой; бенчмарки и бюджеты; `docs/scrolltop.md`.
- Стили: `scrolltop` добавляется в `theme.components`.
- **Не входит:** `buttonProps` целиком (доступны только `severity`, `raised`, `rounded`); `iconTemplate` (вместо него `icon: ViewFn`); `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (устарели или привязаны к `@primeuix/motion`).

## Capabilities

### New Capabilities
- `scrolltop`: кнопка возврата к началу прокручиваемой области, видимая после порога прокрутки.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-scrolltop` (`publish = false`); зависимости: `prime-core`, `prime-button`, `prime-icons` (`chevronup`), `web-sys` (`Window`, `Element`, `HtmlElement`, `ScrollToOptions`, `ScrollBehavior`).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
