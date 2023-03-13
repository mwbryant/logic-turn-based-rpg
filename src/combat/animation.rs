use crate::prelude::*;
use bevy_easings::Lerp;

pub struct CombatAnimationPlugin;

impl Plugin for CombatAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_weapon.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(
                despawn_with::<WeaponGraphic>.in_schedule(OnExit(CombatState::PlayerAttacking)),
            )
            .add_system(spawn_enemy_weapon.in_schedule(OnEnter(CombatState::EnemyAttacking)))
            .add_system(
                despawn_with::<WeaponGraphic>.in_schedule(OnExit(CombatState::EnemyAttacking)),
            )
            .add_system(animate_melee::<Player>.in_set(OnUpdate(CombatState::PlayerAttacking)))
            .add_system(animate_melee::<Enemy>.in_set(OnUpdate(CombatState::EnemyAttacking)));
    }
}

//TODO player location determined from config somehow
fn animate_melee<T: Component>(
    mut attacker: Query<(&mut Transform, &Children), With<T>>,
    attack: Query<(&MeleeAttack, &AttackAnimation)>,
    mut weapon: Query<&mut Transform, (With<Weapon>, Without<T>)>,
) {
    let (mut transform, children) = attacker.get_single_mut().expect("No or multiple attackers");

    let child = children
        .iter()
        .find(|&&child| weapon.contains(child))
        .expect("Attacker doesn't have a weapon sprite");
    let mut child_transform = weapon.get_mut(*child).unwrap();

    if let Ok((attack, animation)) = attack.get_single() {
        match attack.stage {
            AttackStages::Warmup => {
                transform.translation.x = (animation.starting_x)
                    .lerp(&animation.ending_x, &attack.warmup_timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
            AttackStages::Action => {
                transform.translation.x = animation.ending_x;
                //Probably a more elegant solution to this
                if attack.action_timer.percent() < 0.5 {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &0.0,
                        &animation.max_weapon_rotation,
                        &attack.action_timer.percent(),
                    ));
                } else {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &animation.max_weapon_rotation,
                        &0.0,
                        &attack.action_timer.percent(),
                    ));
                }
            }
            AttackStages::CoolDown => {
                transform.translation.x = (animation.ending_x)
                    .lerp(&animation.starting_x, &attack.cool_down_timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
        }
    }
}

//TODO make generic
fn spawn_enemy_weapon(mut commands: Commands, enemy: Query<Entity, With<Enemy>>) {
    let weapon = Weapon::BasicSpear;
    let enemy = enemy.get_single().expect("No Enemy");
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
