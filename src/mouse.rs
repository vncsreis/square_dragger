use bevy::{prelude::*, time::Stopwatch};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::{
    square::{Square, SquareCoordinates},
    Counter, // Counter, MainCamera,
};

#[derive(Component)]
pub struct ClickDuration {
    pub time: Stopwatch,
}

#[derive(Resource)]
pub struct Holding(pub bool);

pub fn handle_clicking(
    mut counter: ResMut<Counter>,
    // windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    // q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
    mut holding: ResMut<Holding>,
    q_mouse: Query<&MousePosWorld>,
    mut q_square: Query<(
        &mut ClickDuration,
        &mut Transform,
        &SquareCoordinates,
        &Square,
    )>,
) {
    let (mut stopwatch, _, square_coordinates, square) = q_square.single_mut();
    let mouse = *q_mouse.single();

    if buttons.just_pressed(MouseButton::Left) {
        info!("clicked 1");
        info!(
            "{} / {}, {} / {}",
            mouse.x, mouse.y, square_coordinates.0.x, square_coordinates.0.y
        );
        info!("{}", square.size);
    }

    if validate_location(mouse.x, square_coordinates.0.x, square.size.x)
        && validate_location(mouse.y, square_coordinates.0.y, square.size.y)
    {
        if buttons.just_released(MouseButton::Left) && holding.0 {
            holding.0 = false;
            return;
        }

        if buttons.just_pressed(MouseButton::Left) {
            stopwatch.time.reset();
            info!("clicked")
        }

        if buttons.pressed(MouseButton::Left) {
            stopwatch.time.tick(time.delta());

            info!("{}", stopwatch.time.elapsed_secs());

            if stopwatch.time.elapsed_secs() > 0.5 && !holding.0 {
                holding.0 = true;
            }
        }
        if buttons.just_released(MouseButton::Left)
            && stopwatch.time.elapsed_secs() < 0.5
            && !holding.0
        {
            info!("x: {}, y: {}", mouse.x, mouse.y);
            counter.count += 1;
        }
    }
}

fn validate_location(pos: f32, coord: f32, size: f32) -> bool {
    let halfs = size / 2.;

    pos <= coord + halfs && pos >= coord - halfs
}

// pub fn cursor_system(
//     windows: Res<Windows>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) -> Option<Vec2> {
//     let (camera, camera_transform) = q_camera.single();

//     let wnd = windows.get_primary().unwrap();

//     if let Some(screen_pos) = wnd.cursor_position() {
//         let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
//         let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
//         let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
//         let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
//         let world_pos: Vec2 = world_pos.truncate();

//         return Some(world_pos);
//     }

//     None
// }
