# BlockUI

Port of `p-blockui` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-blockui::BlockUI`, `BlockTarget` | `packages/primeng/src/blockui/blockui.ts`, `blockui/style/blockuistyle.ts` |
| CSS | `@primeuix/styles/blockui` and the shared `p-overlay-mask*` rules from `base.css` — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/blockui/{basic,document}-doc.ts` |

Spec: `openspec/specs/blockui`.

## What matches PrimeNG

- Mask classes: `p-blockui p-blockui-mask`, for the document `p-blockui-mask-document`, when shown `p-overlay-mask p-overlay-mask-enter-active`, when released `p-overlay-mask-leave-active`; inline `display: flex`; `aria-busy` — `true` when shown and `false` when leaving.
- Target (`target`): the mask is the last child of the target element, which gets `position: relative` while shown; the theme stretches the mask (`position: absolute`). Without a target the mask lies in `body` (`position: fixed`), and page scrolling is blocked through `prime_dom::block_body_scroll` (a counter shared with Dialog and Drawer).
- Layer: with `auto_z_index` (on by default) the mask gets a `z-index` of the `Modal` group with the `base_z_index` offset. As in PrimeNG (`ZIndexUtils`), the layer stack is shared: a new layer is above the previous one, and the base value is taken into account only when there are no other overlays above.
- Exit: after the block is lifted an animation plays; on `animationend` (or the fallback timer of 400 ms) the mask is removed and the layer and the scroll block are released; re-enabling during the exit returns the mask to the "blocked" state.

## Differences

| Difference | Reason |
|---|---|
| While `blocked = false`, nothing is in the DOM; the mask is rendered only in the browser (`Portal`) and is not sent by the server before hydration | In PrimeNG the host stays in the markup (with `display: none` after the first release); here it is as for Drawer and Dialog. `aria-busy` also exists only on the shown mask. |
| `target` is a `BlockTarget` made from the `NodeRef` of any element instead of `BlockableUI` (`getBlockableElement()`) | Rust has no Angular interfaces; the target is an element on the page. |
| The inline `position` value of the target is restored after the mask exits | PrimeNG keeps `position: relative` forever. |
| The content is `children` (`ChildrenFn`, rebuilt on each show) instead of the `content` template | There is no `ng-template`. |
| If `target` is set but its element is not yet in the DOM, the mask is not rendered | PrimeNG throws an exception. For an initial block of a target, `blocked` must be enabled after mounting. |
| No `Bind`, `pTemplate`; `pt` is one `Pt` for the root of the mask | `pt` is simple additions of classes, style and attributes. |

## Not verified

- Focus blocking: the mask covers the pointer but does not intercept the keyboard — as in PrimeNG, Tab may move to elements under the mask.
- Several masks enabled at the same time on different targets: the layers and the scroll counter were verified on one and on sequential scenarios.

## Performance

See `benches/` (`blockui-1`, `blockui-1000`, `case-blockui`) and `benches/*budgets.toml`: on the server the mask is not rendered, only the wrapper is measured (28 allocations per instance).
