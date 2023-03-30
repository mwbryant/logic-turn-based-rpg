use crate::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_npc)
            .add_system(player_interaction);
    }
}

#[derive(Component)]
pub struct Npc;

fn spawn_test_npc(mut commands: Commands) {
    commands.spawn((
        CharacterBundle::new(Vec3::new(-3.0, 3.0, NPC_Z), Character::WomanOld),
        Npc,
        Name::new("TestNPC"),
    ));
}

fn player_interaction(
    npcs: Query<&Transform, With<Npc>>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<Npc>)>,
    input: Res<Input<KeyCode>>,
) {
    let player = player.single();
    if !input.just_pressed(KeyCode::E) {
        return;
    }
    for npc in &npcs {
        if Vec2::distance(player.translation.truncate(), npc.translation.truncate()) < 1.0 {
            info!("Player chat");
        }
    }
}
