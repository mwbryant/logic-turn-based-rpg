use crate::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CombatState>()
            .add_system(
                spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)),
            )
            .add_systems(
                (lock_in_attack, despawn_player_attack_icons)
                    .chain()
                    .in_schedule(OnExit(CombatState::PlayerSelecting)),
            )
            .add_systems(
                (player_select_attack, update_icon_location)
                    .in_set(OnUpdate(CombatState::PlayerSelecting)),
            )
            .register_type::<CombatStats>()
            .register_type::<Player>()
            .register_type::<CurrentSelectedMenuItem>()
            .register_type::<SelectionIcon>()
            .register_type::<PlayerAttack>()
            .register_type::<WeaponIcon>()
            .register_type::<Enemy>();
    }
}

#[derive(Component, Reflect)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Default, Reflect)]
pub struct Player {
    pub experience: i32,
    pub level: i32,
}

#[derive(Component, Default, Reflect)]
pub struct CurrentSelectedMenuItem {
    pub selection: i32,
    pub slots: i32,
}

#[derive(Component, Reflect)]
pub struct SelectionIcon;

#[derive(Component, Reflect)]
pub struct PlayerAttack;

#[derive(Component, Reflect)]
pub struct WeaponIcon(i32);

#[derive(Component, Reflect)]
pub struct Enemy {
    pub base_experience_reward: i32,
}

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
enum CombatState {
    #[default]
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
}

fn lock_in_attack(
    mut commands: Commands,
    selection: Query<&CurrentSelectedMenuItem, With<SelectionIcon>>,
    weapon_icons: Query<(&WeaponIcon, &Weapon)>,
) {
    let selection = selection.single();
    let slot = selection.selection.rem_euclid(selection.slots);

    for (icon, weapon) in &weapon_icons {
        if icon.0 == slot {
            commands.spawn((weapon.clone(), PlayerAttack));
            return;
        }
    }

    unreachable!("Player didn't select anything :(");
}

fn despawn_player_attack_icons(
    mut commands: Commands,
    selection: Query<Entity, With<SelectionIcon>>,
    weapons: Query<Entity, With<WeaponIcon>>,
) {
    for entity in &selection {
        commands.entity(entity).despawn_recursive()
    }
    for entity in &weapons {
        commands.entity(entity).despawn_recursive()
    }
}

fn spawn_player_attack_icons(mut commands: Commands) {
    commands.spawn((
        WeaponBundle::new(Vec2::new(-3.0, 1.7), Weapon::BasicSpear, Vec2::splat(0.75)),
        WeaponIcon(0),
    ));

    commands.spawn((
        WeaponBundle::new(
            Vec2::new(-2.5, 1.7),
            Weapon::BasicStaffOrange,
            Vec2::splat(0.75),
        ),
        WeaponIcon(1),
    ));

    commands.spawn((
        IconBundle::new(Vec2::new(-3.0, 1.0), Icon::Pointer, Vec2::splat(0.5)),
        CurrentSelectedMenuItem {
            selection: 0,
            slots: 2,
        },
        SelectionIcon,
    ));
}

fn player_select_attack(
    mut selection: Query<&mut CurrentSelectedMenuItem, With<SelectionIcon>>,
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for mut selection in &mut selection {
        if keyboard.just_pressed(KeyCode::A) {
            selection.selection -= 1;
        }
        if keyboard.just_pressed(KeyCode::D) {
            selection.selection += 1;
        }
        if keyboard.just_pressed(KeyCode::Space) {
            next_state.set(CombatState::PlayerAttacking);
        }
    }
}

fn update_icon_location(
    mut selection: Query<(&mut Transform, &CurrentSelectedMenuItem), With<SelectionIcon>>,
) {
    for (mut transform, selection) in &mut selection {
        let location = (selection.selection.rem_euclid(selection.slots)) as f32;
        transform.translation = Vec3::new(-3.0 + location / 2.0, 1.0, ICON_Z);
    }
}
