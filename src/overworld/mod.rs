mod player;

use crate::prelude::*;

use self::player::PlayerPlugin;

pub struct OverWorldPlugin;

impl Plugin for OverWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin);
    }
}

#[derive(Component)]
pub struct PlayerOverworld {
    pub movement_speed: f32,
}
