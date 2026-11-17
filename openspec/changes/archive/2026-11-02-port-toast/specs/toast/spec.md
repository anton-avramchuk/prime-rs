# toast Specification

## Purpose
Toast shows short-lived notifications in a screen corner; messages come from a shared service from anywhere in the app, disappear on their own or are closed by the user.

## ADDED Requirements

### Requirement: Container markup
The `Toast` of crate `prime-toast` SHALL render `div.p-toast.p-component` with class `p-toast-<position>`, inline style `position: fixed` and offsets by position (`top`/`right`/`bottom`/`left` 20px at the corresponding edges, 50% at the center) and `data-p` with the position. Without messages the container SHALL stay empty. `style_class` and `pt` SHALL apply to the container.

#### Scenario: Empty container
- **WHEN** a `Toast` without messages is rendered
- **THEN** `.p-toast.p-toast-top-right` is rendered without `.p-toast-message`

#### Scenario: Position
- **WHEN** `position=BottomCenter`
- **THEN** the container has `p-toast-bottom-center`, `bottom: 20px` and `left: 50%`

### Requirement: Message markup
Each message SHALL be a `div.p-toast-message` with class `p-toast-message-<severity>` (`info` without an explicit severity), `role="alert"`, `aria-live="assertive"`, `aria-atomic="true"` and `data-p` with the severity, containing `div.p-toast-message-content` with an icon (`Check`, `InfoCircle`, `TimesCircle`, `ExclamationTriangle` by severity or `span.p-toast-message-icon` with the class from `icon`), `div.p-toast-message-text` (`div.p-toast-summary`, `div.p-toast-detail`) and, if `closable` is not false, `button.p-toast-close-button` with `aria-label` from the locale. The message `style_class` and `content_style_class` SHALL be added to the corresponding elements.

#### Scenario: Severity and texts
- **WHEN** a `Success` message with `summary="Done"` and `detail="Saved"` is added
- **THEN** `.p-toast-message-success` is rendered with "Done" in `.p-toast-summary` and "Saved" in `.p-toast-detail`

#### Scenario: No close button
- **WHEN** a message is added with `closable=false`
- **THEN** there is no close button

### Requirement: Message service
`MessageService` SHALL be provided via context and accept `add`, `add_all`, `clear` and `clear_key`. Each `Toast` SHALL accept only messages with its own `key` (both key-less is a match); `clear()` SHALL remove messages of all `Toast`s, `clear_key(k)` only of `Toast`s with key `k`. After a `Toast` unmounts the service SHALL NOT call it.

#### Scenario: Keys
- **WHEN** there are two `Toast`s with keys "a" and "b" and a message with key "a" is added
- **THEN** it is shown only in the first

#### Scenario: Clear by key
- **WHEN** both `Toast`s have messages and `clear_key("a")` is called
- **THEN** the messages of the first close, those of the second remain

### Requirement: Lifetime
A message SHALL close on its own after the message `life`, and if absent, after the `Toast` `life` (3000 ms by default); `sticky` SHALL disable time-based closing. Hovering over a message SHALL stop the timer, the cursor leaving SHALL restart it (unless the message is closing).

#### Scenario: Closing on time
- **WHEN** a message is added with `life=200`
- **THEN** after 200 ms the exit animation starts, then the message is removed

#### Scenario: Hover
- **WHEN** the cursor is over the message after the lifetime has elapsed
- **THEN** the message stays until the cursor leaves

### Requirement: Closing and animation
The close button and `clear` SHALL start the message exit animation (`p-toast-message-leave-active`); entering SHALL be accompanied by `p-toast-message-enter-active`. After the animation ends (`animationend` or the fallback time if there is no animation) the message SHALL be removed from the DOM and `on_close` SHALL be called with that message.

#### Scenario: Closing via the button
- **WHEN** the user presses the close button
- **THEN** the message gets `p-toast-message-leave-active`, then is removed, `on_close` is called once

### Requirement: Duplicates
With `prevent_open_duplicates` a message with the same `summary`, `detail` and severity as an already shown one SHALL NOT be added. With `prevent_duplicates` a message matching any previously added to this `Toast` SHALL NOT be added.

#### Scenario: Duplicate of a shown one
- **WHEN** `prevent_open_duplicates` is on and the same message is added twice in a row
- **THEN** one is shown

### Requirement: Layer
While there are messages, with `auto_z_index` the container SHALL have a modal group `z-index` with offset `base_z_index`; after the last message leaves the index SHALL be released.

#### Scenario: Layer
- **WHEN** a message is shown, then closed
- **THEN** the container `z-index` was set and is released after closing

### Requirement: Custom views
`message` SHALL replace the icon and texts inside the message content (the close button stays); `headless` SHALL replace everything inside `div.p-toast-message` and receive the message close function.

#### Scenario: Headless
- **WHEN** `headless` is set and a message is added
- **THEN** inside `.p-toast-message` there is only the supplied view, and its close function closes the message

### Requirement: Responsiveness
With `breakpoints` a `style` with rules `@media screen and (max-width: <width>)` for the container SHALL be added to `head`; on unmount it SHALL be removed.

#### Scenario: Rule
- **WHEN** `breakpoints` contains `920px` with style `width: 50%`
- **THEN** `head` has a rule with `max-width: 920px` and `width: 50% !important`

### Requirement: Rendering without a browser
Outside the browser (SSR) `Toast` SHALL render as an empty container and SHALL NOT touch `window` and `document`; the service SHALL work without a browser (messages are not shown on the server).

#### Scenario: SSR
- **WHEN** a page with `Toast` is rendered on the server
- **THEN** there is an empty `.p-toast` in the markup
