use crate::prelude::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)),
        )
        .add_systems(
            (player_select_attack, update_icon_location)
                .in_set(OnUpdate(CombatState::PlayerSelecting)),
        )
        .add_systems(
            (
                lock_in_attack,
                despawn_with::<SelectionIcon>,
                despawn_with::<WeaponIcon>,
            )
                .chain()
                .in_schedule(OnExit(CombatState::PlayerSelecting)),
        );
    }
}

fn lock_in_attack(
    mut commands: Commands,
    selection: Query<&CurrentSelectedMenuItem, With<SelectionIcon>>,
    enemy: Query<(Entity, &Enemy), Without<PlayerCombat>>,
    weapon_icons: Query<(&WeaponIcon, &Weapon)>,
) {
    let (entity, enemy) = enemy
        .iter()
        .min_by_key(|(_, enemy)| enemy.slot)
        .expect("No enemy to target");
    info!("Locking in attack");

    let selection = selection.single();
    let slot = selection.selection.rem_euclid(selection.slots);

    for (icon, weapon) in &weapon_icons {
        if icon.0 == slot {
            info!("Attacked locked");
            commands.spawn((
                weapon.clone(),
                PlayerAttack {
                    target: entity,
                    slot: enemy.slot,
                },
                CombatEntity,
            ));
            return;
        }
    }

    unreachable!("Player didn't select anything :(");
}

fn spawn_player_attack_icons(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(-3.0, 1.7, 0.0)),
        BillboardSprite::Weapon(Weapon::BasicSpear),
        Weapon::BasicSpear,
        WeaponIcon(0),
        Name::new("SpearIcon"),
    ));

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(-2.5, 1.7, 0.0)),
        BillboardSprite::Weapon(Weapon::BasicStaffOrange),
        Weapon::BasicStaffOrange,
        WeaponIcon(1),
        Name::new("StaffIcon"),
    ));

    commands.spawn((
        SpatialBundle::from_transform(
            Transform::from_xyz(-3.25, 1.0, 0.0).with_scale(Vec3::splat(0.5)),
        ),
        BillboardSprite::Icon(Icon::Pointer),
        CurrentSelectedMenuItem {
            selection: 0,
            slots: 2,
        },
        SelectionIcon,
        Name::new("SelectionIcon"),
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
            info!("Attack Selected");
            next_state.set(CombatState::PlayerAttacking);
        }
    }
}

fn update_icon_location(
    mut selection: Query<(&mut Transform, &CurrentSelectedMenuItem), With<SelectionIcon>>,
) {
    for (mut transform, selection) in &mut selection {
        let location = (selection.selection.rem_euclid(selection.slots)) as f32;
        transform.translation = Vec3::new(-3.25 + location / 2.0, 1.0, 0.0);
    }
}
