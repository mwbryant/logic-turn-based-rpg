use std::time::Duration;

use rand::Rng;

use crate::{comp_from_config, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_start_combat.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(enemy_wander.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(despawn_with::<EnemyOverworld>.in_schedule(OnExit(GameState::Overworld)))
            .add_system(despawn_with::<OverworldEntity>.in_schedule(OnExit(GameState::Overworld)))
            .add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)));
    }
}

//TODO Use physics or enemy holds range
fn enemy_start_combat(
    mut commands: Commands,
    enemies: Query<(&Transform, &EnemyOverworld, &EnemyId)>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<EnemyOverworld>)>,
    //TODO shouldn't remove from room if player runs from battle
    mut room: ResMut<CurrentRoom>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
) {
    let transform = player.get_single().expect("Only 1 Player");
    for (enemy_transform, enemy, id) in &enemies {
        if Vec2::distance(
            transform.translation.truncate(),
            enemy_transform.translation.truncate(),
        ) < 0.5
        {
            commands.spawn(comp_from_config!(CombatDescriptor, &enemy.combat_ref));
            overworld_state.set(OverworldState::CombatStarting);
            room.enemies.retain(|(room_id, _, _)| *room_id != id.0);
            return;
        }
    }
}

//FIXME move this logic to combat
fn start_combat(
    mut commands: Commands,
    combat_descriptor: Query<(Entity, &CombatDescriptor)>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut main_game_state: ResMut<NextState<GameState>>,
    // TODO combat state for starting
    mut combat_state: ResMut<NextState<CombatState>>,
    assets: Res<AssetServer>,
) {
    assert!(combat_descriptor.iter().count() <= 1);
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

fn enemy_wander(mut enemies: Query<(&mut Transform, &mut EnemyOverworld)>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for (mut transform, mut enemy) in &mut enemies {
        enemy.new_direction_timer.tick(time.delta());
        if enemy.new_direction_timer.just_finished() {
            let wander_time = rng.gen_range(1.5..3.0);

            enemy
                .new_direction_timer
                .set_duration(Duration::from_secs_f32(wander_time));

            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            enemy.direction = Vec2::new(x, y).normalize();
        }

        if (transform.translation.truncate() - enemy.home).length() > enemy.wander_range {
            enemy.direction = -(transform.translation.truncate() - enemy.home).normalize();
        }

        transform.translation += enemy.direction.extend(0.0) * time.delta_seconds();
    }
}
