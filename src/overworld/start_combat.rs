use crate::prelude::{ui::spawn_enemy_health_ui, *};

pub struct CombatTransitionPlugin;

impl Plugin for CombatTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (start_combat, spawn_enemy_health_ui)
                .chain()
                .in_set(OnUpdate(OverworldState::CombatStarting)),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn start_combat(
    fadeout: Query<(&Fadeout, &CombatFadeout)>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut main_game_state: ResMut<NextState<GameState>>,
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    //Check that fade completes
    assert!(combat_descriptor.iter().count() == 1);
    if let Ok((fadeout, _)) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            info!("Starting Combat");
            overworld_state.set(OverworldState::NotInOverworld);
            main_game_state.set(GameState::Combat);
            combat_state.set(CombatState::PlayerSelecting);
        }
    } else {
        warn!("No fadeout for entering combat!");
    }
}
