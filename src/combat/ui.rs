use crate::prelude::*;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy_health_ui.in_base_set(StartupSet::PostStartup))
            .add_system(update_enemy_health_ui);
    }
}

#[derive(Component)]
pub struct EnemyHealthUI(pub Entity);
#[derive(Component)]
pub struct EnemyHealthUIText(pub Entity);
#[derive(Component)]
pub struct EnemyHealthUIBar(pub Entity);

fn update_enemy_health_ui(
    mut commands: Commands,
    enemies: Query<&CombatStats>,
    parent: Query<(Entity, &EnemyHealthUI)>,
    mut bar: Query<(&mut Style, &EnemyHealthUIBar)>,
    mut text: Query<(&mut Text, &EnemyHealthUIText)>,
) {
    //Despawn all if connect entity is dead
    for (parent, attached) in &parent {
        if enemies.get(attached.0).is_err() {
            commands.entity(parent).despawn_recursive();
        }
    }

    for (mut bar, attached) in &mut bar {
        if let Ok(enemy) = enemies.get(attached.0) {
            bar.size.width = Val::Percent(enemy.health as f32 / enemy.max_health as f32 * 100.0);
        }
    }

    for (mut text, attached) in &mut text {
        if let Ok(enemy) = enemies.get(attached.0) {
            text.sections[0].value =
                format!("{:?}/{:?}", enemy.health.clamp(0, 9999), enemy.max_health);
        }
    }
}

//FIXME doesn't survive resizing
fn spawn_enemy_health_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, &Enemy)>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let font = asset_server.load("fonts/pointfree.ttf");
    let (camera, transform) = camera.single();

    for (entity, enemy) in &enemies {
        let x = match enemy.slot {
            0 => 0.1,
            1 => 1.3,
            2 => 2.5,
            3 => 3.7,
            _ => unreachable!("Bad slot"),
        };

        let mut coords = camera
            .world_to_viewport(transform, Vec3::new(x, -0.6, 0.0))
            .unwrap();
        //Ugh I think bevy flipped UI after this function was added...
        coords.y = camera.logical_viewport_size().unwrap().y - coords.y;

        let parent_node = (
            NodeBundle {
                style: Style {
                    //TODO can I get the bottom corner from world space too?
                    size: Size::new(Val::Percent(6.8), Val::Percent(5.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(coords.y),
                        left: Val::Px(coords.x),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            EnemyHealthUI(entity),
        );

        let outline_node = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Percent(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        };

        let bar_background_node = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::ORANGE_RED),
            ..default()
        };

        let bar_node = (
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            },
            EnemyHealthUIBar(entity),
        );

        let health_text = (
            TextBundle::from_section(
                "2/2",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Relative,
                position: UiRect { ..default() },
                ..default()
            }),
            EnemyHealthUIText(entity),
        );

        commands.spawn(parent_node).with_children(|commands| {
            commands.spawn(outline_node).with_children(|commands| {
                commands
                    .spawn(bar_background_node)
                    .with_children(|commands| {
                        commands.spawn(bar_node);
                    });
            });
            commands.spawn(health_text);
        });
    }
}
