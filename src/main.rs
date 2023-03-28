use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_turn_based_rpg::{comp_from_config, prelude::*};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_system(despawn_with::<CombatEntity>.in_schedule(OnExit(GameState::Combat)))
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
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_startup_system(setup)
        .add_system(update_lifetimes.in_base_set(CoreSet::PostUpdate))
        .add_plugin(CombatPlugin)
        .add_plugin(OverWorldPlugin)
        .add_plugin(ArtPlugin);

    app.run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    let size = 10.0;

    camera.projection.scaling_mode = ScalingMode::FixedVertical(size);

    commands.spawn(camera);

    commands.spawn((
        CharacterBundle::new(Vec2::new(-3.0, 0.0), Character::Knight),
        PlayerCombat::default(),
        comp_from_config!(PlayerOverworld, "config/player_overworld.ron"),
        //PlayerOverworld {
        //movement_speed: 2.5,
        //},
        CombatStats {
            health: 10,
            max_health: 10,
            attack: 1,
            defense: 0,
        },
        Name::new("Player"),
    ));

    /*

    commands.spawn((
        CharacterBundle::new(Vec2::new(0.6, 0.0), Character::GreenBase),
        Enemy {
            slot: 0,
            base_experience_reward: 5,
        },
        CombatStats {
            health: 2,
            max_health: 2,
            attack: 1,
            defense: 0,
        },
        Name::new("Enemy"),
        CombatEntity,
    ));

    commands.spawn((
        CharacterBundle::new(Vec2::new(1.8, 0.0), Character::GreenBase),
        Enemy {
            slot: 1,
            base_experience_reward: 5,
        },
        CombatStats {
            health: 2,
            max_health: 2,
            attack: 1,
            defense: 0,
        },
        Name::new("Enemy"),
        CombatEntity,
    ));

    commands.spawn((
        CharacterBundle::new(Vec2::new(3.0, 0.0), Character::GreenBase),
        Enemy {
            slot: 2,
            base_experience_reward: 5,
        },
        CombatStats {
            health: 2,
            max_health: 2,
            attack: 1,
            defense: 0,
        },
        Name::new("Enemy"),
        CombatEntity,
    ));

    commands.spawn((
        CharacterBundle::new(Vec2::new(4.2, 0.0), Character::ManOld),
        Enemy {
            //TODO use slots to look up into a resource for x,y and ui positioning
            slot: 3,
            base_experience_reward: 5,
        },
        CombatStats {
            health: 4,
            max_health: 4,
            attack: 1,
            defense: 0,
        },
        Name::new("Enemy"),
        CombatEntity,
    ));

    /*
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.1, 0.1)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.9),
            ..default()
        },
        Name::new("WhiteDot"),
    ));
    */

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(11.0, 7.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -1.2, BACKGROUND_Z),
            texture: assets.load("Stage.png"),
            ..default()
        },
        Name::new("Background"),
        CombatEntity,
    ));
    */
}
