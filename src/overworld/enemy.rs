use std::time::Duration;

use rand::Rng;

use crate::{comp_from_config, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_test)
            .add_system(enemy_start_combat.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(enemy_wander.in_set(OnUpdate(OverworldState::FreeRoam)))
            .add_system(despawn_with::<EnemyOverworld>.in_schedule(OnExit(GameState::Overworld)))
            .add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)));
    }
}

fn setup_test(mut commands: Commands) {
    let enemies = ["config/basic_enemy.ron", "config/basic_enemy2.ron"];

    for config in enemies {
        //FIXME windows uses \ .. fix in macro
        let enemy = comp_from_config!(EnemyOverworld, config);
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

fn start_combat(
    mut commands: Commands,
    combat_descriptor: Query<&CombatDescriptor>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut main_game_state: ResMut<NextState<GameState>>,
    // TODO combat state for starting
    mut combat_state: ResMut<NextState<CombatState>>,
    assets: Res<AssetServer>,
) {
    assert!(combat_descriptor.iter().count() <= 1);
    for combat_desc in &combat_descriptor {
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
