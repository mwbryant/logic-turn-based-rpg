use crate::prelude::*;

pub struct PlayerWinsPlugin;

impl Plugin for PlayerWinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_fadeout
                .in_set(OnUpdate(CombatState::PlayerWins))
                .in_set(OnUpdate(GameState::Combat)),
        )
        .add_system(update_fadeout.in_set(OnUpdate(CombatState::PlayerWins)))
        .register_type::<Fadeout>()
        .add_system(test_combat_end);
    }
}

#[derive(Reflect)]
pub enum FadeoutState {
    FadingIn,
    Hold,
    FadingOut,
}

#[derive(Component, Reflect)]
pub struct Fadeout {
    timer: Timer,
    state: FadeoutState,
}

fn test_combat_end(mut combat_state: ResMut<NextState<CombatState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::P) {
        combat_state.set(CombatState::PlayerWins);
    }
}

fn spawn_fadeout(
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
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(200.0), Val::Percent(200.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.10, 0.10, 0.10, 0.0)),
                z_index: ZIndex::Global(10000),
                ..default()
            },
            Fadeout {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                state: FadeoutState::FadingIn,
            },
            Name::new("Fadeout"),
        ));
    }
}

//TODO break into 2 functions
fn update_fadeout(
    mut commands: Commands,
    mut fadeout: Query<(Entity, &mut BackgroundColor, &mut Fadeout)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut color, mut fadeout)) = fadeout.get_single_mut() {
        fadeout.timer.tick(time.delta());
        match fadeout.state {
            FadeoutState::FadingIn => {
                if fadeout.timer.just_finished() {
                    next_state.set(GameState::Overworld);
                    fadeout.state = FadeoutState::Hold;
                    color.0.set_a(1.0);
                } else {
                    color.0.set_a(fadeout.timer.percent());
                }
            }
            FadeoutState::Hold => {
                color.0.set_a(1.0);
                if fadeout.timer.just_finished() {
                    fadeout.state = FadeoutState::FadingOut;
                }
            }
            FadeoutState::FadingOut => {
                if fadeout.timer.just_finished() {
                    commands.entity(entity).despawn_recursive();
                    color.0.set_a(0.0);
                } else {
                    color.0.set_a(fadeout.timer.percent_left());
                }
            }
        }
    }
}
