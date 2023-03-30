use crate::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_test_npc.in_schedule(OnEnter(GameState::Overworld)))
            .add_system(update_test_npc)
            .add_system(player_interaction);
    }
}

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct NpcText;

fn update_test_npc(
    mut text: Query<&mut Style, With<NpcText>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera.single();
    let screen_width = camera.logical_viewport_size().unwrap().x;
    for mut text in &mut text {
        text.max_size.width = Val::Px(screen_width * 0.75);
    }
}

fn spawn_test_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera.single();
    let screen_width = camera.logical_viewport_size().unwrap().x;
    //FIXME: Global font setting
    let font = asset_server.load("fonts/pointfree.ttf");
    commands.spawn((
        CharacterBundle::new(Vec3::new(-3.0, 3.0, NPC_Z), Character::WomanOld),
        Npc,
        Name::new("TestNPC"),
    ));
    let parent_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(30.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                position: UiRect::left(Val::Percent(10.0)),
                padding: UiRect::top(Val::Px(15.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        Name::new("Dialog UI"),
    );
    let mut dialog_text = (
        TextBundle::from_section(
            "Hello I am an NPC and this is my test text it should be pretty long and might even span multiple lines on some screen sizes",
            TextStyle {
                font,
                font_size: 36.0,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Left),
        NpcText
    );
    //AHHHHH why must this be Px not percent :(
    //https://github.com/bevyengine/bevy/issues/1490
    dialog_text.0.style.max_size.width = Val::Px(screen_width * 0.75);

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(dialog_text);
    });
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
