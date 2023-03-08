mod art;

pub mod prelude {
    pub use crate::art::*;
    pub use bevy::{prelude::*, utils::HashMap};

    pub const CHARACTER_Z: f32 = 100.0;
}
