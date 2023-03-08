use std::thread::spawn;

use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_bevy_new_scheduling::prelude::*;

pub const WIDTH: f32 = 1080.0;
pub const HEIGHT: f32 = 720.0;

#[derive(Component)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Default)]
pub struct Player {
    pub experience: i32,
    pub level: i32,
}

#[derive(Component)]
pub struct Enemy {
    pub base_experience_reward: i32,
}

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
enum CombatState {
    #[default]
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
}

fn main() {
    App::new()
        .add_state::<CombatState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Game".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_startup_system(setup)
        .add_system(spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)))
        .add_plugin(ArtPlugin)
        .run();
}

fn spawn_player_attack_icons(mut commands: Commands) {
    commands.spawn(WeaponBundle::new(
        Vec2::new(-3.0, 1.7),
        Weapon::BasicSpear,
        Vec2::splat(0.75),
    ));

    commands.spawn(WeaponBundle::new(
        Vec2::new(-2.5, 1.7),
        Weapon::BasicStaffOrange,
        Vec2::splat(0.75),
    ));

    commands.spawn(IconBundle::new(
        Vec2::new(-3.0, 1.0),
        Icon::Pointer,
        Vec2::splat(0.5),
    ));
}

fn player_select_attack(player: Query<&Player>) {}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    let size = 10.0;

    camera.projection.scaling_mode = ScalingMode::FixedVertical(size);

    commands.spawn(camera);

    commands.spawn((
        CharacterBundle::new(Vec2::new(-3.0, 0.0), Character::Knight),
        Player::default(),
        CombatStats {
            health: 10,
            max_health: 10,
            attack: 1,
            defense: 0,
        },
    ));

    commands.spawn((
        CharacterBundle::new(Vec2::new(3.0, 0.0), Character::GreenBase),
        Enemy {
            base_experience_reward: 5,
        },
        CombatStats {
            health: 2,
            max_health: 2,
            attack: 1,
            defense: 0,
        },
    ));
}
