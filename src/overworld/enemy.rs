use std::time::Duration;

use bevy::math::Vec3Swizzles;
use rand::Rng;

use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_start_combat.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(enemy_wander.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(despawn_with::<OverworldEntity>.in_schedule(OnExit(GameState::Overworld)));
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
    assets: Res<AssetServer>,
) {
    let transform = player.get_single().expect("Only 1 Player");

    for (enemy_transform, enemy, id) in &enemies {
        if Vec3::distance(transform.translation, enemy_transform.translation) < 0.5 {
            info!("Spawning combat");
            let combat: Handle<CombatDescriptor> = assets.load(&enemy.combat_ref);
            commands.spawn((combat, CombatStartTag));
            overworld_state.set(OverworldState::CombatStarting);

            let fadeout = spawn_fadeout(&mut commands);
            commands.entity(fadeout).insert(CombatFadeout);

            room.enemies.retain(|(room_id, _, _)| *room_id != id.0);
            return;
        }
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

        if (transform.translation.xzy().truncate() - enemy.home).length() > enemy.wander_range {
            enemy.direction = -(transform.translation.xzy().truncate() - enemy.home).normalize();
        }

        transform.translation +=
            Vec3::new(enemy.direction.x, 0.0, enemy.direction.y) * time.delta_seconds();
    }
}
