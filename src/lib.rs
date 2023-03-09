mod art;
mod combat;
mod weapons;

pub mod prelude {
    pub use crate::art::*;
    pub use crate::combat::*;
    pub use crate::weapons::*;
    pub use bevy::{prelude::*, utils::HashMap};

    pub const CHARACTER_Z: f32 = 100.0;
    pub const WEAPON_Z: f32 = 150.0;
    pub const ICON_Z: f32 = 850.0;
}
