mod enemy;
mod map;
mod npc;
mod player;
mod room;
mod start_combat;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use self::{
    enemy::EnemyPlugin, map::MapPlugin, npc::NpcPlugin, player::PlayerPlugin, room::RoomPlugin,
    start_combat::CombatTransitionPlugin,
};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum OverworldState {
    #[default]
    FreeRoam,
    CombatStarting,
    Dialog,
    NotInOverworld,
}

pub struct OverWorldPlugin;

impl Plugin for OverWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OverworldState>()
            .add_plugin(RonAssetPlugin::<EnemyOverworld>::new(&["enemy.ron"]))
            .add_plugin(RonAssetPlugin::<PlayerOverworld>::new(&["player.ron"]))
            .add_plugin(PlayerPlugin)
            .add_plugin(NpcPlugin)
            .add_plugin(CombatTransitionPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(RoomPlugin)
            .add_plugin(EnemyPlugin);
    }
}

#[derive(Component)]
pub struct Npc(pub usize);

#[derive(Component, Deserialize, TypeUuid)]
#[uuid = "b43f4b0e-29de-4069-b462-41f9ed63d845"]
pub struct PlayerOverworld {
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct OverworldEntity;

#[derive(Component)]
pub struct CombatFadeout;

#[derive(Component)]
pub struct InteractIcon;

#[derive(Component, Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "530989f8-3a50-4e51-927f-f5cd3f4a24d0"]
pub struct EnemyOverworld {
    pub movement_speed: f32,
    pub chase_movement_speed: f32,
    pub home: Vec2,
    pub direction: Vec2,
    new_direction_timer: Timer,
    pub wander_range: f32,
    pub follow_range: f32,
    pub combat_ref: String,
}

//For lookup in room
#[derive(Component, Serialize, Deserialize)]
pub struct EnemyId(pub usize);

#[derive(Component, Serialize, Deserialize, TypeUuid)]
#[uuid = "9d0b9466-8797-486e-a930-bcb696f8e2f3"]
pub struct CombatDescriptor {
    enemies: Vec<(Enemy, CombatStats, Character)>,
}

#[derive(Resource)]
pub struct CurrentRoom {
    pub current_player_translation: Vec3,
    pub background_image: String,
    pub enemies: Vec<(usize, EnemyOverworld, Vec3)>,
    //pub npcs: Vec<(String, Vec3)>,
}
