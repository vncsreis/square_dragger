use bevy::prelude::*;

use crate::screen_to_world;

#[derive(Component)]
pub struct Walled;

pub fn handle_wall_collision(
    q_walled: Query<&Walled>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    let walled = q_walled.single();
    let window = windows.get_primary().unwrap();
    let camera = q_camera.single();

    let bottom_left = Vec2::new(0., 0.);
    let top_left = Vec2::new(0., window.height());
    let top_right = Vec2::new(window.width(), window.height());
    let bottom_right = Vec2::new(window.width(), 0.);

    let borders = vec![bottom_left, top_left, top_right, bottom_right];

    let borders_world_position: Vec<Vec3> = borders
        .iter()
        .map(|pos| screen_to_world(*pos, camera, window))
        .collect();
}
