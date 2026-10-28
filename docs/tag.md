# Tag

Порт `p-tag` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-tag::Tag` | `packages/primeng/src/tag/tag.ts`, `tag/style/tagstyle.ts` |
| CSS | `@primeuix/styles/tag` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/tag/*-doc.ts` (`basic`, `icon`, `pill`, `severity`, `template`) |

Спека: `openspec/specs/tag`.

## Что совпадает с PrimeNG

- Классы `p-tag p-component`, `p-tag-<severity>`, `p-tag-rounded`, `p-tag-icon`, `p-tag-label` и `data-p` (`rounded`, серьёзность).
- Порядок содержимого: `children`, иконка (если задана), метка; метка выводится всегда, даже пустая.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<span>`, а не хост `<p-tag>` | Хост в Angular — деталь фреймворка; CSS привязан к классам. |
| Иконка — `ViewFn` в оболочке `span.p-tag-icon` | Шрифта PrimeIcons нет. |
| В демонстрации `template` нет флага | Внешняя картинка с CDN PrimeNG. |
| Иконки демонстраций заменены ближайшими из `prime-icons` | `pi-user` → `HomeIcon`, `pi-cog` → `RefreshIcon`. |

## Производительность

SSR: 11 аллокаций на экземпляр (бюджет 14); вклад в сырой wasm +97 КБ (бюджет 140 КБ, из них почти всё — общая реактивная среда Leptos).
Шаблон — тот же, что у Button и Badge (`docs/button.md`): `Prop`, две ветви отрисовки, паритет-тесты.
