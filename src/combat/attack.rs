use crate::prelude::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::PlayerAttacking)))
            .add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::EnemyAttacking)))
            .add_system(
                attack_flow
                    .in_set(OnUpdate(CombatState::PlayerAttacking))
                    .in_set(CombatSet::Logic),
            )
            .add_system(
                attack_flow
                    .in_set(OnUpdate(CombatState::EnemyAttacking))
                    .in_set(CombatSet::Logic),
            );
    }
}

pub enum DeathState {
    NoChange,
    PlayerDied,
    //TODO handle multiple enemies
    EnemyDied,
    AllEnemiesDead,
}

//FIXME really bad and fragile
pub fn check_death(
    player: &Query<(Entity, &CombatStats), With<Player>>,
    enemy: &Query<(Entity, &CombatStats), (With<Enemy>, Without<Player>)>,
) -> (DeathState, Option<Entity>) {
    let player = player.get_single().expect("No player");
    let enemy_count = enemy.iter().count();

    if enemy_count == 0 {
        return (DeathState::AllEnemiesDead, None);
    }

    for enemy in enemy {
        if enemy.1.health <= 0 {
            if enemy_count == 1 {
                return (DeathState::AllEnemiesDead, Some(enemy.0));
            } else {
                return (DeathState::EnemyDied, Some(enemy.0));
            }
        }
    }

    if player.1.health <= 0 {
        return (DeathState::PlayerDied, Some(player.0));
    }
    (DeathState::NoChange, None)
}

fn attack_flow(
    mut attack: Query<&mut Attack>,
    time: Res<Time>,
    mut hit_event: EventWriter<HitEvent>,
    state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
    player: Query<(Entity, &CombatStats), With<Player>>,
    enemy: Query<(Entity, &CombatStats), (With<Enemy>, Without<Player>)>,
) {
    for mut attack in &mut attack {
        attack.timer.tick(time.delta());
        if attack.timer.just_finished() {
            let finished_stage = &attack.stages[attack.current_stage].0;
            if matches!(finished_stage, AttackStage::Action) {
                hit_event.send(HitEvent {
                    attacker: attack.attacker,
                    target: attack.target,
                    player_attacking: matches!(state.0, CombatState::PlayerAttacking),
                    action: attack.action.action_input,
                    combat_state: state.0.clone(),
                });
            }

            attack.current_stage += 1;

            //Turn ending
            if attack.current_stage >= attack.stages.len() {
                attack.current_stage = attack.stages.len() - 1;

                //FIXME not the place to check this
                let (death_state, _entity) = check_death(&player, &enemy);

                match death_state {
                    DeathState::NoChange => match state.0 {
                        CombatState::PlayerAttacking => next_state.set(CombatState::EnemyAttacking),
                        CombatState::EnemyAttacking => next_state.set(CombatState::PlayerSelecting),
                        _ => unreachable!("Can't finish attack in this state"),
                    },
                    DeathState::AllEnemiesDead => next_state.set(CombatState::PlayerWins),
                    DeathState::EnemyDied => next_state.set(CombatState::EnemyDying),
                    _ => unreachable!("Bad death state"),
                }
                return;
            }

            let next_timer = attack.stages[attack.current_stage].1;
            attack.timer = Timer::from_seconds(next_timer, TimerMode::Once);
        }
    }
}
