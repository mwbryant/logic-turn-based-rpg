use crate::prelude::*;

pub struct PlayerWinsPlugin;

impl Plugin for PlayerWinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_win_fadeout
                .in_set(OnUpdate(CombatState::PlayerWins))
                .in_set(OnUpdate(GameState::Combat)),
        )
        .add_system(transition_to_overworld)
        .add_system(test_combat_end);
    }
}

fn test_combat_end(
    mut combat_state: ResMut<NextState<CombatState>>,
    input: Res<Input<KeyCode>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
) {
    if input.just_pressed(KeyCode::P) {
        combat_state.set(CombatState::PlayerWins);
        overworld_state.set(OverworldState::FreeRoam);
    }
}

fn spawn_win_fadeout(
    mut commands: Commands,
    particle: Query<&VictoryParticle>,
    fadeout: Query<&Fadeout>,
) {
    if fadeout.iter().count() != 0 {
        return; // Only allow 1 fadeout
    }

    //TODO better resource to make sure everything related to winning has finished
    if particle.iter().count() == 0 {
        info!("No particles left, starting fadeout");
        let fadeout = spawn_fadeout(&mut commands);
        commands.entity(fadeout).insert(VictoryFadeout);
    }
}

fn transition_to_overworld(
    fadeout: Query<&Fadeout, With<VictoryFadeout>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
) {
    if let Ok(fadeout) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            next_state.set(GameState::Overworld);
            overworld_state.set(OverworldState::LoadingRoom);
        }
    }
}
