use crate::prelude::*;

pub struct StartCombatPlugin;

impl Plugin for StartCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_combat.in_schedule(OnEnter(GameState::Combat)));
    }
}

pub fn spawn_combat(
    mut commands: Commands,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut player: Query<&mut Transform, With<PlayerCombat>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerCombat>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    combats: Res<Assets<CombatDescriptor>>,
    assets: Res<AssetServer>,
) {
    let mut player = player.single_mut();
    player.translation = Vec3::new(-3.0, 0.5, 0.0);
    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(0.0, 2.0, 6.5);
    camera.look_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);

    let (entity, combat_desc) = &combat_descriptor.single();
    commands.entity(*entity).despawn_recursive();
    // FIXME this is a kinda unsound assumption...
    let combat_desc = combats
        .get(combat_desc)
        .expect("Combat should have loaded by end of fade...");

    // Spawn all the enemies
    for (enemy, stats, character) in combat_desc.enemies.iter() {
        let x = match enemy.slot {
            0 => 0.6,
            1 => 1.8,
            2 => 3.0,
            3 => 4.2,
            _ => unreachable!("bad slot"),
        };
        commands.spawn((
            BillboardSprite::Character(character.clone()),
            HandOffset {
                left: Vec2::new(-0.40, -0.37),
                right: Vec2::new(0.35, -0.37),
            },
            SpatialBundle::from(Transform::from_xyz(x, 0.5, 0.0)),
            *enemy,
            *stats,
            Name::new("Enemy"),
            CombatEntity,
        ));
    }

    let my_gltf = assets.load("3d/stage.glb#Scene0");

    commands.spawn((
        SceneBundle {
            scene: my_gltf,
            ..Default::default()
        },
        Name::new("Stage Mesh"),
        CombatEntity,
    ));

    /*
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(26.0, 11.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z - 0.1),
            texture: assets.load("CaveBackground.png"),
            ..default()
        },
        Name::new("Background"),
        CombatEntity,
    ));
    */

    /*
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
