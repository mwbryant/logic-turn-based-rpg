use crate::prelude::*;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_starting_room)
            .add_system(check_if_room_loaded.in_set(OnUpdate(OverworldState::LoadingRoom)))
            .add_system(spawn_current_room.in_set(OnUpdate(OverworldState::RestoreRoom)))
            .add_system(
                update_player_translation_in_room
                    .in_set(OnUpdate(GameState::Overworld))
                    .in_set(OnUpdate(OverworldState::FreeRoam)),
            );
    }
}

fn check_if_room_loaded(
    mut commands: Commands,
    assets: Res<AssetServer>,
    room: Query<(Entity, &Handle<RoomDescriptor>)>,
    rooms: Res<Assets<RoomDescriptor>>,
    enemy: Res<Assets<EnemyOverworld>>,
    mut next_state: ResMut<NextState<OverworldState>>,
) {
    let (entity, room) = room.single();
    if let Some(room) = rooms.get(room) {
        let mut enemies = Vec::new();
        for (id, config) in room.enemies.iter().enumerate() {
            //Try loading, each frame this call should return the same handle
            let handle = assets.load(config);
            if let Some(enemy) = enemy.get(&handle) {
                enemies.push((id, enemy.clone(), enemy.home.extend(ENEMY_Z)));
            } else {
                info!("Waiting on enemy load");
                return;
            }
        }

        let room = CurrentRoom {
            current_player_translation: Vec3::new(0.0, 0.0, CHARACTER_Z),
            background_image: "Background_Mockup.png".to_string(),
            enemies,
        };

        commands.insert_resource(room);
        commands.entity(entity).despawn();

        info!("Room loaded!");
        next_state.set(OverworldState::RestoreRoom);
    } else {
        info!("Waiting on room load");
    }
}

fn load_starting_room(mut commands: Commands, assets: Res<AssetServer>) {
    //TODO pull from room file
    //let files = [
    //"config/basic_enemy.enemy.ron",
    //"config/basic_enemy2.enemy.ron",
    //];
    //ron::from_str::<RoomDescriptor>(&include_str!("../../assets/config/sample_room.room.ron"))
    //.expect("Failled to load");

    commands.spawn(assets.load::<RoomDescriptor, _>("config/sample_room.room.ron"));
}

fn update_player_translation_in_room(
    mut room: ResMut<CurrentRoom>,
    player: Query<&Transform, With<PlayerOverworld>>,
) {
    let player = player.single();
    room.current_player_translation = player.translation;
}

fn spawn_current_room(
    mut commands: Commands,
    assets: Res<AssetServer>,
    room: Res<CurrentRoom>,
    mut player: Query<&mut Transform, With<PlayerOverworld>>,
    mut next_state: ResMut<NextState<OverworldState>>,
) {
    for (id, enemy, position) in room.enemies.iter() {
        ron::from_str::<CombatDescriptor>(
            &std::fs::read_to_string("assets/".to_owned() + &enemy.combat_ref.to_owned()).unwrap(),
        )
        .expect("Failled to load");
        let descriptor: Handle<CombatDescriptor> = assets.load(&enemy.combat_ref);
        let mut character = CharacterBundle::new(*position, Character::GreenBase);
        character.sprite_sheet.transform.translation.z = ENEMY_Z;

        commands.spawn((
            enemy.clone(),
            descriptor,
            character,
            Name::new("Enemy"),
            EnemyId(*id),
        ));
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

    next_state.set(OverworldState::FreeRoam);
}
