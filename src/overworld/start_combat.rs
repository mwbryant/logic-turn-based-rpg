use crate::prelude::*;

pub struct CombatTransitionPlugin;

impl Plugin for CombatTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)));
    }
}

#[allow(clippy::too_many_arguments)]
fn start_combat(
    mut commands: Commands,
    fadeout: Query<(&Fadeout, &CombatFadeout)>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut main_game_state: ResMut<NextState<GameState>>,
    mut combat_state: ResMut<NextState<CombatState>>,
    combats: Res<Assets<CombatDescriptor>>,
    assets: Res<AssetServer>,
) {
    //Check that fade completes
    assert!(combat_descriptor.iter().count() == 1);
    if let Ok((fadeout, _)) = fadeout.get_single() {
        if !fadeout.fade_in_just_finished {
            return;
        }
        info!("Starting Combat");
    } else {
        warn!("No fadeout for entering combat!");
        return;
    }

    //Find the enemy that we encountered and get the combat descriptor
    let (_, combat) = fadeout.single();
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
        let character = CharacterBundle::new(Vec3::new(x, 0.0, ENEMY_Z), character.clone());
        commands.spawn((character, *enemy, *stats, Name::new("Enemy"), CombatEntity));
    }

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

    overworld_state.set(OverworldState::NotInOverworld);
    main_game_state.set(GameState::Combat);
    combat_state.set(CombatState::PlayerSelecting);
}
