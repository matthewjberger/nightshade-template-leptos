//! The wasm module inside the web worker. The engine `World`, the render
//! loop, and the page conversation (input injection, resize, picking, stats)
//! are all owned by `nightshade_api::offscreen::run_offscreen`; this crate is
//! only the game, written against the raw `nightshade` engine with its own
//! user-side ECS world (`TemplateWorld`).

mod ecs;
mod systems;

use wasm_bindgen::prelude::*;

use crate::ecs::TemplateWorld;

#[wasm_bindgen(start)]
pub fn start() {
    nightshade_api::offscreen::run_offscreen(
        TemplateWorld::default(),
        systems::setup::initialize,
        systems::example::tick,
        systems::example::apply_custom,
    );
}
