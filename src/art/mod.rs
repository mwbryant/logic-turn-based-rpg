mod animation;
mod particles;
pub mod sprite_sheets;
pub use sprite_sheets::*;

use crate::prelude::*;

pub use particles::create_new_rect_emitter;

use self::animation::AnimationPlugin;
use self::particles::ParticlePlugin;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetPlugin)
            .add_plugin(ParticlePlugin)
            .add_plugin(AnimationPlugin)
            .register_type::<Icon>()
            .register_type::<Particle>()
            .register_type::<FallingParticle>()
            .register_type::<FadingParticle>()
            .register_type::<RadialParticle>()
            .register_type::<RotatingParticle>()
            .register_type::<Character>();
    }
}

#[derive(Component, Reflect)]
pub struct DeathAnimation;

#[derive(Component)]
pub struct RectParticleEmitter {
    pub particle_parent: Entity,
    pub size: Vec2,
    pub rate: Timer,
    pub varients: usize,
    //It would be nice to be able to give the emitter a tag to add to particles
    pub desc: ParticleDesc,
}

#[derive(Component, Default, Clone)]
pub struct ParticleDesc {
    pub particle: Particle,
    pub sprite: SpriteSheetBundle,
    pub falling: Option<FallingParticle>,
    pub radial: Option<RadialParticle>,
    pub rotating: Option<RotatingParticle>,
    pub fading: Option<FadingParticle>,
}

#[derive(Component, Reflect)]
pub struct ParticleParent;

#[derive(Component, Clone, Reflect)]
pub struct Particle {
    pub lifetime: Timer,
}

#[derive(Component, Clone, Reflect)]
pub struct FallingParticle {
    pub speed: f32,
}

#[derive(Component, Clone, Reflect)]
pub struct RadialParticle {
    pub speed: f32,
}

#[derive(Component, Clone, Reflect)]
pub struct RotatingParticle {
    pub speed: f32,
}

#[derive(Component, Clone, Reflect)]
pub struct FadingParticle {}

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
    hand_offset: HandOffset,
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

#[derive(Component)]
pub struct HandOffset {
    pub left: Vec2,
    pub right: Vec2,
}

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
