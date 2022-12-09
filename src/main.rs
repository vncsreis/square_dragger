use bevy::{
    prelude::*,
    time::{FixedTimestep, Stopwatch},
};

const BUTTON_X: f32 = 0.0;
const BUTTON_Y: f32 = 0.0;
const BUTTON_SIZE: Vec3 = Vec3::new(200.0, 200.0, 0.0);
const TIMESTEP: f64 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Counter { count: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(handle_clicking),
        )
        .add_system(update_counter_text)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Button;

#[derive(Component)]
struct ButtonCoordinates(Vec3);

#[derive(Component)]
struct ClickDuration {
    time: Stopwatch,
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct Counter {
    count: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(BUTTON_X, BUTTON_Y, 0.0),
                scale: BUTTON_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        Button,
        ButtonCoordinates(Vec3::new(BUTTON_X, BUTTON_Y, 0.0)),
        ClickDuration {
            time: Stopwatch::new(),
        },
    ));

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Clicks\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 32.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }),
    );
}

fn handle_clicking(
    mut counter: ResMut<Counter>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
    mut q_button: Query<(&mut ClickDuration, &mut Transform, &mut ButtonCoordinates), With<Button>>,
) {
    let (mut stopwatch, mut transform, mut button_coordinates) = q_button.single_mut();

    if let Some(Vec2 { x, y }) = cursor_system(windows, q_camera) {
        if validate_location(x, button_coordinates.0.x, BUTTON_SIZE.x)
            && validate_location(y, button_coordinates.0.y, BUTTON_SIZE.y)
        {
            if buttons.just_pressed(MouseButton::Left) {
                stopwatch.time.reset();
            }

            if buttons.pressed(MouseButton::Left) {
                stopwatch.time.tick(time.delta());

                info!("{}", stopwatch.time.elapsed_secs());

                if stopwatch.time.elapsed_secs() > 0.5 {
                    info!("holding");
                    transform.translation = Vec3::new(x, y, 0.0);
                    button_coordinates.0 = Vec3::new(x, y, 0.0);
                }
            }
            if buttons.just_released(MouseButton::Left) && stopwatch.time.elapsed_secs() < 0.5 {
                info!("x: {}, y: {}", x, y);
                counter.count += 1;
            }
        }
    }
}

fn validate_location(pos: f32, coord: f32, size: f32) -> bool {
    let halfs = size / 2.;

    pos <= coord + halfs && pos >= coord - halfs
}

fn update_counter_text(counter: Res<Counter>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = counter.count.to_string();
}

fn cursor_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();

    let wnd = windows.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        return Some(world_pos);
    }

    None
}
