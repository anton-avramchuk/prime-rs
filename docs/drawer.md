# Drawer

Порт `p-drawer` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-drawer::Drawer`, `DrawerPosition`, `DrawerPt` | `drawer/drawer.ts`, `drawer/style/*` |
| CSS | `@primeuix/styles/drawer` + блок `/** For PrimeNG **/` из `drawerstyle.ts` (позиции, `position: fixed`, классы анимаций) — `cargo xtask gen-theme`; `drawer` входит в `primeng_extra` |
| Демонстрации | `apps/showcase/doc/drawer/{basic,position,size,fullscreen,template,headless}-doc.ts` |

Спека: `openspec/specs/drawer`.

## Как пользоваться

```rust
let visible = RwSignal::new(false);
view! {
    <Button label="Open" on_click=Callback::new(move |_| visible.set(true)) />
    <Drawer visible=visible header="Drawer" position=DrawerPosition::Right>
        <p>"Content"</p>
    </Drawer>
}
```

`visible` — двусторонний сигнал: пользователь закрывает панель маской, кнопкой или `Escape`, и в сигнал записывается `false`. Вместо `children` для заголовка и подвала есть `header_content` и `footer` (`ViewFn`), `headless` заменяет всё содержимое панели.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-drawer.p-component[role=complementary]` (`p-drawer-<position>`, `p-drawer-open`, `p-drawer-full`), `p-drawer-header` > `p-drawer-title` + `button.p-drawer-close-button`, `p-drawer-content`, `p-drawer-footer`; маска `div.p-drawer-mask.p-overlay-mask` в `body`, `data-p`, `data-p-open`.
- Анимации — классы `p-drawer-enter-<position>` / `p-drawer-leave-<position>` и `p-overlay-mask-enter-active` / `-leave-active`; панель убирается из DOM по окончании анимации выхода (`animationend`).
- Закрытие: клик по маске при `dismissible`, кнопка закрытия, `Escape` у верхней по `z-index` панели при `close_on_escape`.
- `z-index` группы модальных окон (`auto_z_index`, `base_z_index`), маска на единицу ниже панели; `block_scroll` ставит на `body` класс `p-overflow-hidden` и `--p-scrollbar-width`; `FocusTrap` на время показа.

## Отличия

| Отличие | Причина |
|---|---|
| Панель и маска всегда в `body` (через `Portal`), `appendTo` нет | Так проще обойти `transform` у предков; `Portal` уже есть в `prime-dom`. |
| Нет `@primeuix/motion`, `motionOptions` и `transitionOptions` | Анимации — CSS-классы PrimeNG; запасной таймер (700 мс) убирает панель, если `animationend` не придёт (нет стилей, отключена анимация). |
| `on_hide` вызывается при любом закрытии (в том числе записью `false` в `visible`) | В PrimeNG `onHide` вызывается только при действиях пользователя. |
| Нет обработчика `keydown` на корне с `hide(false)` | В PrimeNG он ничего не закрывает (закрывает слушатель документа). |
| Кнопка закрытия — `Button` (вторичная, текстовая, круглая), нет `closeButtonProps`; нет устаревшего `showCloseIcon` | Свойства по умолчанию совпадают, настройка — через `pt.close_button` и `close_icon`. |
| Содержимое — `children`, нет шаблона `content`; `headless` — `ViewFn` без `closeCallback` | Закрытие из содержимого — запись `false` в `visible`. |
| `size` задаётся `style`/`style_class`, адаптивные утилиты Tailwind не используются | В демо — `width: min(100%, 30rem)`. |
| Демо `headless` упрощена (нет меню профиля, `Ripple`, `StyleClass`, логотипа); в `template` кнопки подвала без значков `user` и `sign-out`, а аватар — инициалы | Этих значков нет в `prime-icons`, `Ripple` и `StyleClass` не перенесены. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |

## Стоимость

39 аллокаций на экземпляр при SSR (скрытая панель); вклад в wasm ≈ 584 КБ (бюджет 655 КБ в `benches/budgets.toml`), из них ≈ 358 КБ — `Button`.

## Не проверено

Касание, `rtl`, экранные считыватели и поведение на iOS. Проверены в Chrome (браузерные тесты и e2e): показ в `body`, закрытие кнопкой, маской и Escape, анимация выхода (панель убирается ровно по `animationend`, ≈ 500 мс), позиции и `full_screen`, блокировка прокрутки, ловушка фокуса (Tab/Shift+Tab, фокус при открытии остаётся на кнопке-открывалке, как в PrimeNG), две панели друг над другом и повторный показ во время выхода. Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось. Открытая до гидратации панель не поддерживается (сервер её не отдаёт).
