use std::f32::consts::PI;

use crate::prelude::*;

pub struct TurnBasedPlugin;

impl Plugin for TurnBasedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_attack.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(spawn_enemy_attack.in_schedule(OnEnter(CombatState::EnemyAttacking)))
            .add_system(check_death.in_schedule(OnExit(CombatState::PlayerAttacking)))
            .add_system(check_death.in_schedule(OnExit(CombatState::EnemyAttacking)))
            .add_systems(
                (player_action_timing, deal_damage).in_set(OnUpdate(CombatState::PlayerAttacking)),
            )
            //I wish I could and an in set
            .add_systems(
                (player_action_timing, deal_damage).in_set(OnUpdate(CombatState::EnemyAttacking)),
            );
    }
}

fn spawn_enemy_attack(mut commands: Commands, enemy: Query<&Enemy>) {
    //TODO attack based on enemy
    let _enemy = enemy.get_single().expect("More than 1 or 0 enemies...");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    commands.spawn((
        MeleeAttack {
            stage: AttackStages::Warmup,
            action_input: ActionTiming::NotEntered,
            warmup_timer: Timer::from_seconds(1.0, TimerMode::Once),
            action_timer: Timer::from_seconds(0.2, TimerMode::Once),
            cool_down_timer: Timer::from_seconds(0.7, TimerMode::Once),
        },
        AttackAnimation {
            starting_x: 3.0,
            ending_x: -1.9,
            max_weapon_rotation: 6.0 * PI,
        },
    ));
}

fn spawn_player_attack(
    mut commands: Commands,
    locked_attack: Query<(Entity, &Weapon), With<PlayerAttack>>,
) {
    let (entity, weapon) = locked_attack.get_single().expect("No attack!");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    let attack_type = weapon.attack_type();
    match attack_type {
        WeaponAttackType::Melee => {
            commands.entity(entity).insert((
                MeleeAttack {
                    stage: AttackStages::Warmup,
                    action_input: ActionTiming::NotEntered,
                    warmup_timer: Timer::from_seconds(1.0, TimerMode::Once),
                    action_timer: Timer::from_seconds(0.2, TimerMode::Once),
                    cool_down_timer: Timer::from_seconds(0.7, TimerMode::Once),
                },
                //FIXME this should be from a bundle or something...
                AttackAnimation {
                    starting_x: -3.0,
                    ending_x: 1.9,
                    max_weapon_rotation: -1.0,
                },
            ));
        }
        WeaponAttackType::Range => todo!(),
    }
}

fn player_action_timing(mut attack: Query<&mut MeleeAttack>, keyboard: Res<Input<KeyCode>>) {
    for mut attack in &mut attack {
        if keyboard.just_pressed(KeyCode::Space) && attack.action_input == ActionTiming::NotEntered
        {
            match attack.stage {
                AttackStages::Warmup => {
                    if attack.warmup_timer.percent() > 0.7 {
                        attack.action_input = ActionTiming::Early;
                    }
                }
                AttackStages::Action => {
                    attack.action_input = ActionTiming::Critical;
                }
                AttackStages::CoolDown => {
                    if attack.cool_down_timer.percent() < 0.3 {
                        attack.action_input = ActionTiming::Late;
                    }
                }
            }
        }
    }
}

//FIXME should happen after deal damage but state change should happen after attack cooldown
fn check_death(
    player: Query<&CombatStats, With<Player>>,
    enemy: Query<&CombatStats, (With<Enemy>, Without<Player>)>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    let player = player.get_single().expect("No player");
    let enemy = enemy.get_single().expect("No enemy");
    if enemy.health <= 0 {
        info!("Enemy Died");
        next_state.set(CombatState::PlayerWins);
    }
    if player.health <= 0 {
        todo!();
    }
}

fn deal_damage(
    mut hit_event: EventReader<HitEvent>,
    mut player: Query<&mut CombatStats, With<Player>>,
    mut enemy: Query<&mut CombatStats, (With<Enemy>, Without<Player>)>,
) {
    for hit in hit_event.iter() {
        let mut player = player.get_single_mut().expect("No player");
        let mut enemy = enemy.get_single_mut().expect("No enemy");

        match hit.combat_state {
            CombatState::PlayerSelecting | CombatState::PlayerWins => {
                unreachable!("Can't hit in this state")
            }
            CombatState::PlayerAttacking => {
                let damage = (if matches!(hit.action, ActionTiming::Critical) {
                    (player.attack - enemy.defense) * 2
                } else {
                    player.attack - enemy.defense
                })
                .clamp(0, 99);
                info!("player hit, {:?} {:?}", hit.action, damage);
                enemy.health -= damage;
            }
            CombatState::EnemyAttacking => {
                let damage = (if matches!(hit.action, ActionTiming::Critical) {
                    (enemy.attack - player.defense) / 2
                } else {
                    enemy.attack - player.defense
                })
                .clamp(0, 99);
                info!("enemy hit, {:?} {:?}", hit.action, damage);
                player.health -= damage;
            }
        }
    }
}
