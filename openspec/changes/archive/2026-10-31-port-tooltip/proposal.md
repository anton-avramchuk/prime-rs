# Proposal

## Why

Tooltip is the seventh component of wave 3 and the only one in it that is not a component in PrimeNG but the `pTooltip` directive on an arbitrary element (`<input pTooltip="Enter your username">`). We need a simple way to attach a hint to any element without changing the page markup; besides, the tooltip is the next consumer of positioning and layers (`ZGroup::Tooltip`) after `Popover` and `Select`.

## What Changes

- New crate `prime-tooltip` (a port of the `Tooltip` directive from PrimeNG 21.1.10). In Leptos the analog of `pTooltip` is a `use:` directive: `<input use:tooltip=TooltipOptions::text("Enter your username") />`. The directive is a function `tooltip(el: web_sys::Element, options: TooltipOptions)` that attaches listeners to the element and creates the tooltip on show; the page markup does not change (no wrappers).
- `TooltipOptions` (builder): `content` (`TooltipContent`: `Text(Prop<String>)`, `Html(String)` is the analog of `[escape]="false"`, `View(ViewFn)` is the analog of a template), `position` (`TooltipPosition`: `Right` by default, `Left`, `Top`, `Bottom`), `event` (`TooltipEvent`: `Hover` by default, `Focus`, `Both`), `show_delay`, `hide_delay`, `life` (ms), `position_top`, `position_left` (offsets, px), `auto_hide` (yes), `fit_content` (yes), `hide_on_escape` (yes), `show_on_ellipsis` (no), `disabled` (`Prop<bool>`), `position_style`, `style_class`, `z_index` (`None` means automatic by `ZGroup::Tooltip`), `id`.
- The tooltip is `div.p-tooltip.p-component.p-tooltip-<position>[role=tooltip]` in `body` with `div.p-tooltip-arrow` and `div.p-tooltip-text`. The position is computed as in PrimeNG: the primary side, and when it goes off screen the next ones by priority (for `top`: top → bottom → right → left, etc.); the arrow shifts toward the element center.
- Behavior: `Hover` shows on `mouseenter` and touch, hides on `mouseleave`, `click` and touch end; `Focus` on `focus`/`blur` of the element itself (or of the input inside a `p-inputwrapper` wrapper); `show_delay`/`hide_delay`/`life`; with `auto_hide` off the tooltip can be hovered with the mouse and it hides when the cursor leaves; `Escape`, window resize and ancestor scroll hide the tooltip; when the text changes the active tooltip is updated and recomputed; with `show_on_ellipsis` it is shown only if the element text is clipped.
- A showcase page `/tooltip` with the `position`, `event`, `delay`, `autohide`, `options`, `custom` demos; e2e with a real mouse and keyboard; benchmarks and budgets; `docs/tooltip.md`.
- Styles: `tooltip` is added to `theme.components`.
- **Out of scope:** `appendTo` (the tooltip is always in `body`; `target` and an arbitrary element are not supported), `pTooltipPT` / `pTooltipUnstyled` (no `Bind`/`pt`), the 100 ms delay inside `p-dialog` (not needed in our `Dialog`), the `fadeIn` animation as a JS utility is replaced with a CSS opacity transition, the `accessibility` demo (a text page).

## Capabilities

### New Capabilities
- `tooltip`: a tooltip on an element shown on hover or focus, with delays, side selection and automatic fallback when there is not enough room.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-tooltip` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
