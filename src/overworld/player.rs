use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            player_movement
                .in_set(OnUpdate(GameState::Overworld))
                .in_set(OnUpdate(OverworldState::FreeRoam)),
        );
    }
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
