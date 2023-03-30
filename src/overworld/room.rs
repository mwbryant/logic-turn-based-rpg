use crate::{comp_from_config, prelude::*};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_starting_room)
            .add_system(spawn_room.in_schedule(OnEnter(GameState::Overworld)))
            .add_system(update_player_translation_in_room.in_set(OnUpdate(GameState::Overworld)));
    }
}

fn load_starting_room(mut commands: Commands) {
    //TODO pull from room file
    let enemies = ["config/basic_enemy.ron", "config/basic_enemy2.ron"];
    let mut enemy_setup = Vec::new();

    for (id, config) in enemies.iter().enumerate() {
        //FIXME windows uses \ .. fix in macro
        let enemy = comp_from_config!(EnemyOverworld, config);
        enemy_setup.push((id, config.to_string(), enemy.home.extend(ENEMY_Z)));
    }

    let room = CurrentRoom {
        current_player_translation: Vec3::new(0.0, 0.0, CHARACTER_Z),
        background_image: "Background_Mockup.png".to_string(),
        enemies: enemy_setup,
    };
    commands.insert_resource(room);
}

fn update_player_translation_in_room(
    mut room: ResMut<CurrentRoom>,
    player: Query<&Transform, With<PlayerOverworld>>,
) {
    let player = player.single();
    room.current_player_translation = player.translation;
}

fn spawn_room(
    mut commands: Commands,
    assets: Res<AssetServer>,
    room: Res<CurrentRoom>,
    mut player: Query<&mut Transform, With<PlayerOverworld>>,
) {
    for (id, config, position) in room.enemies.iter() {
        let enemy = comp_from_config!(EnemyOverworld, config);
        let mut character = CharacterBundle::new(*position, Character::GreenBase);
        character.sprite_sheet.transform.translation.z = ENEMY_Z;

        commands.spawn((enemy, character, Name::new("Enemy"), EnemyId(*id)));
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(48.0, 27.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
            texture: assets.load(&room.background_image),
            ..default()
        },
        Name::new("Background"),
        OverworldEntity,
    ));

    let mut player = player.single_mut();
    player.translation = room.current_player_translation;
}
