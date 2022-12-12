use bevy::{prelude::*, time::Stopwatch};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::{
    square::{Square, SquareCoordinates},
    Counter,
};

#[derive(Component)]
pub struct Click {
    pub offset: Vec2,
    pub time: Stopwatch,
}

#[derive(Resource)]
pub struct Holding(pub bool);

pub fn handle_clicking(
    mut counter: ResMut<Counter>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut holding: ResMut<Holding>,
    q_mouse: Query<&MousePosWorld>,
    mut q_square: Query<(&mut Click, &mut Transform, &SquareCoordinates, &Square)>,
) {
    let (mut click, _, square_coordinates, square) = q_square.single_mut();
    let mouse = *q_mouse.single();

    if validate_location(mouse.x, square_coordinates.0.x, square.size.x)
        && validate_location(mouse.y, square_coordinates.0.y, square.size.y)
    {
        if buttons.just_released(MouseButton::Left) && holding.0 {
            holding.0 = false;
            return;
        }

        if buttons.just_pressed(MouseButton::Left) {
            click.time.reset();
        }

        if buttons.pressed(MouseButton::Left) {
            click.time.tick(time.delta());

            if click.time.elapsed_secs() > 0.5 && !holding.0 {
                click.offset = get_click_offset(square_coordinates.0.truncate(), mouse.truncate());
                holding.0 = true;
            }
        }
        if buttons.just_released(MouseButton::Left) && click.time.elapsed_secs() < 0.5 && !holding.0
        {
            counter.count += 1;
        }
    }
}

fn validate_location(pos: f32, coord: f32, size: f32) -> bool {
    let halfs = size / 2.;

    pos <= coord + halfs && pos >= coord - halfs
}

fn get_click_offset(square_coordinates: Vec2, mouse_coordinates: Vec2) -> Vec2 {
    // let x_offset = (square_coordinates.x - mouse_coordinates.x).abs();
    // let y_offset = (square_coordinates.y - mouse_coordinates.y).abs();

    let x_offset = match mouse_coordinates.x > square_coordinates.x {
        true => -(square_coordinates.x - mouse_coordinates.x).abs(),
        false => (square_coordinates.x - mouse_coordinates.x).abs(),
    };

    let y_offset = match mouse_coordinates.y > square_coordinates.y {
        true => -(square_coordinates.y - mouse_coordinates.y).abs(),
        false => (square_coordinates.y - mouse_coordinates.y).abs(),
    };

    Vec2::new(x_offset, y_offset)
}
