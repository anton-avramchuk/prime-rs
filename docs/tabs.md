# Tabs

Port of PrimeNG 21.1.10 `p-tabs`, `p-tablist`, `p-tab`, `p-tabpanels`, `p-tabpanel` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-tabs::Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt` | `tabs/{tabs,tablist,tab,tabpanels,tabpanel}.ts`, `tabs/style/*` |
| CSS | `@primeuix/styles/tabs`: `cargo xtask gen-theme` (one module for all five parts; there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/tabs/{basic,controlled,disabled,scrollable,dynamic,lazy,customtemplate}-doc.ts` |

Spec: `openspec/specs/tabs`.

## What matches PrimeNG

- Markup and classes: `div.p-tabs.p-component` (+ `p-tabs-scrollable`) > `div.p-tablist` > buttons `p-tablist-prev-button` / `p-tablist-next-button` and `div.p-tablist-content.p-tablist-viewport` > `div.p-tablist-tab-list[role=tablist]` with tabs `div.p-tab[role=tab]` (`p-tab-active`, `p-disabled`, `aria-selected`, `aria-controls`, `data-p-active`, `data-p-disabled`) and the bar `span.p-tablist-active-bar`; panels `div.p-tabpanels` > `div.p-tabpanel[role=tabpanel]` (`hidden` on inactive ones).
- Identifiers `<group id>_tab_<value>` and `<group id>_tabpanel_<value>`, by default `pn_id_<N>`.
- Keys on a tab: `ArrowRight`/`ArrowLeft` wrapping around and skipping disabled ones, `Home`/`End`, `PageUp`/`PageDown` (scrolling without focus), `Enter`/`NumpadEnter`/`Space`; `tabindex` only on the active tab; `select_on_focus`.
- The indicator bar gets the `width` and `left` of the active tab; scroll buttons appear based on the scroll position and list sizes; the scroll step is computed as in PrimeNG (width of the visible area, for "forward" minus the buttons).
- `lazy` (on `Tabs` or `TabPanel`): content is rendered on first activation and stays.

## Differences

| Difference | Reason |
|---|---|
| A tab is a `<div>` without a `<p-tab>` host, etc. | Hosts in Angular are a framework detail. |
| Tab values are strings (`value="0"`), the group `value` is `RwSignal<Option<String>>` | In PrimeNG it is `string \| number`; a single type is simpler to compare. |
| Adjacent tab lookup works over the DOM children of the list and the pure function `wrap_target` | Instead of recursing over `nextElementSibling`; the result is the same. |
| The indicator is recomputed on value change and by the list's `ResizeObserver`; there is no `MutationObserver` on tab content | Tab content that changes its width without changing the list size will not update the indicator. |
| `prev_icon` / `next_icon` are `ViewFn`, no `Ripple` | Instead of `ng-template` and the `pRipple` directive. |
| The content of a `lazy` panel is ordinary `children`, no `content` template | The component itself defers rendering. |
| In `customtemplate` the `pi-minus` / `pi-plus` icons are replaced by the signs "−" / "+", Tailwind utility classes by inline styles | `prime-icons` does not have these icons. |
| The `tabmenu` demo is not ported | It needs `Router` and `Menu`. |
| No `Bind`/`pt` with context | `pt` is simple additions of classes, style and attributes. |

## Cost

281 allocations per instance in SSR (three tabs and three panels); contribution to wasm is about 374 KB (budget 420 KB in `benches/budgets.toml`).

## Not verified

Touch, `rtl` in a real browser and screen readers. Markup, click, keyboard, `lazy`, indicator and scroll buttons were verified in Chrome (e2e and browser tests).
