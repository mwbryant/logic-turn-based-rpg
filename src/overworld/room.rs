use crate::prelude::*;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_starting_room)
            .add_system(spawn_starting_room.in_schedule(OnEnter(GameState::Overworld)))
            //.add_system(spawn_room.in_schedule(OnEnter(GameState::Overworld)))
            .add_system(update_player_translation_in_room.in_set(OnUpdate(GameState::Overworld)));
    }
}

#[derive(Resource)]
pub struct RoomDescriptor {
    enemies: Vec<Handle<EnemyOverworld>>,
}

fn spawn_starting_room(
    mut commands: Commands,
    assets: Res<AssetServer>,
    room: Res<RoomDescriptor>,
    enemies: Res<Assets<EnemyOverworld>>,
) {
    let mut enemy_setup = Vec::new();

    for (id, config) in room.enemies.iter().enumerate() {
        if let Some(enemy) = enemies.get(config) {
            enemy_setup.push((id, enemy.clone(), enemy.home.extend(ENEMY_Z)));
        } else {
            // give up and try next frame if any enemy isn't loaded
            info!("Room failed!");
            return;
        }
    }

    let room = CurrentRoom {
        current_player_translation: Vec3::new(0.0, 0.0, CHARACTER_Z),
        background_image: "Background_Mockup.png".to_string(),
        enemies: enemy_setup,
    };

    commands.insert_resource(room);
    info!("Room loaded!");
}

fn load_starting_room(mut commands: Commands, assets: Res<AssetServer>) {
    //TODO pull from room file
    let files = [
        "config/basic_enemy.enemy.ron",
        "config/basic_enemy2.enemy.ron",
    ];

    let mut enemies = Vec::new();
    for (_id, config) in files.iter().enumerate() {
        let enemy: Handle<EnemyOverworld> = assets.load(*config);
        enemies.push(enemy);
    }

    commands.insert_resource(RoomDescriptor { enemies });
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
    for (id, enemy, position) in room.enemies.iter() {
        //let enemy: Handle<CombatDescriptor> = assets.load(config);
        let mut character = CharacterBundle::new(*position, Character::GreenBase);
        character.sprite_sheet.transform.translation.z = ENEMY_Z;

        commands.spawn((enemy.clone(), character, Name::new("Enemy"), EnemyId(*id)));
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
