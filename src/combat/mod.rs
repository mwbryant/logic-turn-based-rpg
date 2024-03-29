pub mod animation;
pub mod attack;
pub mod graphic_effects;
pub mod player_wins;
pub mod selection;
pub mod start_combat;
pub mod turn_based;
pub mod ui;
pub mod weapons;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use self::{
    animation::CombatAnimationPlugin, attack::AttackPlugin, graphic_effects::GraphicEffectsPlugin,
    player_wins::PlayerWinsPlugin, selection::SelectionPlugin, start_combat::StartCombatPlugin,
    turn_based::TurnBasedPlugin, ui::CombatUIPlugin, weapons::WeaponPlugin,
};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum CombatState {
    #[default]
    NotInCombat,
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
    EnemyDying,
    PlayerWins,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CombatSet {
    Logic,
    Animation,
    CleanUp,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CombatState>()
            .add_plugin(RonAssetPlugin::<CombatDescriptor>::new(&["combat.ron"]))
            .add_event::<HitEvent>()
            .add_event::<DeathEvent>()
            .add_plugin(TurnBasedPlugin)
            .add_plugin(AttackPlugin)
            .add_plugin(SelectionPlugin)
            .add_plugin(CombatAnimationPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(GraphicEffectsPlugin)
            .add_plugin(CombatUIPlugin)
            .add_plugin(PlayerWinsPlugin)
            .add_plugin(StartCombatPlugin)
            .configure_set(CombatSet::Logic.before(CombatSet::Animation))
            .configure_set(CombatSet::CleanUp.after(CombatSet::Animation))
            .register_type::<CombatStats>()
            .register_type::<PlayerCombat>()
            .register_type::<CurrentSelectedMenuItem>()
            .register_type::<SelectionIcon>()
            .register_type::<PlayerAttack>()
            .register_type::<EnemyAttack>()
            .register_type::<WeaponIcon>()
            .register_type::<Weapon>()
            .register_type::<VictoryParticle>()
            .register_type::<AttackAnimation>()
            .register_type::<Projectile>()
            .register_type::<Enemy>();
    }
}

#[derive(Component)]
pub struct CombatEntity;

#[derive(Component)]
pub struct VictoryFadeout;

//XXX where does weapon declaration belong
#[derive(
    Component, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
pub enum Weapon {
    #[default]
    BasicStaffOrange,
    BasicSpear,
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

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Copy)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Default, Reflect)]
pub struct PlayerCombat {
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
pub struct VictoryParticle;

#[derive(Component, Reflect)]
pub struct PlayerAttack {
    pub target: Entity,
    pub slot: usize,
}

#[derive(Component, Reflect)]
pub struct EnemyAttack;

#[derive(Component, Reflect)]
pub struct WeaponIcon(pub i32);

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Copy)]
pub struct Enemy {
    pub slot: usize,
    pub base_experience_reward: i32,
}

#[derive(Component, Reflect)]
pub struct Projectile;

#[derive(Component, Reflect)]
pub struct ProjectileParticleEmitter {
    pub projectile: Entity,
}

#[derive(Component)]
pub struct Attack {
    pub attacker: Entity,
    pub target: Entity,
    pub attack_type: WeaponAttackType,
    pub current_stage: usize,
    pub stages: Vec<(AttackStage, f32)>,
    pub action: Action,
    pub timer: Timer,
}

pub struct Action {
    //TODO action type
    pub stage: AttackStage,
    pub action_input: ActionTiming,
}

pub struct HitEvent {
    target: Entity,
    attacker: Entity,
    player_attacking: bool,
    action: ActionTiming,
}

pub struct DeathEvent {
    pub entity: Entity,
}

#[derive(Reflect, PartialEq, Eq)]
pub enum AttackStage {
    Charge,
    WalkUp,
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
