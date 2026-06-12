use leptos::prelude::*;
use protocol::ClientMessage;

use crate::bridge::{Bridge, send};
use crate::state::TemplateState;

/// Example HUD panel: renderer facts streamed from the worker and a button
/// that sends a game message back. Replace with your own UI as the game
/// grows.
#[component]
pub fn Hud(
    bridge: StoredValue<Option<Bridge>, LocalStorage>,
    state: TemplateState,
) -> impl IntoView {
    let on_spawn = move |_| {
        if let Some(bridge) = bridge.get_value() {
            send(&bridge, &ClientMessage::SpawnCube);
        }
    };

    view! {
        <div class="hud">
            <div class="hud-title">"Nightshade Template"</div>
            <div class="hud-row">
                <span class="hud-label">"Adapter"</span>
                <span>{move || state.adapter.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"FPS"</span>
                <span>{move || format!("{:.0}", state.fps.get())}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Entities"</span>
                <span>{move || state.entity_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Cubes"</span>
                <span>{move || state.cube_count.get()}</span>
            </div>
            <div class="hud-row">
                <span class="hud-label">"Selected"</span>
                <span>
                    {move || {
                        state
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
            <div class="hud-hint">"Drag to orbit, right-drag to pan, wheel to zoom, click to select"</div>
        </div>
    }
}
