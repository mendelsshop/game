use bevy::prelude::*;
#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Dino;
#[derive(Component)]
struct Cactus(u32);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
struct Entity(u64);
const SPRITE_SIZE: f32 = 75.0;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
            ..Default::default()
        },
        texture: asset_server.load("dino.png"),
        ..default()
    });
}
fn jump_system(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowDown) {
        // Jumping!
    }
}
fn jump_system_up(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Dino>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        if input.just_pressed(KeyCode::ArrowUp) {
            // Jumping!
        }
        transform.rotate_x(180.0)
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Topsy Turvey T-Rex game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update,jump_system_up )
        .run();
}
