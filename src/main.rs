mod mouse;
mod square;

use bevy_mouse_tracking_plugin::prelude::{InsertExt, MousePosPlugin};
use mouse::{handle_clicking, ClickDuration, Holding};

use bevy::{
    prelude::*,
    time::{FixedTimestep, Stopwatch},
};
use square::{handle_moving, Square, SquareCoordinates};

const SQUARE_X: f32 = 0.0;
const SQUARE_Y: f32 = 0.0;
const SQUARE_SIZE: Vec3 = Vec3::new(200.0, 200.0, 0.0);
const TIMESTEP: f64 = 0.5 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
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

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct Counter {
    count: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Camera2dBundle::default(), MainCamera))
        .add_world_tracking();

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(SQUARE_X, SQUARE_Y, 0.0),
                scale: SQUARE_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        Square { size: SQUARE_SIZE },
        SquareCoordinates(Vec3::new(SQUARE_X, SQUARE_Y, 0.0)),
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

fn update_counter_text(counter: Res<Counter>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = counter.count.to_string();
}
