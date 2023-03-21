use crate::prelude::*;

use super::attack::check_death;

pub struct TurnBasedPlugin;

impl Plugin for TurnBasedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_attack.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(spawn_enemy_attack.in_schedule(OnEnter(CombatState::EnemyAttacking)))
            .add_system(wait_for_death.in_set(OnUpdate(CombatState::EnemyDying)))
            .add_systems(
                (player_action_timing, deal_damage, send_deaths)
                    .chain()
                    .in_set(OnUpdate(CombatState::PlayerAttacking)),
            )
            //I wish I could and an in set
            .add_systems(
                (player_action_timing, deal_damage, send_deaths)
                    .chain()
                    .in_set(OnUpdate(CombatState::EnemyAttacking)),
            );
    }
}

fn wait_for_death(mut next_state: ResMut<NextState<CombatState>>, dying: Query<&DeathAnimation>) {
    if dying.iter().count() == 0 {
        next_state.set(CombatState::EnemyAttacking);
    }
}

pub fn spawn_enemy_attack(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    //Without dying enemies
    enemy: Query<(Entity, &Transform), (With<Enemy>, Without<Player>, Without<Lifetime>)>,
) {
    //TODO attack based on enemy
    let (enemy, transform) = enemy.iter().next().expect("0 enemies");
    let player = player.get_single().expect("One player only!");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    let mut attack = Weapon::BasicSpear.get_attack_bundle(false, enemy, player);
    attack.animation.starting_x = transform.translation.x;
    commands.spawn(attack);
}

fn spawn_player_attack(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    locked_attack: Query<(Entity, &Weapon, &PlayerAttack)>,
) {
    let (entity, weapon, attack) = locked_attack.get_single().expect("No attack!");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    let player = player.get_single().expect("One player only!");

    commands
        .entity(entity)
        .insert(weapon.get_attack_bundle(true, player, attack.target));
}

fn player_action_timing(mut attack: Query<&mut Attack>, keyboard: Res<Input<KeyCode>>) {
    for mut attack in &mut attack {
        if keyboard.just_pressed(KeyCode::Space)
            && attack.action.action_input == ActionTiming::NotEntered
        {
            match attack.stages[attack.current_stage].0 {
                //FIXME should look at what is the current stage regardless of fixed step flow
                AttackStage::WalkUp => {
                    if attack.timer.percent() > 0.7 {
                        attack.action.action_input = ActionTiming::Early;
                    }
                }
                AttackStage::Action => {
                    attack.action.action_input = ActionTiming::Critical;
                }
                AttackStage::CoolDown => {
                    if attack.timer.percent() < 0.3 {
                        attack.action.action_input = ActionTiming::Late;
                    }
                }
                _ => {}
            }
        }
    }
}

fn deal_damage(mut hit_event: EventReader<HitEvent>, mut combatants: Query<&mut CombatStats>) {
    for hit in hit_event.iter() {
        let [mut target, attacker] = combatants
            .get_many_mut([hit.target, hit.attacker])
            .expect("Either target or attacker doesn't have stats");

        let modifer = if matches!(hit.action, ActionTiming::Critical) {
            if hit.player_attacking {
                2.0
            } else {
                0.5
            }
        } else {
            1.0
        };

        let damage = (((attacker.attack - target.defense) as f32 * modifer) as i32).clamp(0, 99);
        target.health -= damage;
    }
}

fn send_deaths(
    mut hit_event: EventReader<HitEvent>,
    player: Query<(Entity, &CombatStats), With<Player>>,
    enemy: Query<(Entity, &CombatStats), (With<Enemy>, Without<Player>)>,
    mut death_event: EventWriter<DeathEvent>,
) {
    if hit_event.iter().count() > 0 {
        let (_death_state, entity) = check_death(&player, &enemy);

        if let Some(entity) = entity {
            info!("Sending death");

            death_event.send(DeathEvent { entity });
        }
    }
}
