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
    mut characters: Query<(&Handle<StandardMaterial>, &Handle<Mesh>, &Character)>,
    /*
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
    */
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (material, mesh, character) in &mut characters {
        let mut material = materials.get_mut(material).unwrap();
        material.base_color_texture = Some(sprite_sheets.character_atlas.clone());
        material.alpha_mode = AlphaMode::Blend;
        let mesh = meshes.get_mut(mesh).unwrap();
        //FIXME gross
        let index = sprite_sheets.characters[character];
        let size = sprite_sheets.character_size;
        let u_left = index.0 as f32 / size.0 as f32;
        let u_right = index.0 as f32 / size.0 as f32 + 1.0 / size.0 as f32;
        let u_bottom = index.1 as f32 / size.1 as f32;
        let u_top = index.1 as f32 / size.1 as f32 + 1.0 / size.1 as f32;
        let uv = vec![
            [u_left, u_top],
            [u_left, u_bottom],
            [u_right, u_bottom],
            [u_right, u_top],
        ];
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    }
}

fn setup_spritesheet_maps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let character_handle =
        asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");

    let icon_handle = asset_server.load("input_icons/Tilemap/tilemap.png");

    let planet_handle = asset_server.load("planets/Planets/planet08.png");

    let characters = HashMap::from([
        (Character::WhiteBase, (0, 0)),
        (Character::WhiteBaseMouth, (1, 0)),
        (Character::TanBase, (0, 1)),
        (Character::TanBaseMouth, (1, 1)),
        (Character::BlackBase, (0, 2)),
        (Character::BlackBaseMouth, (1, 2)),
        (Character::GreenBase, (0, 3)),
        (Character::GreenBaseMouth, (1, 3)),
        (Character::WomanBraid, (0, 5)),
        (Character::WomanOld, (1, 5)),
        (Character::ManStrap, (0, 6)),
        (Character::ManBeard, (1, 6)),
        (Character::WomanBraidAlt, (0, 7)),
        (Character::WomanTwoBraid, (1, 7)),
        (Character::ManMohawk, (0, 8)),
        (Character::ManBaldish, (1, 8)),
        (Character::WomanRedhead, (0, 9)),
        (Character::WomanSoldier, (1, 9)),
        (Character::Jester, (0, 10)),
        (Character::ManOld, (1, 10)),
        (Character::KnightArmed, (0, 11)),
        (Character::Knight, (1, 11)),
    ]);

    let icons = HashMap::from([
        (Icon::Pointer, ICON_SHEET_WIDTH * 17),
        (Icon::KeyE, 19 + ICON_SHEET_WIDTH * 10),
    ]);

    let planets = HashMap::from([(Planet::Fireball, 0)]);

    let weapons = HashMap::from([(Weapon::BasicStaffOrange, 42), (Weapon::BasicSpear, 47)]);

    commands.insert_resource(SpriteSheetMaps {
        character_atlas: character_handle,
        icon_atlas: icon_handle,
        planet_atlas: planet_handle,
        character_size: (CHARACTER_SHEET_WIDTH, CHARACTER_SHEET_HEIGHT),
        characters,
        weapons,
        icons,
        planets,
    });
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            sprite: MaterialMeshBundle {
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
    pub fn new(
        position: Vec3,
        character: Character,
        //XXX I hate needing this here now...
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))));

        let material_handle = materials.add(StandardMaterial {
            double_sided: true,
            cull_mode: None,
            ..default()
        });

        let mut bundle = CharacterBundle {
            character,
            sprite: MaterialMeshBundle {
                mesh: quad_handle,
                material: material_handle,
                ..default()
            },
            ..default()
        };

        bundle.sprite.transform.translation = position;

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
