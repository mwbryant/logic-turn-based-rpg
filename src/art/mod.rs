mod animation;
mod fade_in;
mod particles;
mod post_processing;
pub mod sprite_sheets;

use serde::{Deserialize, Serialize};
pub use sprite_sheets::*;

use crate::prelude::*;

pub use fade_in::spawn_fadeout;
pub use particles::create_new_rect_emitter;

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

use self::animation::AnimationPlugin;
use self::fade_in::FadeInPlugin;
use self::particles::ParticlePlugin;
use self::post_processing::PostProcessingPlugin;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetPlugin)
            .add_plugin(ParticlePlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(FadeInPlugin)
            .add_plugin(PostProcessingPlugin)
            .register_type::<Icon>()
            .register_type::<Particle>()
            .register_type::<FallingParticle>()
            .register_type::<FadingParticle>()
            .register_type::<RadialParticle>()
            .register_type::<Fadeout>()
            .register_type::<RotatingParticle>()
            .register_type::<Character>();
    }
}

#[derive(Reflect)]
pub enum FadeoutState {
    FadingIn,
    Hold,
    FadingOut,
}

#[derive(Component, Reflect)]
pub struct MainCamera;

#[derive(Resource)]
pub struct MainRender(pub Handle<Image>);

#[derive(Component, Reflect)]
pub struct Fadeout {
    pub fade_in_just_finished: bool,
    in_timer: Timer,
    hold_timer: Timer,
    out_timer: Timer,
    state: FadeoutState,
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

//Used for the weapon in the players hand during an attack animation
#[derive(Component)]
pub struct WeaponGraphic;

#[derive(Component)]
pub struct HandOffset {
    pub left: Vec2,
    pub right: Vec2,
}

pub const CHARACTER_SHEET_WIDTH: usize = 54;
pub const CHARACTER_SHEET_HEIGHT: usize = 12;
pub const ICON_SHEET_WIDTH: usize = 34;
pub const ICON_SHEET_HEIGHT: usize = 24;

#[derive(Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize)]
pub enum Icon {
    #[default]
    Pointer,
    KeyE,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize)]
pub enum BillboardSprite {
    #[default]
    None,
    Character(Character),
    Icon(Icon),
    Planet(Planet),
    Weapon(Weapon),
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize, FromReflect)]
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
#[derive(Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect, Serialize, Deserialize)]
pub enum Planet {
    #[default]
    Fireball,
}

#[derive(Resource)]
pub struct SpriteSheetMaps {
    character_atlas: Handle<Image>,
    icon_atlas: Handle<Image>,
    planet_atlas: Handle<Image>,
    weapon_atlas: Handle<Image>,
    pub sprites: HashMap<BillboardSprite, (usize, usize)>,
}

//TODO better way to do this
impl SpriteSheetMaps {
    pub fn get_atlas(&self, sprite: BillboardSprite) -> Handle<Image> {
        match sprite {
            BillboardSprite::Character(_) => self.character_atlas.clone(),
            BillboardSprite::Icon(_) => self.icon_atlas.clone(),
            BillboardSprite::Planet(_) => self.planet_atlas.clone(),
            BillboardSprite::Weapon(_) => self.weapon_atlas.clone(),
            BillboardSprite::None => todo!(),
        }
    }
}
