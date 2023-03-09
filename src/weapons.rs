use bevy::sprite::Anchor;

use crate::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>();
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Weapon {
    #[default]
    BasicStaffOrange,
    BasicSpear,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    weapon: Weapon,
}

#[derive(PartialEq, Eq)]
pub enum WeaponAttackType {
    Melee,
    Range,
}

impl Weapon {
    pub fn attack_type(&self) -> WeaponAttackType {
        match self {
            Weapon::BasicSpear => WeaponAttackType::Melee,
            Weapon::BasicStaffOrange => WeaponAttackType::Range,
        }
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
