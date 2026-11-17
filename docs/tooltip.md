# Tooltip

A port of the `pTooltip` directive from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-tooltip::tooltip` (directive), `TooltipOptions`, `TooltipContent`, `TooltipPosition`, `TooltipEvent` | `tooltip/tooltip.ts`, `tooltip/style/*` |
| CSS | `@primeuix/styles/tooltip`: `cargo xtask gen-theme` (there is no `For PrimeNG` block) |
| Demos | `apps/showcase/doc/tooltip/{position,event,delay,autohide,options,custom}-doc.ts` |

Spec: `openspec/specs/tooltip`.

## Usage

In Leptos the analog of `pTooltip` is a `use:` directive: it works on any element and on the root of any project component and does not change the markup.

```rust
view! {
    <input use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Top) />
    <Button label="Save" use:tooltip=TooltipOptions::text("Confirm to proceed").show_delay(1000).hide_delay(300) />
    <InputText use:tooltip=TooltipOptions::text("Enter your username").event(TooltipEvent::Focus) />
}
```

`TooltipOptions::text_signal(signal)` makes the text reactive, `.disabled(signal)` disables it; `html(..)` and `view(..)` set markup and a view. The directive is invoked by Leptos in the browser when the element is built and hydrated; it is not invoked on the server.

## What matches PrimeNG

- Markup: `div.p-tooltip.p-component.p-tooltip-<side>[role=tooltip]` in `body` > `div.p-tooltip-arrow` + `div.p-tooltip-text`; `id`, `style_class`, `position_style`, `fit_content`, `z_index`.
- Events `Hover` (mouse and touch), `Focus`, `Both`; `click` hides; focus is listened for on `.p-component` inside the element, on the `input` in `p-inputwrapper`, or on the element itself.
- Position: four sides and the same fallback order when there is not enough room (`top`: top → bottom → right → left, etc.), the arrow at the edge and centered, `position_top` and `position_left` offsets.
- `show_delay`, `hide_delay`, `life`, `auto_hide` (when `false` the tooltip can be hovered with the mouse), `hide_on_escape`, `show_on_ellipsis`, `disabled`.
- Closing on Escape, window `resize` and scrolling of a nested scrollable ancestor; the tooltip group layer (`ZGroup::Tooltip`).

## Differences

| Difference | Reason |
|---|---|
| The tooltip is always in `body`; there is no `appendTo` (`target`, element) | Same as for `Popover`, `Dialog`, `Drawer`. |
| Scrolling the page itself does not hide the tooltip | The tooltip is in page coordinates and moves with the page. In PrimeNG `ConnectedOverlayScrollHandler` listens to scrollable ancestors, not the window. |
| Appearance is a 250 ms CSS opacity transition rather than the JS `fadeIn` function | The result is the same. |
| Parameters are assembled by the `TooltipOptions` builder; there is no `tooltipOptions` object and separate inputs | A typed analog of the `pTooltip`, `tooltipPosition`, `showDelay`, etc. inputs. |
| Content is `Text` / `Html` / `View`; there is no `TemplateRef` and no `escape` flag | `Html` is the analog of `[escape]="false"`, `View` of a template. |
| There is no `pTooltipPT`, `pTooltipUnstyled`, `Bind` | The PrimeNG `pt` mechanism is not ported. |
| There is no 100 ms delay inside `p-dialog` | Our `Dialog` positions the window before the tooltip is shown. |
| When the directive is applied again (`rebuild`), listeners are rebound to the new parameters, while the reactive signals of the previous content keep being tracked | A consequence of Leptos calling the directive on every attribute build. |

## Cost

The directive is not invoked in SSR, so the cost is about 1 allocation per element (building the parameters); the contribution to wasm is about 151 KB (budget 170 KB in `benches/budgets.toml`).

## Not verified

Touch (`touchstart`/`touchend`, closing by touch outside the tooltip with `auto_hide(false)`) on real input, `rtl` and screen readers (`aria-describedby` is not set, same as in PrimeNG). Verified in Chrome (browser tests and e2e with a real mouse and keyboard): show and hide on hover, focus and both, `click`, the four sides and fallback to another one when there is not enough room, offsets and view options, delays and `life`, `auto_hide(false)`, Escape, `resize` and scrolling of a nested ancestor, text/HTML/`View`, reactive text and `disabled`, `show_on_ellipsis`, the directive on the root of `Button` and `InputText`, cleanup on unmount and re-applying to the same element. Screenshot comparison with PrimeNG (`pages.json`) was not run.
