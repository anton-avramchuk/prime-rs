# Drawer

Port of `p-drawer` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-drawer::Drawer`, `DrawerPosition`, `DrawerPt` | `drawer/drawer.ts`, `drawer/style/*` |
| CSS | `@primeuix/styles/drawer` + the `/** For PrimeNG **/` block from `drawerstyle.ts` (positions, `position: fixed`, animation classes) - `cargo xtask gen-theme`; `drawer` is part of `primeng_extra` |
| Demos | `apps/showcase/doc/drawer/{basic,position,size,fullscreen,template,headless}-doc.ts` |

Spec: `openspec/specs/drawer`.

## Usage

```rust
let visible = RwSignal::new(false);
view! {
    <Button label="Open" on_click=Callback::new(move |_| visible.set(true)) />
    <Drawer visible=visible header="Drawer" position=DrawerPosition::Right>
        <p>"Content"</p>
    </Drawer>
}
```

`visible` is a two-way signal: the user closes the panel with the mask, the button or `Escape`, and `false` is written to the signal. Instead of `children`, there are `header_content` and `footer` (`ViewFn`) for the header and footer; `headless` replaces the entire panel content.

## What matches PrimeNG

- Markup and classes: `div.p-drawer.p-component[role=complementary]` (`p-drawer-<position>`, `p-drawer-open`, `p-drawer-full`), `p-drawer-header` > `p-drawer-title` + `button.p-drawer-close-button`, `p-drawer-content`, `p-drawer-footer`; mask `div.p-drawer-mask.p-overlay-mask` in `body`, `data-p`, `data-p-open`.
- Animations are the classes `p-drawer-enter-<position>` / `p-drawer-leave-<position>` and `p-overlay-mask-enter-active` / `-leave-active`; the panel is removed from the DOM when the leave animation ends (`animationend`).
- Closing: a click on the mask with `dismissible`, the close button, `Escape` for the topmost panel by `z-index` with `close_on_escape`.
- `z-index` of the modal group (`auto_z_index`, `base_z_index`), the mask is one below the panel; `block_scroll` puts the `p-overflow-hidden` class and `--p-scrollbar-width` on `body`; `FocusTrap` while shown.

## Differences

| Difference | Reason |
|---|---|
| The panel and mask are always in `body` (via `Portal`), no `appendTo` | It is simpler to work around ancestors' `transform`; `Portal` already exists in `prime-dom`. |
| No `@primeuix/motion`, `motionOptions` and `transitionOptions` | Animations are PrimeNG CSS classes; a fallback timer (700 ms) removes the panel if `animationend` never arrives (no styles, animation disabled). |
| `on_hide` is called on any close (including writing `false` to `visible`) | In PrimeNG `onHide` is called only on user actions. |
| No `keydown` handler on the root with `hide(false)` | In PrimeNG it closes nothing (the document listener does). |
| The close button is a `Button` (secondary, text, rounded), no `closeButtonProps`; no deprecated `showCloseIcon` | Default properties match, customization is via `pt.close_button` and `close_icon`. |
| Content is `children`, no `content` template; `headless` is a `ViewFn` without `closeCallback` | Closing from the content is writing `false` to `visible`. |
| `size` is set via `style`/`style_class`, Tailwind responsive utilities are not used | The demo uses `width: min(100%, 30rem)`. |
| The `headless` demo is simplified (no profile menu, `Ripple`, `StyleClass`, logo); in `template` the footer buttons have no `user` and `sign-out` icons, and the avatar is initials | These icons are absent from `prime-icons`, and `Ripple` and `StyleClass` are not ported. |
| No `Bind`/`pt` with context | `pt` is plain additions of classes, style and attributes. |

## Cost

39 allocations per instance in SSR (hidden panel); contribution to wasm is about 584 KB (budget 655 KB in `benches/budgets.toml`), of which about 358 KB is `Button`.

## Not verified

Touch, `rtl`, screen readers and iOS behaviour. Verified in Chrome (browser tests and e2e): showing in `body`, closing by button, mask and Escape, leave animation (the panel is removed exactly on `animationend`, about 500 ms), positions and `full_screen`, scroll blocking, focus trap (Tab/Shift+Tab, focus on open stays on the opener button, as in PrimeNG), two panels one above the other and showing again during the leave. Screenshot comparison with PrimeNG (`pages.json`) was not run. A panel opened before hydration is not supported (the server does not emit it).
