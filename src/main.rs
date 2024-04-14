use bevy::{app::AppExit, log, prelude::*, window::PrimaryWindow};
use rand::random;
pub const ENEMY_SIZE: f32 = 25.0; // This is the enemy sprite size.
pub const PLAYER_SIZE: f32 = 25.0; // This is the player sprite size.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpInnerDirection {
    Go,
    Wait(u8),
    Reset,
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyPausedState {
    #[default]
    Paused,
    Running,
}

pub struct Playing;

impl Plugin for Playing {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                jump_system_recieve,
                jump_system,
                enemy_movement,
                enemy_bounds,
                collision_detection,
            )
                .run_if(in_state(MyPausedState::Running)),
        );
    }
}

pub struct Paused;
impl Plugin for Paused {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (reset_game).run_if(in_state(MyPausedState::Paused)));
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpDirection {
    Up(JumpInnerDirection),
    Down(JumpInnerDirection),
    None,
}
fn toggle_pause_game(
    state: Res<State<MyPausedState>>,
    mut next_state: ResMut<NextState<MyPausedState>>,
) {
    match state.get() {
        MyPausedState::Paused => next_state.set(MyPausedState::Running),
        MyPausedState::Running => next_state.set(MyPausedState::Paused),
    }
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
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
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

fn reset_game(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<MyPausedState>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(MyPausedState::Running);
    }
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
                tranform.translation.y += 40.;
                player.jump = JumpDirection::Up(JumpInnerDirection::Wait(15));
            }
            JumpDirection::Down(JumpInnerDirection::Reset) => {
                tranform.translation.y += 40.;
                player.jump = JumpDirection::None;
            }
            JumpDirection::Down(JumpInnerDirection::Go) => {
                tranform.translation.y -= 40.;
                player.jump = JumpDirection::Down(JumpInnerDirection::Wait(15));
            }
            JumpDirection::Up(JumpInnerDirection::Reset) => {
                tranform.translation.y -= 40.;
                player.jump = JumpDirection::None;
            }
            JumpDirection::Up(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Up(JumpInnerDirection::Reset);
            }
            JumpDirection::Down(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Down(JumpInnerDirection::Reset);
            }
            JumpDirection::Up(JumpInnerDirection::Wait(n)) => {
                player.jump = JumpDirection::Up(JumpInnerDirection::Wait(n - 1));
            }
            JumpDirection::Down(JumpInnerDirection::Wait(n)) => {
                player.jump = JumpDirection::Down(JumpInnerDirection::Wait(n - 1));
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
            transform.translation.x -= 225. * time.delta_seconds();
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

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < -200.0 {
            enemy.direction = Direction::Right;
        }
        if translation.x > 200.0 {
            enemy.direction = Direction::Left;
        }
    }
}
fn collision_detection(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Cactus>>,
    player_query: Query<&Transform, With<Dino>>,
    state: Res<State<MyPausedState>>,
    mut next_state: ResMut<NextState<MyPausedState>>,
) {
    if let Ok(player) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player.translation.distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                toggle_pause_game(state, next_state)
            }
            break;
        }
    }
}

fn main() {
    App::new()
        .init_state::<MyPausedState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Topsy Turvey T-Rex game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, (spawn_player, spawn_enemies).chain())
        .add_plugins(Playing)
        .add_plugins(Paused)
        .run();
}
