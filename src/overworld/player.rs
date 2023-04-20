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
    mut camera: Query<&mut Transform, With<MainCamera>>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<MainCamera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();
    camera.translation.x = player.translation.x;
    camera.translation.z = player.translation.z + 10.0;
    camera.translation.y = 5.0;
    camera.look_at(player.translation, Vec3::Y);
    camera.rotation.z = 0.0;
}

fn player_movement(
    mut player: Query<(&PlayerOverworld, &mut KinematicCharacterController)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut controller) = player.get_single_mut().expect("1 Player");
    let mut target_movement = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        target_movement.z -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        target_movement.z += player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        target_movement.x -= player.movement_speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        target_movement.x += player.movement_speed * time.delta_seconds();
    }
    controller.translation = Some(target_movement);
}
