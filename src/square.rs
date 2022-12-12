use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::mouse::Holding;

#[derive(Component)]
pub struct Square {
    pub size: Vec3,
}

#[derive(Component)]
pub struct SquareCoordinates(pub Vec3);

pub fn handle_moving(
    holding: Res<Holding>,
    mut q_square: Query<(&mut Transform, &mut SquareCoordinates), With<Square>>,
    q_mouse: Query<&MousePosWorld>,
) {
    if !holding.0 {
        return;
    }

    let (mut transform, mut square_coordinates) = q_square.single_mut();
    let mouse = *q_mouse.single();

    transform.translation = Vec3::new(mouse.x, mouse.y, 0.0);
    square_coordinates.0 = Vec3::new(mouse.x, mouse.y, 0.0);
}
