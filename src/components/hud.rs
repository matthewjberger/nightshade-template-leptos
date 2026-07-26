use leptos::prelude::*;
use nightshade_leptos::Engine;
use protocol::Command;
use wasm_bindgen::JsCast;

use crate::state::TemplateState;

/// Example HUD panel: renderer facts streamed from the worker and buttons
/// that send game messages back. Replace with your own UI as the game grows.
#[component]
pub fn Hud(engine: Engine, state: TemplateState) -> impl IntoView {
    let on_spawn = move |event: web_sys::MouseEvent| {
        if let Some(button) = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = button.blur();
        }
        engine.send(&Command::SpawnCube);
    };

    let on_grow = move |_| engine.send(&Command::GrowSelected);

    view! {
        <div class="hud">
            <div class="hud-title">"Nightshade Template"</div>
            <div class="hud-row">
                <span class="hud-label">"Adapter"</span>
                <span>{move || engine.state.adapter.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"FPS"</span>
                <span>{move || format!("{:.0}", engine.state.fps.get())}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Entities"</span>
                <span>{move || engine.state.entity_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Cubes"</span>
                <span>{move || state.cube_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Selected"</span>
                <span>
                    {move || {
                        engine
                            .state
                            .selected
                            .get()
                            .map(|detail| format!("{} ({})", detail.name, detail.id))
                            .unwrap_or_else(|| "None".to_string())
                    }}
                </span>
            </div>
            <button class="hud-button" on:click=on_spawn>
                "Spawn Cube (Space)"
            </button>
            <button
                class="hud-button"
                disabled=move || engine.state.selected.get().is_none()
                on:click=on_grow
            >
                "Grow Selected"
            </button>
            <div class="hud-hint">"Drag to orbit, right-drag to pan, wheel to zoom, click to select"</div>
        </div>
    }
}
