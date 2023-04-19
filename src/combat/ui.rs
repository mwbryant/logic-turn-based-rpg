use crate::prelude::*;

use super::start_combat::spawn_combat;

pub struct CombatUIPlugin;

#[derive(Component)]
pub struct HeaderBarUI;
#[derive(Component)]
pub struct PlayerHealthUIText;

#[derive(Component)]
pub struct EnemyHealthUI(pub Entity);
#[derive(Component)]
pub struct EnemyHealthUIText(pub Entity);
#[derive(Component)]
pub struct EnemyHealthUIBar(pub Entity);

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_system_buffers, spawn_enemy_health_ui)
                .chain()
                .after(spawn_combat)
                .in_schedule(OnEnter(GameState::Combat)),
        )
        .add_startup_system(spawn_header_bar_ui.in_base_set(StartupSet::PostStartup))
        .add_system(update_header_bar_ui)
        .add_system(update_enemy_health_ui);
    }
}

fn update_header_bar_ui(
    mut player_text: Query<&mut Text, With<PlayerHealthUIText>>,
    player: Query<&CombatStats, With<PlayerCombat>>,
) {
    let player = player.get_single().expect("More than 1 player?");
    let mut player_text = player_text
        .get_single_mut()
        .expect("More than 1 health bar?");
    player_text.sections[0].value = format!(
        "{:?} / {:?}",
        player.health.clamp(0, 9999),
        player.max_health
    );
}

fn spawn_header_bar_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/pointfree.ttf");

    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                size: Size::new(Val::Px(1920.0), Val::Px(1080.0 * 0.15)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            //background_color: BackgroundColor(Color::GREEN),
            ..default()
        },
        HeaderBarUI,
        Name::new("Header Bar UI"),
    );

    let column_1 = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(33.3), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        //background_color: BackgroundColor(Color::BLUE),
        ..default()
    },);

    let column_2 = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(66.6), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        //background_color: BackgroundColor(Color::GREEN),
        ..default()
    },);

    let player_health_background = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            padding: UiRect::right(Val::Percent(1.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::PINK),
        ..default()
    },);

    let player_icon = (ImageBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Percent(200.0)),
            ..default()
        },
        image: UiImage::new(asset_server.load("characters/PlayerIcon.png")),
        ..default()
    },);

    let health_text = (
        TextBundle::from_section(
            "10 / 10",
            TextStyle {
                font,
                font_size: 36.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center),
        PlayerHealthUIText,
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(column_1).with_children(|commands| {
            commands
                .spawn(player_health_background)
                .with_children(|commands| {
                    commands.spawn(player_icon);
                    commands.spawn(health_text);
                });
        });
        commands.spawn(column_2);
    });
}

fn update_enemy_health_ui(
    mut commands: Commands,
    enemies: Query<(&Enemy, &CombatStats)>,
    mut parent: Query<(Entity, &mut Style, &EnemyHealthUI)>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut bar: Query<(&mut Style, &EnemyHealthUIBar), Without<EnemyHealthUI>>,
    mut text: Query<(&mut Text, &EnemyHealthUIText)>,
) {
    let (camera, transform) = camera.single();

    for (parent, mut style, attached) in &mut parent {
        if let Ok((enemy, _stats)) = enemies.get(attached.0) {
            let x = match enemy.slot {
                0 => 0.1,
                1 => 1.3,
                2 => 2.5,
                3 => 3.7,
                _ => unreachable!("Bad slot"),
            };
            let mut coords_top_left = camera
                .world_to_viewport(transform, Vec3::new(x, -0.6, 0.0))
                .unwrap();
            let mut coords_bottom_right = camera
                .world_to_viewport(transform, Vec3::new(x + 1.0, -1.0, 0.0))
                .unwrap();
            //Ugh I think bevy flipped UI after this function was added...
            coords_top_left.y = camera.logical_viewport_size().unwrap().y - coords_top_left.y;
            coords_bottom_right.y =
                camera.logical_viewport_size().unwrap().y - coords_bottom_right.y;

            style.size = Size::new(
                Val::Px(coords_bottom_right.x - coords_top_left.x),
                Val::Px(coords_bottom_right.y - coords_top_left.y),
            );
            style.position = UiRect {
                top: Val::Px(coords_top_left.y),
                left: Val::Px(coords_top_left.x),
                bottom: Val::Px(coords_bottom_right.y),
                right: Val::Px(coords_bottom_right.x),
            };
        } else {
            //Despawn all if connect entity is dead
            commands.entity(parent).despawn_recursive();
        }
    }

    for (mut bar, attached) in &mut bar {
        if let Ok((_, enemy)) = enemies.get(attached.0) {
            bar.size.width = Val::Percent(enemy.health as f32 / enemy.max_health as f32 * 100.0);
        }
    }

    for (mut text, attached) in &mut text {
        if let Ok((_, enemy)) = enemies.get(attached.0) {
            text.sections[0].value =
                format!("{:?}/{:?}", enemy.health.clamp(0, 9999), enemy.max_health);
        }
    }
}

fn spawn_enemy_health_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let font = asset_server.load("fonts/pointfree.ttf");

    for entity in &enemies {
        // Size and Position setup in update
        let parent_node = (
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            EnemyHealthUI(entity),
            Name::new("Enemy Health Bar UI"),
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
            .with_text_alignment(TextAlignment::Center),
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
