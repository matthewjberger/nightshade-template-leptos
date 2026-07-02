use leptos::prelude::*;

/// Game-specific page state, grouped as signals. `Copy`, so it threads into
/// every component and closure without cloning. Renderer facts (ready,
/// adapter, fps, entity count, selection) live on the engine handle's
/// `EngineState`; add your own signals here.
#[derive(Clone, Copy)]
pub struct TemplateState {
    pub cube_count: RwSignal<u32>,
}

impl TemplateState {
    pub fn new() -> Self {
        Self {
            cube_count: RwSignal::new(0),
        }
    }
}

impl Default for TemplateState {
    fn default() -> Self {
        Self::new()
    }
}
