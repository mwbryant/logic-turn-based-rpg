use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_turn_based_rpg::prelude::*;

pub const WIDTH: f32 = 1080.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
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
        .add_plugin(CombatPlugin)
        .add_plugin(ArtPlugin)
        .run();
}

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
