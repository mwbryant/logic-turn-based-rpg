pub mod animation;
pub mod attack;
pub mod selection;
pub mod turn_based;
pub mod weapons;

use crate::prelude::*;

use self::{
    animation::CombatAnimationPlugin, attack::AttackPlugin, selection::SelectionPlugin,
    turn_based::TurnBasedPlugin, weapons::WeaponPlugin,
};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum CombatState {
    #[default]
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
    PlayerWins,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CombatState>()
            .add_event::<HitEvent>()
            .add_plugin(TurnBasedPlugin)
            .add_plugin(AttackPlugin)
            .add_plugin(SelectionPlugin)
            .add_plugin(CombatAnimationPlugin)
            .add_plugin(WeaponPlugin)
            .register_type::<CombatStats>()
            .register_type::<Player>()
            .register_type::<CurrentSelectedMenuItem>()
            .register_type::<SelectionIcon>()
            .register_type::<PlayerAttack>()
            .register_type::<EnemyAttack>()
            .register_type::<WeaponIcon>()
            .register_type::<Attack>()
            .register_type::<Weapon>()
            .register_type::<AttackAnimation>()
            .register_type::<Projectile>()
            .register_type::<Enemy>();
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

#[derive(Bundle)]
pub struct AttackBundle {
    attack: Attack,
    animation: AttackAnimation,
}

#[derive(PartialEq, Eq, Reflect)]
pub enum WeaponAttackType {
    Melee,
    Range,
}

#[derive(Component, Reflect)]
pub struct AttackAnimation {
    pub starting_x: f32,
    pub ending_x: f32,
    pub max_weapon_rotation: f32,
}

#[derive(Component, Reflect)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Default, Reflect)]
pub struct Player {
    pub experience: i32,
    pub level: i32,
}

#[derive(Component, Default, Reflect)]
pub struct CurrentSelectedMenuItem {
    pub selection: i32,
    pub slots: i32,
}

#[derive(Component, Reflect)]
pub struct SelectionIcon;

#[derive(Component, Reflect)]
pub struct PlayerAttack;

#[derive(Component, Reflect)]
pub struct EnemyAttack;

#[derive(Component, Reflect)]
pub struct WeaponIcon(pub i32);

#[derive(Component, Reflect)]
pub struct Enemy {
    pub base_experience_reward: i32,
}

#[derive(Component, Reflect)]
pub struct Projectile;

#[derive(Component, Reflect)]
pub struct Attack {
    pub attack_type: WeaponAttackType,
    pub stage: AttackStages,
    pub action_input: ActionTiming,
    //TODO only use 1 timer, should be more redundant to errors
    pub warmup_timer: Timer,
    pub action_timer: Timer,
    pub cool_down_timer: Timer,
}

pub struct HitEvent {
    action: ActionTiming,
    combat_state: CombatState,
}

#[derive(Reflect)]
pub enum AttackStages {
    Warmup,
    Action,
    CoolDown,
}

#[derive(Reflect, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ActionTiming {
    NotEntered,
    Early,
    Critical,
    Late,
}
