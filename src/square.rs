use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::mouse::{Click, Holding};

#[derive(Component)]
pub struct Square {
    pub size: Vec3,
}

#[derive(Component)]
pub struct SquareCoordinates(pub Vec3);

pub fn handle_moving(
    holding: Res<Holding>,
    mut q_square: Query<(&mut Transform, &mut SquareCoordinates, &mut Square), With<Square>>,
    q_mouse_pos: Query<&MousePosWorld>,
    q_click: Query<&Click>,
) {
    if !holding.0 {
        return;
    }

    let (mut transform, mut square_coordinates, _square) = q_square.single_mut();
    let mouse = q_mouse_pos.single();
    let click = q_click.single();

    // info!("mouse offset: {}/{}", click.offset.x, click.offset.y);
    // info!("mouse position: {}/{}", mouse.x, mouse.y);
    // info!(
    //     "square position: {}/{}",
    //     square_coordinates.0.x, square_coordinates.0.x
    // );

    let new_x_coord = match mouse.x > square_coordinates.0.x {
        true => mouse.x - click.offset.x,
        false => mouse.x + click.offset.x,
    };

    let new_y_coord = match mouse.y > square_coordinates.0.y {
        true => mouse.y - click.offset.y,
        false => mouse.y + click.offset.y,
    };

    transform.translation = Vec3::new(mouse.x + click.offset.x, mouse.y + click.offset.y, 0.0);
    square_coordinates.0 = Vec3::new(mouse.x + click.offset.x, mouse.y + click.offset.y, 0.0);
}
