#![allow(clippy::type_complexity)]
mod art;
mod combat;
mod overworld;
mod utils;

pub mod prelude {
    pub use crate::art::*;
    pub use crate::combat::*;
    pub use crate::overworld::*;
    pub use crate::utils::*;
    pub use bevy::{prelude::*, utils::HashMap};

    pub const BACKGROUND_Z: f32 = 10.0;
    pub const ENEMY_Z: f32 = 90.0;
    pub const NPC_Z: f32 = 95.0;
    pub const CHARACTER_Z: f32 = 100.0;
    pub const WEAPON_Z: f32 = 150.0;
    pub const PARTICLE_Z: f32 = 750.0;
    pub const ICON_Z: f32 = 850.0;
    pub const WORLD_UI_Z: f32 = 999.0;

    #[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
    pub enum GameState {
        #[default]
        Overworld,
        Combat,
    }
}
