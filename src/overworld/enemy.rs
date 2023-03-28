use std::time::Duration;

use rand::Rng;

use crate::{comp_from_config, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_test)
            .add_system(enemy_start_combat.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(enemy_wander.in_set(OnUpdate(OverworldState::FreeRoam)));
    }
}

fn setup_test(mut commands: Commands) {
    let homes = [Vec2::new(1.0, 0.5), Vec2::new(-2.0, -3.0)];

    for home in homes {
        //FIXME windows uses \ .. fix in macro
        let enemy = comp_from_config!(EnemyOverworld, "config/basic_enemy.ron");
        let mut character = CharacterBundle::new(enemy.home, Character::GreenBase);
        character.sprite_sheet.transform.translation.z = ENEMY_Z;
        commands.spawn((enemy, character, Name::new("Enemy")));
    }
}

//TODO Use physics or enemy holds range
fn enemy_start_combat(
    mut commands: Commands,
    enemies: Query<(&Transform, &EnemyOverworld)>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<EnemyOverworld>)>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
) {
    let transform = player.get_single().expect("Only 1 Player");
    for (enemy_transform, enemy) in &enemies {
        if Vec2::distance(
            transform.translation.truncate(),
            enemy_transform.translation.truncate(),
        ) < 0.5
        {
            commands.spawn(comp_from_config!(CombatDescriptor, &enemy.combat_ref));
            overworld_state.set(OverworldState::CombatStarting);
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

        if (transform.translation.truncate() - enemy.home).length() > enemy.wander_range {
            enemy.direction = -(transform.translation.truncate() - enemy.home).normalize();
        }

        transform.translation += enemy.direction.extend(0.0) * time.delta_seconds();
    }
}
