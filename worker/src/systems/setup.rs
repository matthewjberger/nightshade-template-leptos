use crate::ecs::{TemplateResources, register_template_components};
use crate::systems::example;
use nightshade::prelude::*;

/// Builds the scene: atmosphere, lighting, camera, and the first cube.
pub fn initialize(template_resources: &mut TemplateResources, world: &mut World) {
    world.ecs.add_world_at(GAME, register_template_components());

    if let Some((width, height)) = world
        .expect_resource::<nightshade::ecs::window::resources::Window>()
        .cached_viewport_size
    {
        world
            .expect_resource_mut::<nightshade::ecs::window::resources::Window>()
            .active_viewport_rect = Some(nightshade::render::config::ViewportRect {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        });
    }
    world
        .expect_resource_mut::<nightshade::render::config::RenderSettings>()
        .atmosphere = Atmosphere::Nebula;
    world
        .expect_resource_mut::<nightshade::render::config::DebugDraw>()
        .show_grid = true;
    world
        .expect_resource_mut::<nightshade::ecs::graphics::selection::Selection>()
        .outline_enabled = true;
    world
        .expect_resource_mut::<nightshade::ecs::graphics::selection::Selection>()
        .outline_color = [1.0, 0.5, 0.15, 1.0];

    spawn_sun(world);

    let camera = spawn_pan_orbit_camera(
        world,
        Vec3::new(0.0, 0.5, 0.0),
        8.0,
        0.6,
        0.4,
        "Main Camera".to_string(),
    );
    world.resources.active_camera = Some(camera);

    example::spawn_cube(template_resources, world);
}
