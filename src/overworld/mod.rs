mod enemy;
mod player;

use serde::Deserialize;

use crate::prelude::*;

use self::{enemy::EnemyPlugin, player::PlayerPlugin};

pub struct OverWorldPlugin;

impl Plugin for OverWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin).add_plugin(EnemyPlugin);
    }
}

#[derive(Component, Deserialize)]
pub struct PlayerOverworld {
    pub movement_speed: f32,
}

#[derive(Component)]
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
