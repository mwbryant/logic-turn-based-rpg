use logic_bevy_new_scheduling::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_map: Res<SpriteSheetMaps>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        54,
        12,
        Some(Vec2::splat(1.0)),
        None,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(sprite_map.characters[&Character::Knight]),
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    });
}
