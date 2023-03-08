use crate::prelude::*;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
            .add_system(update_art);
    }
}

pub const CHARACTER_SHEET_WIDTH: usize = 54;

#[derive(Component, Clone, PartialEq, Eq, Hash)]
pub enum Character {
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

#[derive(Resource)]
pub struct SpriteSheetMaps {
    character_atlas: Handle<TextureAtlas>,
    pub characters: HashMap<Character, usize>,
}

fn update_art(
    mut sprites: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &Character,
    )>,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (mut sprite, mut atlas, character) in &mut sprites {
        //TODO animation?
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.characters[character];
    }
}

fn setup_spritesheet_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        54,
        12,
        Some(Vec2::splat(1.0)),
        None,
    );
    let character_atlas = texture_atlases.add(texture_atlas);

    let characters = HashMap::from([
        (Character::WhiteBase, 0),
        (Character::WhiteBaseMouth, 1),
        (Character::TanBase, CHARACTER_SHEET_WIDTH),
        (Character::TanBaseMouth, CHARACTER_SHEET_WIDTH + 1),
        (Character::BlackBase, CHARACTER_SHEET_WIDTH * 2),
        (Character::BlackBaseMouth, CHARACTER_SHEET_WIDTH * 2 + 1),
        (Character::GreenBase, CHARACTER_SHEET_WIDTH * 3),
        (Character::GreenBaseMouth, CHARACTER_SHEET_WIDTH * 3 + 1),
        (Character::WomanBraid, CHARACTER_SHEET_WIDTH * 5),
        (Character::WomanOld, CHARACTER_SHEET_WIDTH * 5 + 1),
        (Character::ManStrap, CHARACTER_SHEET_WIDTH * 6),
        (Character::ManBeard, CHARACTER_SHEET_WIDTH * 6 + 1),
        (Character::WomanBraidAlt, CHARACTER_SHEET_WIDTH * 7),
        (Character::WomanTwoBraid, CHARACTER_SHEET_WIDTH * 7 + 1),
        (Character::ManMohawk, CHARACTER_SHEET_WIDTH * 8),
        (Character::ManBaldish, CHARACTER_SHEET_WIDTH * 8 + 1),
        (Character::WomanRedhead, CHARACTER_SHEET_WIDTH * 9),
        (Character::WomanSoldier, CHARACTER_SHEET_WIDTH * 9 + 1),
        (Character::Jester, CHARACTER_SHEET_WIDTH * 10),
        (Character::ManOld, CHARACTER_SHEET_WIDTH * 10 + 1),
        (Character::KnightArmed, CHARACTER_SHEET_WIDTH * 11),
        (Character::Knight, CHARACTER_SHEET_WIDTH * 11 + 1),
    ]);
    info!("Adding sprites");
    commands.insert_resource(SpriteSheetMaps {
        character_atlas,
        characters,
    });
}
