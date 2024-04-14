use std::usize;

use bevy::{prelude::*, time::Stopwatch, window::PrimaryWindow};
use rand::random;
pub const ENEMY_SIZE: f32 = 75.0; // This is the enemy sprite size.
pub const PLAYER_SIZE: f32 = 75.0; // This is the player sprite size.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpInnerDirection {
    Go(u8),
    Wait(u8),
    Reset(u8),
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyPausedState {
    #[default]
    Paused,
    Running,
}
#[derive(Component)]
struct Duration {
    time: Stopwatch,
}
#[derive(Resource)]
struct Score(f32);
pub struct Playing;

impl Plugin for Playing {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                jump_system_recieve,
                jump_system,
                enemy_movement,
                enemy_movement1,
                enemy_movement2,
                update_timer,
                enemy_bounds,
                collision_detection,
                enemy_bounds1,
                enemy_bounds2,
            )
                .run_if(in_state(MyPausedState::Running)),
        );
        app.add_plugins(Despawn);
        app.add_systems(OnEnter(MyPausedState::Running), spawn_enemies);
    }
}

pub struct Despawn;

impl Plugin for Despawn {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MyPausedState::Paused),
            (
                despawn_all_short_cactus,
                despawn_all_long_cactus,
                despawn_all_birds,
            ),
        );
    }
}

pub struct Paused;
impl Plugin for Paused {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (reset_game).run_if(in_state(MyPausedState::Paused)));
    }
}
fn despawn_all_short_cactus(mut commands: Commands, query: Query<Entity, With<LongCactus>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
fn despawn_all_long_cactus(mut commands: Commands, query: Query<Entity, With<ShortCactus>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
fn despawn_all_birds(mut commands: Commands, query: Query<Entity, With<Bird>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JumpDirection {
    Up(JumpInnerDirection),
    Down(JumpInnerDirection),
    None,
}

#[derive(Component)]
pub struct ShortCactus {
    direction: Direction,
}
#[derive(Component)]
pub struct LongCactus {
    direction: Direction,
}
#[derive(Component)]
pub struct Bird {
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Left,
    Right(usize),
}
#[derive(Component, Debug)]
struct Dino {
    jump: JumpDirection,
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn spawn_text(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                ..default()
            }),
            TextSection::new(
                " Top Score: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0.0",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 60.0,
                    ..default()
                },
            ),
        ]),
    ));
}

fn spawn_timer(mut commands: Commands) {
    commands.spawn(Duration {
        time: Stopwatch::new(),
    });
    commands.insert_resource(Score(0.0));
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
            transform: Transform::from_xyz(window.width() / 16.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * PLAYER_SIZE),
                flip_y: true,
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
fn update_timer(mut timer: Query<&mut Duration>, time: Res<Time>, mut query: Query<&mut Text>) {
    let mut timer = timer.single_mut();
    timer.time.tick(time.delta());

    for mut text in &mut query {
        text.sections[1].value = format!("{:.2}", timer.time.elapsed_secs());
    }
}
fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width(), window.height() / 2.0 + 100.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * ENEMY_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("bird.png"),
            ..Default::default()
        },
        Bird {
            direction: Direction::Right(random::<usize>() % 900),
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width(), window.height() / 2.0 - 5.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * ENEMY_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("cactus_short.png"),
            ..Default::default()
        },
        ShortCactus {
            direction: Direction::Left,
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width(), window.height() / 2.0 - 5.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * ENEMY_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("cactus_long.png"),
            ..Default::default()
        },
        ShortCactus {
            direction: Direction::Right(random::<usize>() % 325),
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width(), window.height() / 2.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * ENEMY_SIZE),
                ..Default::default()
            },
            texture: asset_server.load("cactus_long.png"),
            ..Default::default()
        },
        LongCactus {
            direction: Direction::Right(random::<usize>() % 645),
        },
    ));
}

fn reset_game(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut timer: Query<&mut Duration>,
    mut best: ResMut<Score>,
    mut next_state: ResMut<NextState<MyPausedState>>,
    mut query: Query<&mut Text>,
) {
    if input.just_pressed(KeyCode::Space) {
        let timed = &timer.single().time;
        if timed.elapsed().as_secs_f32() > best.0 {
            best.0 = (timed.elapsed().as_secs_f32());
            timer.single_mut().time.reset();
            for mut text in &mut query {
                text.sections[3].value = format!("{:.2}", best.0);
            }
        }
        next_state.set(MyPausedState::Running);
    }
}

fn jump_system_recieve(input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Dino>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        if transform.jump == JumpDirection::None {
            if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::ArrowUp) {
                transform.jump = JumpDirection::Down(JumpInnerDirection::Go(16))
            }
            if input.just_pressed(KeyCode::ArrowDown) {
                transform.jump = JumpDirection::Up(JumpInnerDirection::Go(16))
            }
        }
    }
}

fn jump_system(mut player_query: Query<(&mut Transform, &mut Dino)>) {
    for (mut tranform, mut player) in &mut player_query {
        match player.jump {
            JumpDirection::Up(JumpInnerDirection::Go(0)) => {
                tranform.translation.y += 5.;
                player.jump = JumpDirection::Up(JumpInnerDirection::Wait(25));
            }
            JumpDirection::Down(JumpInnerDirection::Reset(0)) => {
                tranform.translation.y += 5.;
                player.jump = JumpDirection::None;
            }
            JumpDirection::Down(JumpInnerDirection::Go(0)) => {
                tranform.translation.y -= 5.;
                player.jump = JumpDirection::Down(JumpInnerDirection::Wait(25));
            }
            JumpDirection::Up(JumpInnerDirection::Reset(0)) => {
                tranform.translation.y -= 5.;
                player.jump = JumpDirection::None;
            }
            JumpDirection::Down(JumpInnerDirection::Go(n)) => {
                tranform.translation.y -= 5.;
                player.jump = JumpDirection::Down(JumpInnerDirection::Go(n - 1));
            }
            JumpDirection::Up(JumpInnerDirection::Reset(n)) => {
                tranform.translation.y -= 5.;
                player.jump = JumpDirection::Up(JumpInnerDirection::Reset(n - 1));
            }
            JumpDirection::Up(JumpInnerDirection::Go(n)) => {
                tranform.translation.y += 5.;
                player.jump = JumpDirection::Up(JumpInnerDirection::Go(n - 1));
            }
            JumpDirection::Down(JumpInnerDirection::Reset(n)) => {
                tranform.translation.y += 5.;
                player.jump = JumpDirection::Down(JumpInnerDirection::Reset(n - 1));
            }
            JumpDirection::Up(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Up(JumpInnerDirection::Reset(16));
            }
            JumpDirection::Down(JumpInnerDirection::Wait(0)) => {
                player.jump = JumpDirection::Down(JumpInnerDirection::Reset(16));
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
fn enemy_movement1(mut enemy_query: Query<(&mut Transform, &mut Bird)>, time: Res<Time>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        match enemy.direction {
            Direction::Left => transform.translation.x -= 200. * time.delta_seconds(),
            Direction::Right(0) => {
                enemy.direction = Direction::Left;
            }
            Direction::Right(n) => {
                enemy.direction = Direction::Right(n - 1);
            }
        }
    }
}
fn enemy_movement2(mut enemy_query: Query<(&mut Transform, &mut LongCactus)>, time: Res<Time>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        match enemy.direction {
            Direction::Left => transform.translation.x -= 200. * time.delta_seconds(),
            Direction::Right(0) => {
                enemy.direction = Direction::Left;
            }
            Direction::Right(n) => {
                enemy.direction = Direction::Right(n - 1);
            }
        }
    }
}
fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut ShortCactus)>, time: Res<Time>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        match enemy.direction {
            Direction::Left => transform.translation.x -= 200. * time.delta_seconds(),
            Direction::Right(0) => {
                enemy.direction = Direction::Left;
            }
            Direction::Right(n) => {
                enemy.direction = Direction::Right(n - 1);
            }
        }
    }
}
fn enemy_bounds2(
    mut enemy_query: Query<(&mut Transform, &mut Bird)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        if translation.x < x_min {
            transform.translation.x = x_max;
            enemy.direction = Direction::Right(random::<usize>() % 700);
        }
    }
}
fn enemy_bounds1(
    mut enemy_query: Query<(&mut Transform, &mut LongCactus)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        if translation.x < x_min {
            transform.translation.x = x_max;
            enemy.direction = Direction::Right(random::<usize>() % 400)
        }
    }
}
fn enemy_bounds(
    mut enemy_query: Query<(&mut Transform, &mut ShortCactus)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        if translation.x < x_min {
            transform.translation.x = x_max;
            enemy.direction = Direction::Right(random::<usize>() % 200)
        }
    }
}
fn collision_detection(
    enemy_query: Query<(&Transform, &ShortCactus)>,
    enemy_query1: Query<(&Transform, &LongCactus)>,
    enemy_query2: Query<(&Transform, &Bird)>,
    player_query: Query<&Transform, With<Dino>>,
    mut next_state: ResMut<NextState<MyPausedState>>,
) {
    if let Ok(player) = player_query.get_single() {
        for (enemy_transform, enemy) in enemy_query.iter() {
            let distance = player.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("long cactus hit player! Game Over!");
                next_state.set(MyPausedState::Paused)
            }
            break;
        }
        for (enemy_transform, enemy) in enemy_query1.iter() {
            let distance = player.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("long cactus hit player! Game Over!");
                next_state.set(MyPausedState::Paused)
            }
            break;
        }
        for (enemy_transform, enemy) in enemy_query2.iter() {
            let distance = player.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Bird hit player! Game Over!");
                next_state.set(MyPausedState::Paused)
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
        .add_systems(Startup, spawn_timer)
        .add_systems(Startup, spawn_text)
        .add_systems(Startup, spawn_player)
        .add_plugins(Playing)
        .add_plugins(Paused)
        .run();
}
