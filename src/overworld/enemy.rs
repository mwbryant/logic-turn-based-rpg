use std::time::Duration;

use rand::Rng;

use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_test).add_system(enemy_wander);
    }
}

fn setup_test(mut commands: Commands) {
    let homes = [Vec2::new(1.0, 0.5), Vec2::new(-2.0, -3.0)];

    for home in homes {
        commands.spawn((
            CharacterBundle::new(home, Character::GreenBase),
            EnemyOverworld {
                movement_speed: 0.3,
                chase_movement_speed: 2.3,
                home,
                direction: Vec2::ONE,
                new_direction_timer: Timer::from_seconds(0.01, TimerMode::Repeating),
                wander_range: 1.5,
                follow_range: 1.0,
                combat_ref: "".to_string(),
            },
            Name::new("Enemy"),
        ));
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
