use bevy::{input::common_conditions::input_toggle_active, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use logic_turn_based_rpg::{comp_from_config, prelude::*};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

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
        CharacterBundle::new(Vec3::new(-3.0, 0.0, CHARACTER_Z), Character::Knight),
        PlayerCombat::default(),
        comp_from_config!(PlayerOverworld, "config/player_overworld.ron"),
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
