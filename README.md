# nightshade-template-leptos

A template for building [Nightshade](https://github.com/matthewjberger/nightshade) apps with the Leptos/webview architecture. The whole engine runs inside a web worker against an OffscreenCanvas and renders through WebGPU off the main thread. A [Leptos](https://leptos.dev) UI drives it from the main thread, and a native webview shell turns the same bundle into a desktop app. The worker seam (canvas transfer, input forwarding, picking, resize, stats) is `nightshade-api`: the `leptos` feature on the page side, the `offscreen` feature on the worker side.

## Workspace

- `protocol`, the game message enums both sides share, carried as `Custom` payloads on the engine wire.
- `worker`, the wasm module inside the web worker. The game logic against the raw `nightshade` engine plus a `TemplateWorld` (its own `freecs` world), driven by system functions in `worker/src/systems/`.
- the root crate (`page`), the Leptos UI. An `EngineViewport` from `nightshade_api::web`, an example HUD, and the game-specific page state as grouped signals.
- `desktop`, the native shell: a webview window over the web bundle, served from an ephemeral localhost port.

## Quickstart

Tooling is pinned in [mise.toml](mise.toml). Install [mise](https://mise.jdx.dev) and [just](https://github.com/casey/just), then:

```bash
just init
just run
```

`just run` builds the worker, builds the bundle with Trunk, and opens the app in a native webview window. `just run-web` serves the same bundle at http://127.0.0.1:8080 instead. The browser path needs WebGPU and OffscreenCanvas-in-workers support (Chromium 113+, Firefox 141+). The worker compiles the whole engine, so the first build is large.

## How it fits together

The page and the worker share nothing but messages. The transport (forwarded pointer, touch, wheel, and keyboard input, resize, picking, stats) is built into `nightshade-api`; the game messages are yours, defined once in `protocol/src/lib.rs` as the `Command` (page to worker) and `Event` (worker to page) enums.

The page creates an engine handle with `use_engine`, renders an `EngineViewport`, sends `Command`s with `engine.send`, and receives `Event`s through `engine.on_custom`. Renderer facts (ready, adapter, FPS, entities, selection) arrive on the handle's reactive `EngineState`.

The worker hands `nightshade_api::offscreen::run_offscreen` the driver config and the `TemplateWorld` (its own `freecs` world for game components and resources, declared in `worker/src/ecs.rs`), then a setup function, a per-frame tick, and a `Command` handler, all free functions in `worker/src/systems/`. The example system spins every spawned cube and spawns another on Space or when the HUD button sends `SpawnCube`.

Picking is built in: a click or tap without drag picks synchronously, drives the engine's selection outline, reports the selection to the page, and hands it to the `Command` handler. The `Grow Selected` button shows the round trip end to end.

To add a feature, work the seam end to end:

1. Add a variant to `Command` or `Event` in `protocol/src/lib.rs`.
2. Handle it in `apply_custom` (`worker/src/systems/example.rs`) or post it with `nightshade_api::offscreen::post_custom`.
3. Send it with `engine.send` or handle it in `engine.on_custom`, and build the UI in a new file under `src/components/`.

For binary payloads (dropped files, save data), large lists, gizmos, and the MCP agent surface, lift the patterns from the viewer and the editor. They are the same architecture with those features built out.

## License

Dual-licensed under MIT or Apache-2.0, at your option.
