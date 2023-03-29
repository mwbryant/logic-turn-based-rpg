use crate::prelude::*;

pub struct CombatTransitionPlugin;

impl Plugin for CombatTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)));
    }
}

//FIXME move this logic to combat
fn start_combat(
    mut commands: Commands,
    fadeout: Query<&Fadeout, With<CombatFadeout>>,
    combat_descriptor: Query<(Entity, &CombatDescriptor)>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut main_game_state: ResMut<NextState<GameState>>,
    // TODO combat state for starting
    mut combat_state: ResMut<NextState<CombatState>>,
    assets: Res<AssetServer>,
) {
    assert!(combat_descriptor.iter().count() <= 1);
    if let Ok(fadeout) = fadeout.get_single() {
        if !fadeout.fade_in_just_finished {
            return;
        }
    } else {
        warn!("No fadeout for entering combat!");
    }

    for (entity, combat_desc) in &combat_descriptor {
        commands.entity(entity).despawn_recursive();
        info!("Starting combat");
        for (enemy, stats, character) in combat_desc.enemies.iter() {
            let x = match enemy.slot {
                0 => 0.6,
                1 => 1.8,
                2 => 3.0,
                3 => 4.2,
                _ => unreachable!("bad slot"),
            };
            let character = CharacterBundle::new(Vec2::new(x, 0.0), character.clone());
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
}
