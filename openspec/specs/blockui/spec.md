# blockui Specification

## Purpose
BlockUI blocks the document or a chosen element with a semi-transparent mask and shows arbitrary content in the center while `blocked` is enabled.

## Requirements

### Requirement: Showing and hiding the mask
`BlockUI` of the `prime-blockui` crate SHALL not render the mask while `blocked = false`. While `blocked = true` it SHALL render `div.p-blockui.p-blockui-mask.p-overlay-mask.p-overlay-mask-enter-active` with `display: flex`, `aria-busy="true"`, `style_class`, `style` and `pt` and the content `children`. When the block is lifted, the mask SHALL get `p-overlay-mask-leave-active` and `aria-busy="false"`, and when the animation ends (`animationend` or the fallback timer) it SHALL be removed from the DOM. Outside the browser (SSR) the mask SHALL not be rendered whatever `blocked` is.

#### Scenario: Enabling
- **WHEN** `blocked` becomes `true`
- **THEN** the mask with `aria-busy="true"` and the class `p-overlay-mask-enter-active` appears in the DOM

#### Scenario: Disabling
- **WHEN** `blocked` becomes `false`
- **THEN** the mask gets `p-overlay-mask-leave-active`, and then disappears from the DOM

#### Scenario: SSR
- **WHEN** a page with `blocked = true` is rendered on the server
- **THEN** there is no mask markup

### Requirement: Blocking an element
If `target` is set, the mask SHALL be rendered inside the target element (as its last child) and stretched over it by the theme class `p-blockui-mask.p-overlay-mask` (`position: absolute`); while shown, the target element SHALL get `position: relative`, and the previous value of the inline property SHALL be restored after the mask exits. The mask SHALL not have the class `p-blockui-mask-document`, and the page scroll SHALL not be blocked.

#### Scenario: Mask in the target
- **WHEN** `blocked` is enabled with `target` set
- **THEN** the mask lies inside the target element, the target has `position: relative`, and `body` has no scroll block

#### Scenario: Restoring the position
- **WHEN** the block is lifted and the mask is removed
- **THEN** the inline `position` value of the target element equals the previous one

### Requirement: Blocking the document
Without `target` the mask SHALL be rendered in `body` with the class `p-blockui-mask-document` (`position: fixed`), and it SHALL block the page scroll (`prime_dom::block_body_scroll` for the duration of the show, a counter shared with other modal components), and after the mask exits it SHALL release the block once.

#### Scenario: Document
- **WHEN** `blocked` is enabled without `target`
- **THEN** the mask lies in `body`, it has the class `p-blockui-mask-document`, and the page does not scroll

#### Scenario: Release
- **WHEN** the block is lifted and the mask is removed
- **THEN** the page scroll is allowed (if there are no other modal components)

### Requirement: Layer
With `auto_z_index` (default yes) the mask SHALL get the `z-index` of the `Modal` group (`modal + base_z_index`, through the shared layer generator) and release it after exiting; with `auto_z_index` disabled no `z-index` is set.

#### Scenario: Layer
- **WHEN** the block is enabled and `auto_z_index` is the default
- **THEN** the `z-index` of the mask is not less than the base value of the `Modal` group

### Requirement: Re-enabling and unmounting
Enabling the block during the exit animation SHALL return the mask to the "blocked" state (without re-inserting it and without a double scroll block). Unmounting the component SHALL release the layer and the scroll block and remove the mask.

#### Scenario: Quick toggling
- **WHEN** `blocked` is enabled again while the mask is playing its exit
- **THEN** the mask stays single, with `aria-busy="true"` and the enter classes

#### Scenario: Unmounting
- **WHEN** the component is unmounted while the document block is enabled
- **THEN** there is no mask, and the page scroll is allowed
