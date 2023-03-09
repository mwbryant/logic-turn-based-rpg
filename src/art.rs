use bevy_easings::Lerp;

use crate::prelude::*;

pub struct ArtPlugin;

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
            .add_system(spawn_player_weapon.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(despawn_player_weapon.in_schedule(OnExit(CombatState::PlayerAttacking)))
            .add_system(spawn_enemy_weapon.in_schedule(OnEnter(CombatState::EnemyAttacking)))
            .add_system(despawn_enemy_weapon.in_schedule(OnExit(CombatState::EnemyAttacking)))
            .add_system(animate_melee::<Player>.in_set(OnUpdate(CombatState::PlayerAttacking)))
            .add_system(animate_melee::<Enemy>.in_set(OnUpdate(CombatState::EnemyAttacking)))
            .add_system(update_art)
            .register_type::<Icon>()
            .register_type::<Character>();
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    character: Character,
}

#[derive(Bundle)]
pub struct IconBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    icon: Icon,
}

//Used for the weapon in the players hand during an attack animation
#[derive(Component)]
pub struct WeaponGraphic;

pub const CHARACTER_SHEET_WIDTH: usize = 54;
pub const ICON_SHEET_WIDTH: usize = 34;

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Icon {
    #[default]
    Pointer,
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Character {
    #[default]
    WhiteBase,
    WhiteBaseMouth,
    TanBase,
    TanBaseMouth,
    BlackBase,
    BlackBaseMouth,
    GreenBase,
    GreenBaseMouth,
    WomanBraid,
    WomanOld,
    ManStrap,
    ManBeard,
    WomanBraidAlt,
    WomanTwoBraid,
    ManMohawk,
    ManBaldish,
    WomanRedhead,
    WomanSoldier,
    Jester,
    ManOld,
    KnightArmed,
    Knight,
}

#[derive(Resource)]
pub struct SpriteSheetMaps {
    character_atlas: Handle<TextureAtlas>,
    icon_atlas: Handle<TextureAtlas>,
    pub characters: HashMap<Character, usize>,
    pub weapons: HashMap<Weapon, usize>,
    pub icons: HashMap<Icon, usize>,
}

#[derive(Component)]
pub struct AttackAnimation {
    pub starting_x: f32,
    pub ending_x: f32,
    pub max_weapon_rotation: f32,
}

//TODO player location determined from config somehow
fn animate_melee<T: Component>(
    mut attacker: Query<(&mut Transform, &Children), With<T>>,
    attack: Query<(&MeleeAttack, &AttackAnimation)>,
    mut weapon: Query<&mut Transform, (With<Weapon>, Without<T>)>,
) {
    let (mut transform, children) = attacker.get_single_mut().expect("No or multiple attackers");

    let child = children
        .iter()
        .find(|&&child| weapon.contains(child))
        .expect("Attacker doesn't have a weapon sprite");
    let mut child_transform = weapon.get_mut(*child).unwrap();

    if let Ok((attack, animation)) = attack.get_single() {
        match attack.stage {
            AttackStages::Warmup => {
                transform.translation.x = (animation.starting_x)
                    .lerp(&animation.ending_x, &attack.warmup_timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
            AttackStages::Action => {
                transform.translation.x = animation.ending_x;
                //Probably a more elegant solution to this
                if attack.action_timer.percent() < 0.5 {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &0.0,
                        &animation.max_weapon_rotation,
                        &attack.action_timer.percent(),
                    ));
                } else {
                    child_transform.rotation = Quat::from_rotation_z(Lerp::lerp(
                        &animation.max_weapon_rotation,
                        &0.0,
                        &attack.action_timer.percent(),
                    ));
                }
            }
            AttackStages::CoolDown => {
                transform.translation.x = (animation.ending_x)
                    .lerp(&animation.starting_x, &attack.cool_down_timer.percent());
                child_transform.rotation = Quat::from_rotation_z(0.0);
            }
        }
    }
}

//TODO this can probably be a generic system over the With constraint
fn despawn_player_weapon(mut commands: Commands, graphics: Query<Entity, With<WeaponGraphic>>) {
    for entity in &graphics {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_enemy_weapon(mut commands: Commands, graphics: Query<Entity, With<WeaponGraphic>>) {
    for entity in &graphics {
        commands.entity(entity).despawn_recursive();
    }
}

//TODO make generic
fn spawn_enemy_weapon(mut commands: Commands, enemy: Query<Entity, With<Enemy>>) {
    let weapon = Weapon::BasicSpear;
    let enemy = enemy.get_single().expect("No Enemy");
    let new_ent = commands
        .spawn((
            WeaponBundle::new(
                //FIXME magic config
                Vec2::new(-0.40, -0.37),
                weapon,
                Vec2::splat(1.0),
            ),
            WeaponGraphic,
        ))
        .id();
    commands.entity(enemy).add_child(new_ent);
}

fn spawn_player_weapon(
    mut commands: Commands,
    locked_attack: Query<&Weapon, With<PlayerAttack>>,
    player: Query<Entity, With<Player>>,
) {
    let weapon = locked_attack.get_single().expect("No attack selected :(");
    let player = player.get_single().expect("No Player");
    let new_ent = commands
        .spawn((
            WeaponBundle::new(
                //FIXME magic config
                Vec2::new(0.35, -0.37),
                weapon.clone(),
                Vec2::splat(1.0),
            ),
            WeaponGraphic,
        ))
        .id();
    commands.entity(player).add_child(new_ent);
}

fn update_art(
    mut characters: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &Character,
    )>,
    mut weapons: Query<
        (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Weapon),
        Without<Character>,
    >,
    mut icons: Query<
        (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Icon),
        (Without<Character>, Without<Weapon>),
    >,
    sprite_sheets: Res<SpriteSheetMaps>,
) {
    for (mut sprite, mut atlas, character) in &mut characters {
        //TODO animation?
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.characters[character];
    }
    for (mut sprite, mut atlas, weapon) in &mut weapons {
        //TODO animation?
        *atlas = sprite_sheets.character_atlas.clone();
        sprite.index = sprite_sheets.weapons[weapon];
    }
    for (mut sprite, mut atlas, icon) in &mut icons {
        //TODO animation?
        *atlas = sprite_sheets.icon_atlas.clone();
        sprite.index = sprite_sheets.icons[icon];
    }
}

fn setup_spritesheet_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("characters/Spritesheet/roguelikeChar_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        CHARACTER_SHEET_WIDTH,
        12,
        Some(Vec2::splat(1.0)),
        None,
    );
    let character_atlas = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("input_icons/Tilemap/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        ICON_SHEET_WIDTH,
        24,
        Some(Vec2::splat(1.0)),
        None,
    );
    let icon_atlas = texture_atlases.add(texture_atlas);

    let characters = HashMap::from([
        (Character::WhiteBase, 0),
        (Character::WhiteBaseMouth, 1),
        (Character::TanBase, CHARACTER_SHEET_WIDTH),
        (Character::TanBaseMouth, CHARACTER_SHEET_WIDTH + 1),
        (Character::BlackBase, CHARACTER_SHEET_WIDTH * 2),
        (Character::BlackBaseMouth, CHARACTER_SHEET_WIDTH * 2 + 1),
        (Character::GreenBase, CHARACTER_SHEET_WIDTH * 3),
        (Character::GreenBaseMouth, CHARACTER_SHEET_WIDTH * 3 + 1),
        (Character::WomanBraid, CHARACTER_SHEET_WIDTH * 5),
        (Character::WomanOld, CHARACTER_SHEET_WIDTH * 5 + 1),
        (Character::ManStrap, CHARACTER_SHEET_WIDTH * 6),
        (Character::ManBeard, CHARACTER_SHEET_WIDTH * 6 + 1),
        (Character::WomanBraidAlt, CHARACTER_SHEET_WIDTH * 7),
        (Character::WomanTwoBraid, CHARACTER_SHEET_WIDTH * 7 + 1),
        (Character::ManMohawk, CHARACTER_SHEET_WIDTH * 8),
        (Character::ManBaldish, CHARACTER_SHEET_WIDTH * 8 + 1),
        (Character::WomanRedhead, CHARACTER_SHEET_WIDTH * 9),
        (Character::WomanSoldier, CHARACTER_SHEET_WIDTH * 9 + 1),
        (Character::Jester, CHARACTER_SHEET_WIDTH * 10),
        (Character::ManOld, CHARACTER_SHEET_WIDTH * 10 + 1),
        (Character::KnightArmed, CHARACTER_SHEET_WIDTH * 11),
        (Character::Knight, CHARACTER_SHEET_WIDTH * 11 + 1),
    ]);

    let icons = HashMap::from([(Icon::Pointer, ICON_SHEET_WIDTH * 17)]);

    let weapons = HashMap::from([(Weapon::BasicStaffOrange, 42), (Weapon::BasicSpear, 47)]);

    commands.insert_resource(SpriteSheetMaps {
        character_atlas,
        icon_atlas,
        characters,
        weapons,
        icons,
    });
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, CHARACTER_Z)),
                ..Default::default()
            },
            // Calling ..default here causes a stack overflow........
            // Using ..Default::default() gives a proper warning though....
            // https://github.com/bevyengine/bevy/issues/6207
            character: Character::WhiteBase,
        }
    }
}

impl CharacterBundle {
    pub fn new(position: Vec2, character: Character) -> Self {
        let mut bundle = CharacterBundle {
            character,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(CHARACTER_Z);

        bundle
    }
}

impl Default for IconBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, ICON_Z)),
                ..Default::default()
            },
            icon: Icon::Pointer,
        }
    }
}

impl IconBundle {
    pub fn new(position: Vec2, icon: Icon, scale: Vec2) -> Self {
        let mut bundle = IconBundle { icon, ..default() };

        bundle.sprite_sheet.transform.translation = position.extend(ICON_Z);
        bundle.sprite_sheet.transform.scale = scale.extend(1.0);

        bundle
    }
}
