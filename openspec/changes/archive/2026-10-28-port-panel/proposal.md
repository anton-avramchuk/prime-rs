# Proposal

## Why

Panel is a container with a header, optional collapsing and a footer; the first component with an expand animation and the `aria-expanded`/`aria-controls` link. The collapsing logic (the two-way `collapsed`, the toggle, the animation, the ARIA) is needed by both Fieldset and the future Accordion, so it is extracted into shared code at the very first port, rather than copied.

## What Changes

- A new crate `prime-panel` with the `Panel` component: `header`, `toggleable`, a two-way `collapsed` (`RwSignal<bool>`), `icon_pos` (`start`/`end`/`center`), `show_header`, `toggler` (`icon`/`header`), `id`, `style`, `style_class`, the slots `header_template`, `icons_template`, `header_icons_template` (receives `collapsed`), `footer_template`, the events `on_before_toggle`, `on_after_toggle`, `pt`.
- Classes as in PrimeNG 21.1.10 (`panelstyle.ts`): `p-panel p-component`, `p-panel-toggleable`, `p-panel-expanded`/`p-panel-collapsed`, `p-panel-header`, `p-panel-title`, `p-panel-header-actions` (+ `p-panel-icons-start|end|center`), `p-panel-toggle-button`, `p-panel-content-container`, `p-panel-content-wrapper`, `p-panel-content`, `p-panel-footer`; `data-p` as in `dataP`.
- Accessibility: the toggle button with `aria-controls`, `aria-expanded`, `aria-label` (the header text), Enter/Space; the content container with `role="region"`, `aria-labelledby`, `aria-hidden`, `tabindex="-1"` when collapsed; focusable elements inside the collapsed content get `tabindex="-1"`.
- Expand animation: the classes `p-collapsible-enter-active`/`p-collapsible-leave-active` from the theme's `base.css`; `on_after_toggle` is called when the animation ends (`animationend`).
- The shared collapsing logic is extracted into the module `prime-rs-core::collapsible` (the state, the toggle, the animation, the ARIA attributes); Panel and the next Fieldset are built on it.
- The `/panel` page (`basic`, `toggleable`, `header-toggler`, `template`), e2e, benchmarks, budgets, `docs/panel.md`, the CSS of the theme `panel`.
- **Not included:** Fieldset (a separate change `port-fieldset`), `BlockableUI`, `toggleButtonProps`, `motionOptions`/`transitionOptions`, `<p-header>`/`<p-footer>` as facets (replaced by slots).

## Capabilities

### New Capabilities
- `panel`: the Panel component — structure and classes, collapsing and the two-way `collapsed`, ARIA and keyboard, animation and `on_after_toggle`, slots, `pt`, performance.
- `collapsible`: the shared collapsing logic in `prime-rs-core` — the state, the toggle, the before/after events, the ARIA attributes of the toggle and the region, the animation; reused by Panel and Fieldset.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-panel` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`minus`, `plus`), `web-sys`.
- `prime-rs-core`: a new module `collapsible`.
- Changes to the workspace `Cargo.toml`, `upstream.toml` (`panel` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, the CI browser job.
