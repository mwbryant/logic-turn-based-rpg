use crate::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CombatState>()
            .add_event::<HitEvent>()
            .add_system(
                spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)),
            )
            .add_systems(
                (
                    lock_in_attack,
                    despawn_with::<SelectionIcon>,
                    despawn_with::<WeaponIcon>,
                )
                    .chain()
                    .in_schedule(OnExit(CombatState::PlayerSelecting)),
            )
            .add_system(start_player_attack.in_schedule(OnEnter(CombatState::PlayerAttacking)))
            .add_system(start_enemy_attack.in_schedule(OnEnter(CombatState::EnemyAttacking)))
            .add_system(
                despawn_with::<MeleeAttack>.in_schedule(OnExit(CombatState::PlayerAttacking)),
            )
            .add_system(
                despawn_with::<MeleeAttack>.in_schedule(OnExit(CombatState::EnemyAttacking)),
            )
            .add_systems(
                (player_select_attack, update_icon_location)
                    .in_set(OnUpdate(CombatState::PlayerSelecting)),
            )
            .add_systems(
                (melee_attack_flow, player_action_timing, deal_damage)
                    .in_set(OnUpdate(CombatState::PlayerAttacking)),
            )
            //I wish I could and an in set
            .add_systems(
                (melee_attack_flow, player_action_timing, deal_damage)
                    .in_set(OnUpdate(CombatState::EnemyAttacking)),
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
pub struct EnemyAttack;

#[derive(Component, Reflect)]
pub struct WeaponIcon(i32);

#[derive(Component, Reflect)]
pub struct Enemy {
    pub base_experience_reward: i32,
}

pub struct HitEvent {
    action: ActionTiming,
    combat_state: CombatState,
}

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum CombatState {
    #[default]
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
}

pub enum AttackStages {
    Warmup,
    Action,
    CoolDown,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ActionTiming {
    NotEntered,
    Early,
    Critical,
    Late,
}

#[derive(Component)]
pub struct MeleeAttack {
    pub stage: AttackStages,
    pub action_input: ActionTiming,
    pub warmup_timer: Timer,
    pub action_timer: Timer,
    pub cool_down_timer: Timer,
}

fn start_enemy_attack(mut commands: Commands, enemy: Query<&Enemy>) {
    //TODO attack based on enemy
    let _enemy = enemy.get_single().expect("More than 1 or 0 enemies...");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    commands.spawn((
        MeleeAttack {
            stage: AttackStages::Warmup,
            action_input: ActionTiming::NotEntered,
            warmup_timer: Timer::from_seconds(1.0, TimerMode::Once),
            action_timer: Timer::from_seconds(0.2, TimerMode::Once),
            cool_down_timer: Timer::from_seconds(0.7, TimerMode::Once),
        },
        AttackAnimation {
            starting_x: 3.0,
            ending_x: -1.9,
            max_weapon_rotation: 1.0,
        },
    ));
}

fn start_player_attack(
    mut commands: Commands,
    locked_attack: Query<(Entity, &Weapon), With<PlayerAttack>>,
) {
    let (entity, weapon) = locked_attack.get_single().expect("No attack!");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    let attack_type = weapon.attack_type();
    match attack_type {
        WeaponAttackType::Melee => {
            commands.entity(entity).insert((
                MeleeAttack {
                    stage: AttackStages::Warmup,
                    action_input: ActionTiming::NotEntered,
                    warmup_timer: Timer::from_seconds(1.0, TimerMode::Once),
                    action_timer: Timer::from_seconds(0.2, TimerMode::Once),
                    cool_down_timer: Timer::from_seconds(0.7, TimerMode::Once),
                },
                //FIXME this should be from a bundle or something...
                AttackAnimation {
                    starting_x: -3.0,
                    ending_x: 1.9,
                    max_weapon_rotation: -1.0,
                },
            ));
        }
        WeaponAttackType::Range => todo!(),
    }
}

fn player_action_timing(mut attack: Query<&mut MeleeAttack>, keyboard: Res<Input<KeyCode>>) {
    for mut attack in &mut attack {
        if keyboard.just_pressed(KeyCode::Space) && attack.action_input == ActionTiming::NotEntered
        {
            match attack.stage {
                AttackStages::Warmup => {
                    if attack.warmup_timer.percent() > 0.7 {
                        attack.action_input = ActionTiming::Early;
                    }
                }
                AttackStages::Action => {
                    attack.action_input = ActionTiming::Critical;
                }
                AttackStages::CoolDown => {
                    if attack.cool_down_timer.percent() < 0.3 {
                        attack.action_input = ActionTiming::Late;
                    }
                }
            }
        }
    }
}

fn deal_damage(
    mut hit_event: EventReader<HitEvent>,
    mut player: Query<&mut CombatStats, With<Player>>,
    mut enemy: Query<&mut CombatStats, (With<Enemy>, Without<Player>)>,
) {
    for hit in hit_event.iter() {
        let mut player = player.get_single_mut().expect("No player");
        let mut enemy = enemy.get_single_mut().expect("No enemy");

        match hit.combat_state {
            CombatState::PlayerSelecting => unreachable!("Can't hit in menu"),
            CombatState::PlayerAttacking => {
                let damage = (if matches!(hit.action, ActionTiming::Critical) {
                    (player.attack - enemy.defense) * 2
                } else {
                    player.attack - enemy.defense
                })
                .clamp(0, 99);
                info!("player hit, {:?} {:?}", hit.action, damage);
                enemy.health -= damage;
            }
            CombatState::EnemyAttacking => {
                let damage = (if matches!(hit.action, ActionTiming::Critical) {
                    (enemy.attack - player.defense) / 2
                } else {
                    enemy.attack - player.defense
                })
                .clamp(0, 99);
                info!("enemy hit, {:?} {:?}", hit.action, damage);
                player.health -= damage;
            }
        }
    }
}

fn melee_attack_flow(
    mut attack: Query<&mut MeleeAttack>,
    time: Res<Time>,
    mut hit_event: EventWriter<HitEvent>,
    state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for mut attack in &mut attack {
        match attack.stage {
            AttackStages::Warmup => {
                attack.warmup_timer.tick(time.delta());
                if attack.warmup_timer.just_finished() {
                    attack.stage = AttackStages::Action;
                }
            }
            AttackStages::Action => {
                attack.action_timer.tick(time.delta());
                if attack.action_timer.just_finished() {
                    hit_event.send(HitEvent {
                        action: attack.action_input,
                        combat_state: state.0.clone(),
                    });
                    attack.stage = AttackStages::CoolDown;
                }
            }
            AttackStages::CoolDown => {
                attack.cool_down_timer.tick(time.delta());
                if attack.cool_down_timer.just_finished() {
                    info!("Attack Complete");
                    match state.0 {
                        CombatState::PlayerSelecting => unreachable!("Can't finish attack in menu"),
                        CombatState::PlayerAttacking => next_state.set(CombatState::EnemyAttacking),
                        CombatState::EnemyAttacking => next_state.set(CombatState::PlayerSelecting),
                    }
                }
            }
        }
    }
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
        transform.translation = Vec3::new(-3.0 + location / 2.0, 1.0, ICON_Z);
    }
}
