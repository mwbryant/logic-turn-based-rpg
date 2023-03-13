use crate::prelude::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::PlayerAttacking)))
            .add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::EnemyAttacking)))
            .add_system(attack_flow.in_set(OnUpdate(CombatState::PlayerAttacking)))
            .add_system(attack_flow.in_set(OnUpdate(CombatState::EnemyAttacking)));
    }
}

fn attack_flow(
    mut attack: Query<&mut Attack>,
    time: Res<Time>,
    mut hit_event: EventWriter<HitEvent>,
    state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for mut attack in &mut attack {
        match attack.stage {
            AttackStages::Warmup => {
                attack.warmup_timer.tick(time.delta());
                if attack.warmup_timer.just_finished() {
                    attack.stage = AttackStages::Action;
                }
            }
            AttackStages::Action => {
                attack.action_timer.tick(time.delta());
                if attack.action_timer.just_finished() {
                    hit_event.send(HitEvent {
                        action: attack.action_input,
                        combat_state: state.0.clone(),
                    });
                    attack.stage = AttackStages::CoolDown;
                }
            }
            AttackStages::CoolDown => {
                attack.cool_down_timer.tick(time.delta());
                if attack.cool_down_timer.just_finished() {
                    info!("Attack Complete");
                    match state.0 {
                        CombatState::PlayerAttacking => next_state.set(CombatState::EnemyAttacking),
                        CombatState::EnemyAttacking => next_state.set(CombatState::PlayerSelecting),
                        _ => unreachable!("Can't finish attack in this state"),
                    }
                }
            }
        }
    }
}
