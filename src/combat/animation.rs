use crate::prelude::*;
use bevy_easings::Lerp;

use super::turn_based::spawn_enemy_attack;

pub struct CombatAnimationPlugin;

impl Plugin for CombatAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_weapon.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(
                despawn_with::<WeaponGraphic>.in_schedule(OnExit(CombatState::PlayerAttacking)),
            )
            .add_systems(
                (apply_system_buffers, spawn_enemy_weapon)
                    .chain()
                    .after(spawn_enemy_attack)
                    .in_schedule(OnEnter(CombatState::EnemyAttacking)),
            )
            .add_system(
                despawn_with::<WeaponGraphic>.in_schedule(OnExit(CombatState::EnemyAttacking)),
            )
            .add_system(
                animate_melee::<Player>
                    .in_set(OnUpdate(CombatState::PlayerAttacking))
                    .in_set(CombatSet::Animation),
            )
            .add_system(
                animate_melee::<Enemy>
                    .in_set(OnUpdate(CombatState::EnemyAttacking))
                    .in_set(CombatSet::Animation),
            )
            .add_system(
                animate_ranged::<Player>
                    .in_set(OnUpdate(CombatState::PlayerAttacking))
                    .in_set(CombatSet::Animation),
            )
            .add_system(
                animate_ranged::<Enemy>
                    .in_set(OnUpdate(CombatState::EnemyAttacking))
                    .in_set(CombatSet::Animation),
            );
    }
}

//TODO player location determined from config somehow
fn animate_melee<T: Component>(
    mut attacker: Query<(&mut Transform, &Children), With<T>>,
    attack: Query<(&Attack, &AttackAnimation)>,
    mut weapon: Query<&mut Transform, (With<Weapon>, Without<T>)>,
) {
    let (mut transform, children) = attacker.get_single_mut().expect("No or multiple attackers");

    let child = children
        .iter()
        .find(|&&child| weapon.contains(child))
        .expect("Attacker doesn't have a weapon sprite");
    let mut child_transform = weapon.get_mut(*child).unwrap();

    if let Ok((attack, animation)) = attack.get_single() {
        if !matches!(attack.attack_type, WeaponAttackType::Melee) {
            return;
        }
        match attack.stages[attack.current_stage].0 {
            AttackStage::WalkUp => {
                transform.translation.x =
                    (animation.starting_x).lerp(&animation.ending_x, &attack.timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
            AttackStage::Action => {
                transform.translation.x = animation.ending_x;
                if attack.timer.percent() < 0.5 {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &0.0,
                        &animation.max_weapon_rotation,
                        &attack.timer.percent(),
                    ));
                } else {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &animation.max_weapon_rotation,
                        &0.0,
                        &attack.timer.percent(),
                    ));
                }
            }
            AttackStage::CoolDown => {
                transform.translation.x =
                    (animation.ending_x).lerp(&animation.starting_x, &attack.timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
            _ => {}
        }
    }
}

fn animate_ranged<T: Component>(
    //This system shouldn't be responisble for spawning the fireball..
    mut commands: Commands,
    mut attacker: Query<(&mut Transform, &Children), With<T>>,
    attack: Query<(&Attack, &AttackAnimation)>,
    mut projectile: Query<
        (Entity, &mut Transform),
        (
            With<WeaponGraphic>,
            With<Projectile>,
            Without<T>,
            Without<Weapon>,
        ),
    >,
    mut weapon: Query<&mut Transform, (With<Weapon>, Without<T>)>,
) {
    let (mut _transform, children) = attacker.get_single_mut().expect("No or multiple attackers");

    let child = children
        .iter()
        .find(|&&child| weapon.contains(child))
        .expect("Attacker doesn't have a weapon sprite");
    let mut child_transform = weapon.get_mut(*child).unwrap();

    if let Ok((attack, animation)) = attack.get_single() {
        if !matches!(attack.attack_type, WeaponAttackType::Range) {
            return;
        }
        match attack.stages[attack.current_stage].0 {
            AttackStage::Charge => {
                //Probably a more elegant solution to this
                if attack.timer.percent() < 0.5 {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &0.0,
                        &animation.max_weapon_rotation,
                        &attack.timer.percent(),
                    ));
                } else {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &animation.max_weapon_rotation,
                        &0.0,
                        &attack.timer.percent(),
                    ));
                }
            }
            AttackStage::WalkUp => {
                if projectile.iter().count() == 0 {
                    //spawn projectile
                    commands.spawn((
                        PlanetBundle::new(Vec2::ZERO, Planet::Fireball),
                        Projectile,
                        WeaponGraphic,
                    ));
                } else {
                    let (_, mut projectile_transform) =
                        projectile.get_single_mut().expect("too many projectiles");
                    projectile_transform.translation.x =
                        (animation.starting_x).lerp(&animation.ending_x, &attack.timer.percent());
                }
            }
            AttackStage::Action => {
                child_transform.rotation = Quat::from_rotation_z(0.0);
                if let Ok((entity, _)) = projectile.get_single_mut() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

//TODO make generic
fn spawn_enemy_weapon(
    mut commands: Commands,
    attack: Query<&Attack>,
    enemy: Query<Entity, With<Enemy>>,
) {
    let weapon = Weapon::BasicSpear;
    let attack = attack.get_single().expect("No attack or multiple");
    let enemy = enemy.get(attack.attacker).expect("Attacker is not enemy");

    let new_ent = commands
        .spawn((
            WeaponBundle::new(
                //FIXME magic config
                Vec2::new(-0.40, -0.37),
                weapon,
                Vec2::splat(1.0),
            ),
            WeaponGraphic,
            Name::new("Weapon Graphic"),
        ))
        .id();
    commands.entity(enemy).add_child(new_ent);
}

fn spawn_player_weapon(
    mut commands: Commands,
    locked_attack: Query<&Weapon, With<PlayerAttack>>,
    player: Query<Entity, With<Player>>,
) {
    let weapon = locked_attack.get_single().expect("No attack selected :(");
    let player = player.get_single().expect("No Player");
    let new_ent = commands
        .spawn((
            WeaponBundle::new(
                //FIXME magic config
                Vec2::new(0.35, -0.37),
                weapon.clone(),
                Vec2::splat(1.0),
            ),
            WeaponGraphic,
            Name::new("Weapon Graphic"),
        ))
        .id();
    commands.entity(player).add_child(new_ent);
}
