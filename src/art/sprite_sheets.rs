use bevy::sprite::Anchor;

use crate::prelude::*;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
            .add_system(update_art);
    }
}

fn update_art(
    mut characters: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &Character,
    )>,
    mut weapons: Query<
        (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Weapon),
        Without<Character>,
    >,
    mut icons: Query<
        (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Icon),
        (Without<Character>, Without<Weapon>),
    >,
    mut planets: Query<
        (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Planet),
        (Without<Character>, Without<Weapon>, Without<Icon>),
    >,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (mut sprite, mut atlas, character) in &mut characters {
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.characters[character];
    }
    for (mut sprite, mut atlas, weapon) in &mut weapons {
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.weapons[weapon];
    }
    for (mut sprite, mut atlas, icon) in &mut icons {
        *atlas = sprite_sheets.icon_atlas.clone();
        sprite.index = sprite_sheets.icons[icon];
    }
    for (mut sprite, mut atlas, planet) in &mut planets {
        *atlas = sprite_sheets.planet_atlas.clone();
        sprite.index = sprite_sheets.planets[planet];
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
        CHARACTER_SHEET_WIDTH,
        12,
        Some(Vec2::splat(1.0)),
        None,
    );
    let character_atlas = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("input_icons/Tilemap/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        ICON_SHEET_WIDTH,
        24,
        Some(Vec2::splat(1.0)),
        None,
    );
    let icon_atlas = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("planets/Planets/planet08.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(1280.0, 1280.0),
        1,
        1,
        Some(Vec2::splat(0.0)),
        None,
    );
    let planet_atlas = texture_atlases.add(texture_atlas);

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

    let icons = HashMap::from([(Icon::Pointer, ICON_SHEET_WIDTH * 17)]);

    let planets = HashMap::from([(Planet::Fireball, 0)]);

    let weapons = HashMap::from([(Weapon::BasicStaffOrange, 42), (Weapon::BasicSpear, 47)]);

    commands.insert_resource(SpriteSheetMaps {
        character_atlas,
        icon_atlas,
        planet_atlas,
        characters,
        weapons,
        icons,
        planets,
    });
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, CHARACTER_Z)),
                ..Default::default()
            },
            // Calling ..default here causes a stack overflow........
            // Using ..Default::default() gives a proper warning though....
            // https://github.com/bevyengine/bevy/issues/6207
            character: Character::WhiteBase,
            hand_offset: HandOffset {
                left: Vec2::new(-0.40, -0.37),
                right: Vec2::new(0.35, -0.37),
            },
        }
    }
}

impl PlanetBundle {
    pub fn new(position: Vec2, planet: Planet) -> Self {
        let mut bundle = PlanetBundle {
            planet,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(WEAPON_Z);

        bundle
    }
}

impl Default for PlanetBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, WEAPON_Z)),
                ..Default::default()
            },
            planet: Planet::Fireball,
        }
    }
}

impl CharacterBundle {
    pub fn new(position: Vec2, character: Character) -> Self {
        let mut bundle = CharacterBundle {
            character,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(CHARACTER_Z);

        bundle
    }
}

impl Default for IconBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, ICON_Z)),
                ..Default::default()
            },
            icon: Icon::Pointer,
        }
    }
}

impl IconBundle {
    pub fn new(position: Vec2, icon: Icon, scale: Vec2) -> Self {
        let mut bundle = IconBundle { icon, ..default() };

        bundle.sprite_sheet.transform.translation = position.extend(ICON_Z);
        bundle.sprite_sheet.transform.scale = scale.extend(1.0);

        bundle
    }
}

impl Default for WeaponBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    anchor: Anchor::Custom(Vec2::new(-5.0 / 16.0, -0.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, WEAPON_Z)),
                ..Default::default()
            },
            weapon: Weapon::BasicSpear,
        }
    }
}

impl WeaponBundle {
    pub fn new(position: Vec2, weapon: Weapon, scale: Vec2) -> Self {
        let mut bundle = WeaponBundle {
            weapon,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(WEAPON_Z);
        bundle.sprite_sheet.transform.scale = scale.extend(1.0);

        bundle
    }
}
