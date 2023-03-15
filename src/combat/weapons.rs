use std::f32::consts::PI;

use bevy::sprite::Anchor;

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

    pub fn get_attack_bundle(&self, player: bool) -> AttackBundle {
        let animation = if player {
            match self {
                Weapon::BasicStaffOrange => AttackAnimation {
                    starting_x: -2.5,
                    ending_x: 2.7,
                    max_weapon_rotation: -1.0,
                },
                Weapon::BasicSpear => AttackAnimation {
                    starting_x: -3.0,
                    ending_x: 1.9,
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

impl Default for WeaponBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    anchor: Anchor::Custom(Vec2::new(-5.0 / 16.0, -0.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, WEAPON_Z)),
                ..Default::default()
            },
            weapon: Weapon::BasicSpear,
        }
    }
}

impl WeaponBundle {
    pub fn new(position: Vec2, weapon: Weapon, scale: Vec2) -> Self {
        let mut bundle = WeaponBundle {
            weapon,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(WEAPON_Z);
        bundle.sprite_sheet.transform.scale = scale.extend(1.0);

        bundle
    }
}
