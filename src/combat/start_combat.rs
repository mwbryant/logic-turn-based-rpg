use crate::prelude::*;

pub struct StartCombatPlugin;

impl Plugin for StartCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_combat.in_schedule(OnEnter(GameState::Combat)));
    }
}

fn setup_combat(
    mut player: Query<&mut Transform, With<PlayerCombat>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerCombat>)>,
) {
    let mut player = player.single_mut();
    player.translation = Vec3::new(-3.0, 0.0, CHARACTER_Z);
    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(0.0, 0.0, 999.0);
}
