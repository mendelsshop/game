use bevy::{log, prelude::*, window::PrimaryWindow};
use rand::random;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpInnerDirection {
    Go,
    Wait(u8),
    Reset,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpDirection {
    Up(JumpInnerDirection),
    Down(JumpInnerDirection),
    None,
}

enum CactusSize {
    Long,
    Short,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Component, Debug)]
struct Dino {
    jump: JumpDirection,
}
#[derive(Component)]
struct Cactus {
    size: CactusSize,
    direction: Direction,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
const SPRITE_SIZE: f32 = 75.0;
fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            // TODO: top left

            // transform: Transform::from_xyz(-window.width() + 100.0, window.height(), 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("dino.png"),
            ..default()
        },
        Dino {
            jump: JumpDirection::None,
        },
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            // transform: Transform::from_xyz(100.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("cactus_long.png"),
            ..Default::default()
        },
        Cactus {
            size: CactusSize::Long,
            direction: Direction::Left,
        },
    ));
    // commands.spawn((
    //     SpriteBundle {
    //         // transform: Transform::from_xyz(
    //         //     window.width() * random::<f32>(),
    //         //     window.height() / 2.0,
    //         //     0.0,
    //         // ),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
    //             ..Default::default()
    //         },
    //         texture: asset_server.load("cactus_long.png"),
    //         ..Default::default()
    //     },
    //     Cactus(CactusSize::Long),
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform::from_xyz(
    //             window.width() * random::<f32>(),
    //             window.height() / 2.0,
    //             0.0,
    //         ),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
    //             ..Default::default()
    //         },
    //         texture: asset_server.load("cactus_short.png"),
    //         ..Default::default()
    //     },
    //     Cactus(CactusSize::Short),
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform::from_xyz(
    //             window.width() * random::<f32>(),
    //             window.height() / 2.0,
    //             0.0,
    //         ),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(0.25, 0.25) * SPRITE_SIZE),
    //             ..Default::default()
    //         },
    //         texture: asset_server.load("cactus_short.png"),
    //         ..Default::default()
    //     },
    //     Cactus(CactusSize::Short),
    // ));
}

fn jump_system_recieve(input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Dino>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        if transform.jump == JumpDirection::None {
            if input.just_pressed(KeyCode::ArrowUp) {
                transform.jump = JumpDirection::Down(JumpInnerDirection::Go)

                // Jumping down!
            }
            if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowDown) {
                transform.jump = JumpDirection::Up(JumpInnerDirection::Go)
                // Jumping up!
            }
        }
    }
}

fn jump_system(mut player_query: Query<(&mut Transform, &mut Dino)>) {
    for (mut tranform, mut player) in &mut player_query {
    println!("{:?}", player.jump);

        match player.jump {
            JumpDirection::Up(JumpInnerDirection::Go) => {
                tranform.translation.y += 20.;
                player.jump = JumpDirection::Up(JumpInnerDirection::Wait(15));
            }
            JumpDirection::Down(JumpInnerDirection::Reset) => {
                tranform.translation.y += 20.;
                player.jump = JumpDirection::None;
            } 
            JumpDirection::Down(JumpInnerDirection::Go) => {
                tranform.translation.y -= 20.;
                player.jump = JumpDirection::Down(JumpInnerDirection::Wait(15));
            }
            JumpDirection::Up(JumpInnerDirection::Reset) => {
                tranform.translation.y -= 20.;
                player.jump = JumpDirection::None;
            }
            JumpDirection::Up(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Up(JumpInnerDirection::Reset);
            }
            JumpDirection::Down(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Down(JumpInnerDirection::Reset);
            }
            JumpDirection::Up(JumpInnerDirection::Wait(n)) => {
                player.jump = JumpDirection::Up(JumpInnerDirection::Wait(n-1));
            }
            JumpDirection::Down(JumpInnerDirection::Wait(n)) => {
                player.jump = JumpDirection::Down(JumpInnerDirection::Wait(n-1));
            }
            JumpDirection::None => {}
        }
    }
}

const ENEMY_SPEED: f32 = 25.0;
fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Cactus)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        println!("{}", time.delta_seconds());
        if enemy.direction == Direction::Left {
            transform.translation.x -= 125. * time.delta_seconds();
        } else {
            transform.translation.x = 201.;
        }
    }
}
fn enemy_bounds(
    mut enemy_query: Query<(&mut Transform, &mut Cactus)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = 12.5; // 32.0
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < -200.0 {
            enemy.direction = Direction::Right;
        }
        if translation.x > 200.0 {
            enemy.direction = Direction::Left;
        }
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
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, jump_system_recieve)
        .add_systems(Update, jump_system)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_bounds)
        .run();
}
