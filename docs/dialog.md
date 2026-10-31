# Dialog

Порт `p-dialog` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-dialog::Dialog`, `DialogPosition`, `DialogPt` | `dialog/dialog.ts`, `dialog/style/*` |
| CSS | `@primeuix/styles/dialog` — `cargo xtask gen-theme` (блока `For PrimeNG` у `dialogstyle.ts` нет; позиция и выравнивание — инлайн-стили, они записываются самим компонентом) |
| Демонстрации | `apps/showcase/doc/dialog/{basic,position,modal,withoutmodal,maximizable,responsive,longcontent,template,headless}-doc.ts` |

Спека: `openspec/specs/dialog`.

## Как пользоваться

```rust
let visible = RwSignal::new(false);
view! {
    <Button label="Show" on_click=Callback::new(move |_| visible.set(true)) />
    <Dialog visible=visible header="Edit Profile" modal=true style="width: 25rem">
        <p>"Content"</p>
    </Dialog>
}
```

`visible` — двусторонний сигнал: диалог закрывается кнопкой, `Escape` или (при `dismissable_mask`) нажатием на маску, и в сигнал записывается `false`. Заголовок и подвал задаются `header_content` и `footer` (`ViewFn`), `headless` заменяет всё содержимое окна. Ширина по ширине экрана — `breakpoints=vec![("575px".into(), "90vw".into())]`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-dialog-mask` (`p-overlay-mask` при `modal`, `p-dialog-<положение>`) > `div.p-dialog.p-component[role=dialog][aria-modal=true]` > `p-resizable-handle`, `p-dialog-header` (`p-dialog-title`, `p-dialog-header-actions` с `p-dialog-maximize-button` и `p-dialog-close-button`), `p-dialog-content`, `p-dialog-footer`; `data-p`. Маска есть и у немодального диалога (с `pointer-events: none`), окно — её потомок.
- Девять положений на экране (инлайн-стили маски те же, что в `dialogstyle.ts`).
- Анимации — классы `p-dialog-enter-active` / `p-dialog-leave-active` и `p-overlay-mask-*-active`; окно убирается из DOM по окончании анимации выхода.
- Перетаскивание за заголовок (кроме блока кнопок), `keep_in_viewport`, `min_x`, `min_y`; изменение размера за правый нижний угол (пока не перетаскивали, смещение удваивается, как в PrimeNG); максимизация с блокировкой прокрутки для немодального диалога без `block_scroll`; `breakpoints` через `<style>` в `head`.
- `Escape` закрывает верхний по `z-index` диалог; нажатие на саму маску закрывает при `dismissable_mask`; `z-index` группы модальных окон, маска на единицу ниже окна; `FocusTrap`; фокус при показе на первый фокусируемый элемент содержимого, затем подвала и заголовка.

## Отличия

| Отличие | Причина |
|---|---|
| Маска и окно всегда в `body` (через `Portal`), `appendTo` нет | Так проще обойти `transform` у предков; `Portal` уже есть в `prime-dom`. |
| Нет `@primeuix/motion`, `motionOptions`, `maskMotionOptions`, `transitionOptions` | Анимации — CSS-классы PrimeNG; запасной таймер (700 мс) убирает окно, если `animationend` не придёт. |
| `on_show` вызывается сразу после попадания окна в DOM, а не после анимации входа | Так же у `Drawer`; для логики приложения разницы нет. |
| `on_hide` вызывается в начале закрытия, при любом способе закрытия | В PrimeNG — после анимации выхода. |
| Перетаскивание и размер — слушатели `mousemove`/`mouseup` окна, подписанные на время жеста; частота обновлений — как у браузера | В PrimeNG слушатели висят всё время показа; результат тот же. |
| Нет `closeButtonProps`, `maximizeButtonProps`, `rtl`, `closeTabindex`, шаблона `content`; значки — `ViewFn` | Кнопки — `Button` с теми же свойствами по умолчанию (вторичная, текстовая, круглая), настройка через `pt` и значки. |
| Нет `OverlayService.emitParentDrag` | Нужен только для оверлеев, привязанных к окну. |
| Демо `overlaysinside` не перенесена | У `Select` нет `appendTo`: его панель обрежется границами окна. |
| Демо `headless` упрощена (без логотипа и градиента); в `position` у кнопок нет значков со стрелками | Нет значков в `prime-icons`; `backdrop-blur` заменён `backdrop-filter` в `mask_style`. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |

## Стоимость

66 аллокаций на экземпляр при SSR (скрытый диалог); вклад в wasm ≈ 684 КБ (бюджет 765 КБ в `benches/budgets.toml`), из них ≈ 358 КБ — `Button`.

## Не проверено

Касание (перетаскивание и размер — только мышь, как в PrimeNG), `rtl`, экранные считыватели. Проверены в Chrome (браузерные тесты и e2e с настоящей мышью и клавиатурой): показ в `body`, кнопки, закрытие кнопкой, Escape и маской, модальность и блокировка прокрутки, положения, перетаскивание и граница экрана, изменение размера, максимизация, `breakpoints`, фокус и ловушка фокуса, два диалога друг над другом и повторный показ во время выхода. Сравнение скриншотов с PrimeNG (`pages.json`) не запускалось. Открытый до гидратации диалог не поддерживается (сервер его не отдаёт).
