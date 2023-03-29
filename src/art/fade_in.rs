use crate::prelude::*;

pub struct FadeInPlugin;

impl Plugin for FadeInPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_fadeout);
    }
}

pub fn spawn_fadeout(commands: &mut Commands) -> Entity {
    commands
        .spawn((
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
                fade_in_just_finished: false,
                in_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                hold_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                out_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                state: FadeoutState::FadingIn,
            },
            Name::new("Fadeout"),
        ))
        .id()
}

//TODO break into 2 functions
fn update_fadeout(
    mut commands: Commands,
    mut fadeout: Query<(Entity, &mut BackgroundColor, &mut Fadeout)>,
    time: Res<Time>,
) {
    for (entity, mut color, mut fadeout) in &mut fadeout {
        fadeout.fade_in_just_finished = false;
        match fadeout.state {
            FadeoutState::FadingIn => {
                fadeout.in_timer.tick(time.delta());
                if fadeout.in_timer.just_finished() {
                    fadeout.fade_in_just_finished = true;
                    fadeout.state = FadeoutState::Hold;
                    color.0.set_a(1.0);
                } else {
                    color.0.set_a(fadeout.in_timer.percent());
                }
            }
            FadeoutState::Hold => {
                color.0.set_a(1.0);
                fadeout.hold_timer.tick(time.delta());
                if fadeout.hold_timer.just_finished() {
                    fadeout.state = FadeoutState::FadingOut;
                }
            }
            FadeoutState::FadingOut => {
                fadeout.out_timer.tick(time.delta());
                if fadeout.out_timer.just_finished() {
                    commands.entity(entity).despawn_recursive();
                    color.0.set_a(0.0);
                } else {
                    color.0.set_a(fadeout.out_timer.percent_left());
                }
            }
        }
    }
}
