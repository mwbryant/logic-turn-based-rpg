use crate::prelude::*;

pub const CHARACTER_SHEET_WIDTH: usize = 54;

#[derive(Clone, PartialEq, Eq, Hash)]
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
    pub characters: HashMap<Character, usize>,
}

pub fn setup_spritesheet_maps(mut commands: Commands) {
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
    commands.insert_resource(SpriteSheetMaps { characters });
}
