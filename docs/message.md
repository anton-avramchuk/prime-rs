# Message

Порт `p-message` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-message::Message` | `packages/primeng/src/message/message.ts`, `message/style/messagestyle.ts` |
| CSS | `@primeuix/styles/message` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/message/*-doc.ts` (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `life`) |

Спека: `openspec/specs/message`.

## Что совпадает с PrimeNG

- Корень с `role="alert"` и `aria-live="polite"`, классы `p-message p-component p-message-<severity>`, `p-message-<variant>`, `p-message-sm`/`-lg`.
- Структура `p-message-content-wrapper` > `p-message-content` с иконкой, текстом и кнопкой закрытия; `data-p` (`outlined`, `simple`, severity, size) на корне и всех частях.
- При заданном `text` выводится `span.p-message-text` с текстом, а следом всегда второй `span.p-message-text` с дочерним содержимым.
- Кнопка закрытия `p-message-close-button` с `aria-label` из локали (`aria.close`) и иконкой `p-message-close-icon`; `on_close` с исходным событием; `life` в миллисекундах.
- Слоты `icon_template`, `close_icon_template`, `container_template` (получает колбэк закрытия).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-message>` | Хост в Angular — деталь фреймворка. |
| Закрытое сообщение остаётся в DOM с `p-message-leave-active` и `display: none`, без анимации | Анимации `@primeuix/motion` не портированы; PrimeNG убирает элемент после анимации. |
| Нет `escape=false`, `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` | HTML в тексте и анимации вне объёма change; для HTML используйте дочерние элементы. |
| Нет ripple на кнопке закрытия | Кнопка не использует директиву `pRipple`. |
| `icon`/`close_icon` — CSS-классы, как в PrimeNG, но шрифт PrimeIcons не поставляется | Используйте `icon_template` с SVG-иконками `prime-icons`. |
| Статическая ветвь только для неинтерактивных сообщений | При `closable` или `life` нужен сигнал видимости, поэтому выбирается реактивная ветвь. |
| `pt.close_icon` для встроенной иконки задаёт только класс | Иконка — отдельный компонент `prime-icons`. |

## Производительность

SSR: 36 аллокаций на экземпляр с текстом (бюджет 44); вклад в сырой wasm +218 КБ (бюджет 270 КБ).
Шаблон — как у ToggleSwitch (`docs/toggleswitch.md`).
