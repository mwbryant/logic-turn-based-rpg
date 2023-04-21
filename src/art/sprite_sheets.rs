use bevy::sprite::Anchor;

use crate::prelude::*;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
            .add_system(setup_sprite_graphics)
            .add_system(update_art);
    }
}

fn update_art(
    mut sprites: Query<(&Handle<StandardMaterial>, &Handle<Mesh>, &BillboardSprite)>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (material, mesh, character) in &mut sprites {
        let mesh = meshes.get_mut(mesh).unwrap();

        let index = sprite_sheets.sprites[character];
        let size = (CHARACTER_SHEET_WIDTH, CHARACTER_SHEET_HEIGHT);

        let uvs = get_uvs(index, size);

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}

fn get_uvs(index: (usize, usize), size: (usize, usize)) -> Vec<[f32; 2]> {
    let u_left = index.0 as f32 / size.0 as f32;
    let u_right = u_left + 1.0 / size.0 as f32;
    let u_bottom = index.1 as f32 / size.1 as f32;
    let u_top = u_bottom + 1.0 / size.1 as f32;

    vec![
        [u_left, u_top],
        [u_left, u_bottom],
        [u_right, u_bottom],
        [u_right, u_top],
    ]
}

fn setup_spritesheet_maps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let character_handle =
        asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");

    let icon_handle = asset_server.load("input_icons/Tilemap/tilemap.png");

    let planet_handle = asset_server.load("planets/Planets/planet08.png");

    let weapon_handle = asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");

    let mut sprites = HashMap::from([
        (BillboardSprite::Character(Character::WhiteBase), (0, 0)),
        (
            BillboardSprite::Character(Character::WhiteBaseMouth),
            (1, 0),
        ),
        (BillboardSprite::Character(Character::TanBase), (0, 1)),
        (BillboardSprite::Character(Character::TanBaseMouth), (1, 1)),
        (BillboardSprite::Character(Character::BlackBase), (0, 2)),
        (
            BillboardSprite::Character(Character::BlackBaseMouth),
            (1, 2),
        ),
        (BillboardSprite::Character(Character::GreenBase), (0, 3)),
        (
            BillboardSprite::Character(Character::GreenBaseMouth),
            (1, 3),
        ),
        (BillboardSprite::Character(Character::WomanBraid), (0, 5)),
        (BillboardSprite::Character(Character::WomanOld), (1, 5)),
        (BillboardSprite::Character(Character::ManStrap), (0, 6)),
        (BillboardSprite::Character(Character::ManBeard), (1, 6)),
        (BillboardSprite::Character(Character::WomanBraidAlt), (0, 7)),
        (BillboardSprite::Character(Character::WomanTwoBraid), (1, 7)),
        (BillboardSprite::Character(Character::ManMohawk), (0, 8)),
        (BillboardSprite::Character(Character::ManBaldish), (1, 8)),
        (BillboardSprite::Character(Character::WomanRedhead), (0, 9)),
        (BillboardSprite::Character(Character::WomanSoldier), (1, 9)),
        (BillboardSprite::Character(Character::Jester), (0, 10)),
        (BillboardSprite::Character(Character::ManOld), (1, 10)),
        (BillboardSprite::Character(Character::KnightArmed), (0, 11)),
        (BillboardSprite::Character(Character::Knight), (1, 11)),
    ]);

    sprites.insert(BillboardSprite::Icon(Icon::Pointer), (0, 17));
    sprites.insert(BillboardSprite::Icon(Icon::KeyE), (10, 19));

    sprites.insert(BillboardSprite::Planet(Planet::Fireball), (0, 0));

    sprites.extend(HashMap::from([
        (BillboardSprite::Weapon(Weapon::BasicStaffOrange), (42, 0)),
        (BillboardSprite::Weapon(Weapon::BasicSpear), (47, 0)),
    ]));

    commands.insert_resource(SpriteSheetMaps {
        character_atlas: character_handle,
        icon_atlas: icon_handle,
        weapon_atlas: weapon_handle,
        planet_atlas: planet_handle,
        sprites,
    });
}

fn setup_sprite_graphics(
    mut commands: Commands,
    characters: Query<(Entity, &Transform, &BillboardSprite), Without<Handle<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (character, transform, sprite) in &characters {
        let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))));

        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(sprite_sheets.character_atlas.clone()),
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            cull_mode: None,
            ..default()
        });

        commands.entity(character).insert(MaterialMeshBundle {
            mesh: quad_handle,
            material: material_handle,
            ///XXX do I really need this
            transform: transform.clone(),
            ..default()
        });
    }
}
