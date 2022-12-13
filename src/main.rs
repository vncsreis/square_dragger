use bevy::{prelude::*, time::FixedTimestep};
use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use square_dragger::mouse::{handle_clicking, Holding};
use square_dragger::square::handle_moving;
use square_dragger::{setup, update_counter_text, Counter};

const TIMESTEP: f64 = 1. / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 600.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(MousePosPlugin)
        .insert_resource(Counter { count: 0 })
        .insert_resource(Holding(false))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system(handle_clicking)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(handle_moving),
        )
        .add_system(update_counter_text)
        .add_system(bevy::window::close_on_esc)
        .run();
}
