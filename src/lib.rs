#![allow(clippy::type_complexity)]
mod art;
mod combat;
mod utils;
mod weapons;

pub mod prelude {
    pub use crate::art::*;
    pub use crate::combat::*;
    pub use crate::utils::*;
    pub use crate::weapons::*;
    pub use bevy::{prelude::*, utils::HashMap};

    pub const CHARACTER_Z: f32 = 100.0;
    pub const BACKGROUND_Z: f32 = 10.0;
    pub const WEAPON_Z: f32 = 150.0;
    pub const ICON_Z: f32 = 850.0;
}
