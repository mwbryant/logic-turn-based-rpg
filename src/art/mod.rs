pub mod sprite_sheets;
pub use sprite_sheets::*;

use crate::prelude::*;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetPlugin)
            .register_type::<Icon>()
            .register_type::<Character>();
    }
}

#[derive(Bundle)]
pub struct WeaponBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    weapon: Weapon,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    character: Character,
}

#[derive(Bundle)]
pub struct IconBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    icon: Icon,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    planet: Planet,
}

//Used for the weapon in the players hand during an attack animation
#[derive(Component)]
pub struct WeaponGraphic;

pub const CHARACTER_SHEET_WIDTH: usize = 54;
pub const ICON_SHEET_WIDTH: usize = 34;

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Icon {
    #[default]
    Pointer,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Character {
    #[default]
    WhiteBase,
    WhiteBaseMouth,
    TanBase,
    TanBaseMouth,
    BlackBase,
    BlackBaseMouth,
    GreenBase,
    GreenBaseMouth,
    WomanBraid,
    WomanOld,
    ManStrap,
    ManBeard,
    WomanBraidAlt,
    WomanTwoBraid,
    ManMohawk,
    ManBaldish,
    WomanRedhead,
    WomanSoldier,
    Jester,
    ManOld,
    KnightArmed,
    Knight,
}

// I use the planet sheet for magic projectiles
#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Planet {
    #[default]
    Fireball,
}

#[derive(Resource)]
pub struct SpriteSheetMaps {
    character_atlas: Handle<TextureAtlas>,
    icon_atlas: Handle<TextureAtlas>,
    planet_atlas: Handle<TextureAtlas>,
    pub characters: HashMap<Character, usize>,
    pub weapons: HashMap<Weapon, usize>,
    pub icons: HashMap<Icon, usize>,
    pub planets: HashMap<Planet, usize>,
}
