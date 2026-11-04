# Tasks

## 1. Основа

- [x] 1.1 `xtask/theme/extract.mjs`: компонент без модуля стилей; `inputmask` в `theme.components` и `primeng_extra`, `cargo xtask gen-theme`; CSS остальных компонентов не изменился, есть `p-inputmask-clear-icon`
- [x] 1.2 Свериться с `initMask`, `checkVal`, `shiftL/R`, `onKeyPress`, `onInputKeydown`, `onInputFocus/Blur`, `writeValue`, `getUnmaskedValue` исходника; значок `prime-icons`: times

## 2. Крейт `prime-inputmask`

- [x] 2.1 Движок маски `mask.rs` с табличными тестами
- [x] 2.2 Компонент `InputMask`, `InputMaskPt`: оболочка, поле, значок очистки, SSR-тесты, rustdoc
- [x] 2.3 Фокус и каретка, ввод символов, удаление, `Enter`/`Escape`, вставка, `input`, потеря фокуса, `on_complete`, `unmask`, внешнее значение, очистка
- [x] 2.4 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/inputmask`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `inputmask.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test inputmask`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-inputmask`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/inputmask.md`; `prime-inputmask` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты, deny, machete, `openspec validate port-inputmask`; пуш и один прогон CI всей волны 5 — после архива

## Workflow follow-up

- Архивировать; волна 5 завершена: пуш и `gh run watch`, при падении — чинить.
