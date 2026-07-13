//! The wasm module inside the web worker. The engine `World`, the render
//! loop, and the page conversation (input injection, resize, picking, stats)
//! are all owned by `nightshade_api::offscreen::run_offscreen`; this crate is
//! only the game, written against the raw `nightshade` engine with its own
//! app resources (`TemplateResources`).

mod ecs;
mod systems;

use nightshade_api::offscreen::OffscreenConfig;
use wasm_bindgen::prelude::*;

use crate::ecs::TemplateResources;

#[wasm_bindgen(start)]
pub fn start() {
    nightshade_api::offscreen::run_offscreen(
        OffscreenConfig::default(),
        TemplateResources::default(),
        systems::setup::initialize,
        systems::example::tick,
        systems::example::apply_custom,
    );
}
