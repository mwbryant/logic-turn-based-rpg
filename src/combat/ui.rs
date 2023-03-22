use crate::prelude::*;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy_health_ui.in_base_set(StartupSet::PostStartup));
    }
}

//FIXME doesn't survive resizing
fn spawn_enemy_health_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let font = asset_server.load("fonts/pointfree.ttf");

    let (camera, transform) = camera.single();

    let mut coords = camera
        .world_to_viewport(transform, Vec3::new(3.7, -0.6, 0.0))
        .unwrap();
    //Ugh I think bevy flipped UI after this function was added...
    coords.y = camera.logical_viewport_size().unwrap().y - coords.y;

    info!("{:?}", coords);

    let parent_node = NodeBundle {
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
    };

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

    let bar_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::GOLD),
        ..default()
    };

    let health_text = TextBundle::from_section(
        "2/2",
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::WHITE,
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Relative,
        position: UiRect { ..default() },
        ..default()
    });

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
