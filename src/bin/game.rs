use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_turn_based_rpg::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_system(despawn_with::<CombatEntity>.in_schedule(OnExit(GameState::Combat)))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Game".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(update_lifetimes.in_base_set(CoreSet::PostUpdate))
        .add_plugin(CombatPlugin)
        .add_plugin(OverWorldPlugin)
        .add_plugin(ArtPlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        CharacterBundle::new(Vec3::new(-3.0, 0.0, CHARACTER_Z), Character::Knight),
        PlayerCombat::default(),
        PlayerOverworld {
            movement_speed: 2.5,
        },
        CombatStats {
            health: 10,
            max_health: 10,
            attack: 1,
            defense: 0,
        },
        RigidBody::KinematicPositionBased,
        Collider::round_cuboid(0.09, 0.11, 0.006),
        KinematicCharacterController::default(),
        Name::new("Player"),
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
}