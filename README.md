# homelab-ui

Shared look of the homelab panels: the [vps-edge](https://github.com/Darkrizen7/vps-edge)
dashboard and the [game-manager](https://github.com/Darkrizen7/game-manager) games panel. A
Rust crate that embeds the files, so each app serves its own copy and none depends on another
one's routes.

| File | |
|---|---|
| `assets/style.css` | design tokens (dark first, light follows the OS), layout, top bar and navigation, cards, buttons, fields, badges, banner, dialogs, logs drawer |
| `assets/format.js` | `el()` (DOM without innerHTML, for a strict CSP), `bytes`, `percent`, `duration`, `clock`, `status` |
| `assets/fonts/` | Inter and JetBrains Mono (SIL Open Font License 1.1, texts next to them) |
| `assets/favicon.svg` | the ◆ icon |

## Use

```toml
homelab-ui = { git = "https://github.com/Darkrizen7/homelab-ui", tag = "v0.1.0" }
```

Serve `homelab_ui::FILES` under a prefix (`/ui/` in vps-edge, `/games/ui/` in game-manager):
`get(path)` gives the bytes, the MIME type and whether the file may be cached for good
(fonts). Every URL inside the files is relative, so any prefix works. Then:

```html
<link rel="icon" href="/ui/favicon.svg" type="image/svg+xml">
<link rel="stylesheet" href="/ui/style.css">
<script type="module">import { el } from '/ui/format.js';</script>
```

Styles that only one app needs stay in that app (`app.css` in vps-edge, `games.css` in
game-manager).

## Release

Change the files, `cargo test` (every file embedded, URLs relative), commit, then tag:
`git tag v0.x.y && git push --tags`. Each app moves to the new tag in its `Cargo.toml`
(`cargo update -p homelab-ui`), checks its pages and deploys: an app keeps the version it was
tested with until then.
