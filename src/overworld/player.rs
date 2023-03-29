use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (player_movement, camera_follow)
                .chain()
                .in_set(OnUpdate(GameState::Overworld))
                .in_set(OnUpdate(OverworldState::FreeRoam)),
        );
    }
}

fn camera_follow(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();
    camera.translation = player.translation.truncate().extend(999.0);
}

fn player_movement(
    mut player: Query<(&mut Transform, &PlayerOverworld)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, player) = player.get_single_mut().expect("1 Player");

    if input.pressed(KeyCode::W) {
        transform.translation.y += player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += player.movement_speed * time.delta_seconds();
    }
}
