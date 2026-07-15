use crate::ecs::TemplateResources;
use nightshade::prelude::*;
use protocol::{Command, Event};
use serde_json::Value;

const SPIN_RADIANS_PER_SECOND: f32 = 0.8;
const RING_RADIUS: f32 = 3.0;
const GOLDEN_ANGLE_RADIANS: f32 = 2.399_963;

/// Example system. Each system is a free function that takes
/// `&mut TemplateResources` for app-specific state and `&mut World` for the
/// engine's renderer-visible world. Add more files in `src/systems/` and
/// call them from the `run_offscreen` tick to grow your game.
///
/// This one spins every spawned cube and spawns another on Space.
pub fn tick(template_resources: &mut TemplateResources, world: &mut World) {
    let delta_time = world
        .expect_resource::<nightshade::ecs::window::resources::Window>()
        .timing
        .delta_time;
    let spin = nalgebra_glm::quat_angle_axis(SPIN_RADIANS_PER_SECOND * delta_time, &Vec3::y());
    for index in 0..template_resources.example.cubes.len() {
        let cube = template_resources.example.cubes[index];
        if let Some(transform) =
            world.get_mut::<nightshade::ecs::transform::components::LocalTransform>(cube)
        {
            transform.rotation = spin * transform.rotation;
        }
    }

    let events = std::mem::take(
        &mut world
            .expect_resource_mut::<nightshade::ecs::input::resources::Input>()
            .events,
    );
    for event in events {
        if let AppEvent::Keyboard { key, state } = event
            && matches!((key, state), (KeyCode::Space, KeyState::Pressed))
        {
            spawn_cube(template_resources, world);
        }
    }
}

/// Handles the game messages the page sends over the `Custom` channel.
/// `selected` is the entity picked by the driver's built-in click handling.
pub fn apply_custom(
    template_resources: &mut TemplateResources,
    world: &mut World,
    selected: Option<Entity>,
    value: Value,
) {
    let Ok(command) = serde_json::from_value::<Command>(value) else {
        return;
    };
    match command {
        Command::SpawnCube => spawn_cube(template_resources, world),
        Command::GrowSelected => {
            if let Some(entity) = selected
                && let Some(transform) =
                    world.get_mut::<nightshade::ecs::transform::components::LocalTransform>(entity)
            {
                transform.scale *= 1.2;
            }
        }
    }
}

/// Spawns a cube on a ring around the origin, names it, and reports the new
/// count to the page.
pub fn spawn_cube(template_resources: &mut TemplateResources, world: &mut World) {
    let count = template_resources.example.cubes.len() as u32;
    let position = if count == 0 {
        Vec3::new(0.0, 0.5, 0.0)
    } else {
        let angle = count as f32 * GOLDEN_ANGLE_RADIANS;
        Vec3::new(angle.cos() * RING_RADIUS, 0.5, angle.sin() * RING_RADIUS)
    };
    let cube = spawn_cube_at(world, position);
    world.set(cube, Name(format!("Cube {count}")));
    template_resources.example.cubes.push(cube);
    nightshade_api::offscreen::post_custom(&Event::CubeCount { count: count + 1 });
}
