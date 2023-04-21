use bevy_easings::Lerp;

use crate::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_test_npc.in_schedule(OnEnter(GameState::Overworld)))
            .add_system(update_dialog_box.in_base_set(CoreSet::PostUpdate))
            .add_system(close_dialog.in_set(OnUpdate(OverworldState::Dialog)))
            .add_system(update_npc_icon.in_base_set(CoreSet::PostUpdate))
            .add_system(player_interaction.in_set(OnUpdate(OverworldState::FreeRoam)));
    }
}

#[derive(Component)]
pub struct NpcText;

#[derive(Component)]
pub struct DialogUI;

fn update_dialog_box(
    mut text: Query<&mut Style, With<NpcText>>,
    camera: Query<&Camera, With<MainCamera>>,
) {
    let camera = camera.single();

    let screen_width = camera.logical_viewport_size().unwrap().x;

    for mut text in &mut text {
        //AHHHHH why must this be Px not percent :(
        //https://github.com/bevyengine/bevy/issues/1490
        text.max_size.width = Val::Px(screen_width * 0.8 - 30.);
    }
}

fn spawn_test_npc(mut commands: Commands) {
    let icon = commands
        .spawn((
            Transform::from_xyz(0., 1.0, 0.0),
            BillboardSprite::Icon(Icon::KeyE),
            InteractIcon,
            Name::new("Npc Interact Icon"),
        ))
        .id();

    commands
        .spawn((
            Transform::from_xyz(3.0, 0.5, -6.0),
            BillboardSprite::Character(Character::WomanOld),
            Npc(1),
            Name::new("TestNPC"),
            OverworldEntity,
        ))
        .add_child(icon);
}

fn update_npc_icon(
    npcs: Query<(&Children, &Transform), With<Npc>>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<Npc>)>,
    mut icons: Query<&mut TextureAtlasSprite, With<InteractIcon>>,
) {
    let player = player.single();
    for (children, npc) in &npcs {
        let distance = Vec2::distance(player.translation.truncate(), npc.translation.truncate());
        for child in children {
            if let Ok(mut sprite) = icons.get_mut(*child) {
                let x_intercept = 2.5;
                let lerp_range = 1.0;
                let lerp_value = -(distance - x_intercept) / lerp_range;
                let alpha = Lerp::lerp(&0.0, &1.0, &lerp_value);

                sprite.color.set_a(alpha);
            }
        }
    }
}

fn player_interaction(
    mut commands: Commands,
    assets: Res<AssetServer>,
    npcs: Query<(&Transform, &Npc)>,
    player: Query<&Transform, (With<PlayerOverworld>, Without<Npc>)>,
    input: Res<Input<KeyCode>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
) {
    let player = player.single();

    if !input.just_pressed(KeyCode::E) {
        return;
    }

    for (npc, id) in &npcs {
        if Vec2::distance(player.translation.truncate(), npc.translation.truncate()) < 1.5 {
            overworld_state.set(OverworldState::Dialog);
            //TEMP
            if id.0 == 1 {
                spawn_dialog_box(
                    &mut commands,
                    &assets,
                    "Hello I am an NPC and this is my test text it should be pretty long and might even span multiple lines on some screen sizes");
            } else {
                spawn_dialog_box(
                    &mut commands,
                    &assets,
                    "I am another NPC who can say something else!",
                );
            }
        }
    }
}

fn close_dialog(
    mut commands: Commands,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    input: Res<Input<KeyCode>>,
    dialog: Query<Entity, With<DialogUI>>,
) {
    if input.just_pressed(KeyCode::E) {
        for dialog in &dialog {
            commands.entity(dialog).despawn_recursive();
            overworld_state.set(OverworldState::FreeRoam);
        }
    }
}

fn spawn_dialog_box(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    starting_text: &str,
) -> Entity {
    //FIXME: Global font setting
    let font = assets.load("fonts/pointfree.ttf");

    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px instead of Percent because UI uses window size, not camera size
                size: Size::new(Val::Px(1920.0 * 0.800), Val::Px(1080.0 * 0.300)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                position: UiRect::new(
                    Val::Px(1920.0 * 0.100),
                    Val::Auto,
                    Val::Px(700.0),
                    Val::Auto,
                ),
                //margin: UiRect::top(Val::Px(700.0)),
                padding: UiRect::new(Val::Px(1920.0 * 0.01), Val::Auto, Val::Px(15.0), Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..default()
        },
        DialogUI,
        Name::new("Dialog UI"),
    );

    let dialog_text = (
        TextBundle::from_section(
            starting_text,
            TextStyle {
                font,
                font_size: 64.0,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Left),
        NpcText,
    );

    commands
        .spawn(parent_node)
        .with_children(|commands| {
            commands.spawn(dialog_text);
        })
        .id()
}
