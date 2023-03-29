mod enemy;
mod player;
mod room;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use self::{enemy::EnemyPlugin, player::PlayerPlugin, room::RoomPlugin};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum OverworldState {
    #[default]
    FreeRoam,
    CombatStarting,
    NotInOverworld,
}

pub struct OverWorldPlugin;

impl Plugin for OverWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OverworldState>()
            .add_plugin(PlayerPlugin)
            .add_plugin(RoomPlugin)
            .add_plugin(EnemyPlugin);
    }
}

#[derive(Component, Deserialize)]
pub struct PlayerOverworld {
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct OverworldEntity;

#[derive(Component, Serialize, Deserialize)]
pub struct EnemyOverworld {
    pub movement_speed: f32,
    pub chase_movement_speed: f32,
    pub home: Vec2,
    pub direction: Vec2,
    new_direction_timer: Timer,
    pub wander_range: f32,
    pub follow_range: f32,
    // TODO
    pub combat_ref: String,
}

//For lookup in room
#[derive(Component, Serialize, Deserialize)]
pub struct EnemyId(pub usize);

#[derive(Component, Serialize, Deserialize)]
pub struct CombatDescriptor {
    enemies: Vec<(Enemy, CombatStats, Character)>,
}

#[derive(Resource)]
pub struct CurrentRoom {
    pub current_player_translation: Vec3,
    pub background_image: String,
    pub enemies: Vec<(usize, String, Vec3)>,
}
