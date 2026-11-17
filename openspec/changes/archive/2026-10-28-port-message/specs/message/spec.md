## ADDED Requirements

### Requirement: Message markup and classes
The `Message` component of the `prime-message` crate SHALL render a root `<div role="alert" aria-live="polite">` with classes `p-message p-component p-message-<severity>` (default `info`), `p-message-<variant>` (with `variant`), `p-message-sm` and `p-message-lg` (by `size`); inside it SHALL come `div.p-message-content-wrapper` > `div.p-message-content`. The root and all parts SHALL have `data-p` (space-separated `outlined`, `simple`, severity, size).

#### Scenario: Default severity
- **WHEN** `severity` is not set
- **THEN** the root has the class `p-message-info` and `data-p` equals `info`

#### Scenario: Variant and size
- **WHEN** `severity = success`, `variant = outlined`, `size = small` are set
- **THEN** the classes `p-message-success`, `p-message-outlined`, `p-message-sm` are present and `data-p` equals `outlined success small`

### Requirement: Message content
With a non-empty `text`, Message SHALL render a `span.p-message-text` with the text, then always a second `span.p-message-text` with the child content. `icon` (CSS class) SHALL produce `<i class="p-message-icon …">`, `icon_template` a custom element without a wrapper; `container_template` SHALL replace the text and content and receive the close callback.

#### Scenario: Text and children
- **WHEN** `text = "Info"` and a child element are set
- **THEN** inside `p-message-content` there are two `p-message-text`: the first with `Info`, the second with the child element

#### Scenario: Container
- **WHEN** `container_template` is set
- **THEN** it is rendered instead of the text and content and closes the message through the passed callback

### Requirement: Message closing and lifetime
With `closable`, Message SHALL render a `button.p-message-close-button` (`type="button"`, `aria-label` from the locale) with a `p-message-close-icon` icon (or `close_icon_template`/`close_icon`); a click SHALL hide the message (the `p-message-leave-active` class, `display: none`) and call `on_close` with the original event. With `life` (ms) the message SHALL be hidden by a timer in the browser.

#### Scenario: Closing
- **WHEN** the user presses the close button
- **THEN** the message is hidden and `on_close` was called

#### Scenario: Lifetime
- **WHEN** `life = 3000` is set
- **THEN** after 3000 ms the message is hidden

### Requirement: Pass-through Message
`pt` SHALL accept `root`, `content_wrapper`, `content`, `icon`, `text`, `close_button`, `close_icon`, adding classes and attributes without replacing the component's own.

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.content`
- **THEN** `p-message-content` has both `custom` and its own class

### Requirement: Message render cost
Message with static properties, without `closable` and `life`, SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance in SSR SHALL fit the `message` budget in `benches/alloc-budgets.toml`, and the contribution to wasm the `message` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** one set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Message values do not exceed the budgets
