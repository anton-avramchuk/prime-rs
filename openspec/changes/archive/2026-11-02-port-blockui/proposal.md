# Proposal

## Why

BlockUI is the second component of wave 4: a semi-transparent mask that blocks either the whole document or a chosen element (a panel, a card), with arbitrary content in the centre (for example, a loading indicator). It is needed as the basis for the "loading" states and uses the already finished shared parts: the layers `ZGroup::Modal`, the page scroll-lock counter and `Portal`.

## What Changes

- A new crate `prime-blockui` (a port of `p-blockui` from PrimeNG 21.1.10).
- `BlockUI`: while `blocked = false`, nothing is in the DOM; with `blocked = true` a mask `div.p-blockui.p-blockui-mask.p-overlay-mask.p-overlay-mask-enter-active` (in document mode also `p-blockui-mask-document`) with `display: flex`, `aria-busy="true"` and the content `children` is rendered into the target element (`target`) or into `body` (the document); on release the exit animation plays (`p-overlay-mask-leave-active`, `aria-busy="false"`), then the mask is removed.
- Properties: `blocked` (a value or a signal), `target` (a `BlockTarget` from the `NodeRef` of any element; without it the document is blocked), `auto_z_index` (yes), `base_z_index` (0), `style`, `style_class`, `pt`, `children`.
- Behaviour: in target mode the target element temporarily gets `position: relative` (the previous value is restored after the mask exits); in document mode the page scrolling is blocked (`prime_dom::block_body_scroll`, a counter shared with Dialog/Drawer); with `auto_z_index` the mask gets a layer of the `Modal` group shifted by `base_z_index`; re-enabling during the exit returns the mask to the "blocked" state; on unmount the layer and the scrolling are released.
- The showcase page `/blockui` with the demos `basic` (a mask over a panel) and `document` (a mask over the document for three seconds); e2e; benchmarks and budgets; `docs/blockui.md`.
- Styles: `blockui` is added to `theme.components`.
- **Not included:** the `BlockableUI` check (in Rust the target is any `NodeRef`), the selection via the `content` template (the content is `children`), `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `blockui`: a mask that blocks the document or a chosen element.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-blockui` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places).
