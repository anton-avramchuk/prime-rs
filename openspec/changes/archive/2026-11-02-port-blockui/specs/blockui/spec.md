# blockui Specification

## Purpose
BlockUI blocks the document or a chosen element with a semi-transparent mask and shows arbitrary content in the centre while `blocked` is on.

## ADDED Requirements

### Requirement: Showing and hiding the mask
The `BlockUI` of the `prime-blockui` crate SHALL not render the mask when `blocked = false`. When `blocked = true` it SHALL render `div.p-blockui.p-blockui-mask.p-overlay-mask.p-overlay-mask-enter-active` with `display: flex`, `aria-busy="true"`, `style_class`, `style` and `pt` and the content `children`. When the blocking is released, the mask SHALL get `p-overlay-mask-leave-active` and `aria-busy="false"`, and when the animation ends (`animationend` or the fallback timer) it SHALL be removed from the DOM. Outside the browser (SSR) the mask SHALL not be rendered at any `blocked`.

#### Scenario: Enabling
- **WHEN** `blocked` becomes `true`
- **THEN** a mask with `aria-busy="true"` and the class `p-overlay-mask-enter-active` appears in the DOM

#### Scenario: Disabling
- **WHEN** `blocked` becomes `false`
- **THEN** the mask gets `p-overlay-mask-leave-active`, then disappears from the DOM

#### Scenario: SSR
- **WHEN** a page with `blocked = true` is rendered on the server
- **THEN** there is no mask markup

### Requirement: Blocking an element
If `target` is set, the mask SHALL be rendered inside the target element (as its last child) and stretched over it by the theme class `p-blockui-mask.p-overlay-mask` (`position: absolute`); while shown, the target element SHALL get `position: relative`, and the previous value of the inline property SHALL be restored after the mask exits. The mask SHALL not have the class `p-blockui-mask-document`, and the page scrolling SHALL not be blocked.

#### Scenario: Mask in the target
- **WHEN** `blocked` is enabled with a `target` set
- **THEN** the mask is inside the target element, it has `position: relative`, `body` is not scroll-blocked

#### Scenario: Restoring the position
- **WHEN** the blocking is released and the mask is removed
- **THEN** the inline value of `position` of the target element equals the previous one

### Requirement: Blocking the document
Without `target` the mask SHALL be rendered into `body` with the class `p-blockui-mask-document` (`position: fixed`) and SHALL block the page scrolling (`prime_dom::block_body_scroll` while shown, a counter shared with the other modal components); after the mask exits, the blocking SHALL be released once.

#### Scenario: Document
- **WHEN** `blocked` is enabled without `target`
- **THEN** the mask is in `body`, it has the class `p-blockui-mask-document`, the page does not scroll

#### Scenario: Release
- **WHEN** the blocking is released and the mask is removed
- **THEN** the page scrolling is allowed (if there are no other modal components)

### Requirement: Layer
With `auto_z_index` (enabled by default) the mask SHALL get the `z-index` of the `Modal` group (`modal + base_z_index`, through the shared layer generator) and release it after exiting; with `auto_z_index` disabled no `z-index` is set.

#### Scenario: Layer
- **WHEN** the blocking is enabled and `auto_z_index` is the default
- **THEN** the `z-index` of the mask is not less than the base value of the `Modal` group

### Requirement: Re-enabling and unmounting
Enabling the blocking during the exit animation SHALL return the mask to the "blocked" state (without re-inserting it and without a double page-scroll blocking). Unmounting the component SHALL release the layer and the scroll blocking and remove the mask.

#### Scenario: Quick toggling
- **WHEN** `blocked` is enabled again while the mask is playing its exit
- **THEN** the mask remains one, `aria-busy="true"`, the enter classes

#### Scenario: Unmounting
- **WHEN** the component is unmounted with the document blocking enabled
- **THEN** there is no mask, and the page scrolling is allowed
