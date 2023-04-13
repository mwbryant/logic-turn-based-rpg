use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (player_movement, camera_follow)
                .chain()
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
    mut player: Query<(&PlayerOverworld, &mut KinematicCharacterController)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut controller) = player.get_single_mut().expect("1 Player");
    let mut target_movement = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        target_movement.y += player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        target_movement.y -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        target_movement.x -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        target_movement.x += player.movement_speed * time.delta_seconds();
    }
    controller.translation = Some(target_movement);
}
