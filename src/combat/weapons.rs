use std::f32::consts::PI;

use crate::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Weapon {
    pub fn attack_type(&self) -> WeaponAttackType {
        match self {
            Weapon::BasicSpear => WeaponAttackType::Melee,
            Weapon::BasicStaffOrange => WeaponAttackType::Range,
        }
    }

    pub fn get_attack_bundle(
        &self,
        player: bool,
        attacker: Entity,
        target: Entity,
        slot: usize,
    ) -> AttackBundle {
        let animation = if player {
            match self {
                Weapon::BasicStaffOrange => AttackAnimation {
                    starting_x: -2.5,
                    ending_x: 0.3 + slot as f32 * 1.2,
                    max_weapon_rotation: -1.0,
                },
                Weapon::BasicSpear => AttackAnimation {
                    starting_x: -3.0,
                    ending_x: -0.5 + slot as f32 * 1.2,
                    max_weapon_rotation: -1.0,
                },
            }
        } else {
            AttackAnimation {
                starting_x: 3.0,
                ending_x: -1.9,
                max_weapon_rotation: 6.0 * PI,
            }
        };

        let attack = match self {
            Weapon::BasicStaffOrange => Attack {
                attacker,
                target,
                current_stage: 0,
                stages: vec![
                    (AttackStage::Charge, 0.2),
                    (AttackStage::WalkUp, 0.7),
                    (AttackStage::Action, 0.2),
                    (AttackStage::CoolDown, 0.2),
                ],
                action: Action {
                    stage: AttackStage::Action,
                    action_input: ActionTiming::NotEntered,
                },
                attack_type: self.attack_type(),
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
            Weapon::BasicSpear => Attack {
                attacker,
                target,
                current_stage: 0,
                stages: vec![
                    (AttackStage::WalkUp, 0.7),
                    (AttackStage::Action, 0.2),
                    (AttackStage::CoolDown, 0.5),
                ],
                action: Action {
                    stage: AttackStage::Action,
                    action_input: ActionTiming::NotEntered,
                },
                attack_type: self.attack_type(),
                timer: Timer::from_seconds(0.7, TimerMode::Once),
            },
        };

        AttackBundle { attack, animation }
    }
}
