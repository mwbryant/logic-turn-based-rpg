use crate::prelude::*;

pub struct GraphicEffectsPlugin;

impl Plugin for GraphicEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hit_particles)
            .add_system(add_death_animation)
            .add_system(spawn_player_win_particles.in_schedule(OnEnter(CombatState::PlayerWins)));
    }
}

fn add_death_animation(mut commands: Commands, mut death_event: EventReader<DeathEvent>) {
    for death in death_event.iter() {
        commands.entity(death.entity).insert((
            DeathAnimation,
            Lifetime {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            },
        ));
    }
}

fn spawn_hit_particles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut hit_event: EventReader<HitEvent>,
    player: Query<&Transform, With<Player>>,
    enemy: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    for hit in hit_event.iter() {
        let player = player.get_single().expect("No player");
        let enemy = enemy.get_single().expect("No enemy");

        let translation = match hit.combat_state {
            CombatState::PlayerSelecting | CombatState::PlayerWins => {
                unreachable!("Can't hit in this state")
            }
            CombatState::PlayerAttacking => enemy.translation,
            CombatState::EnemyAttacking => player.translation,
        };

        let texture_handle = assets.load("my_art/particles.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2, None, None);
        let particle_atlas = texture_atlases.add(texture_atlas);

        let particle_desc = ParticleDesc {
            particle: Particle::new(1.0),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(0.13)),
                    ..default()
                },
                texture_atlas: particle_atlas,
                ..default()
            },
            //falling: Some(FallingParticle { speed: 3.0 }),
            rotating: Some(RotatingParticle { speed: 10.0 }),
            fading: Some(FadingParticle {}),
            radial: Some(RadialParticle { speed: 5.0 }),
            ..default()
        };

        create_new_rect_emitter(
            &mut commands,
            particle_desc,
            translation.truncate(),
            Vec2::new(0.8, 0.8),
            0.2,
            1,
            0.03,
        );
    }
}

fn spawn_player_win_particles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("my_art/particles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2, None, None);
    let particle_atlas = texture_atlases.add(texture_atlas);

    let particle_desc = ParticleDesc {
        particle: Particle::new(4.0),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(0.13)),
                ..default()
            },
            texture_atlas: particle_atlas,
            ..default()
        },
        falling: Some(FallingParticle { speed: 3.0 }),
        rotating: Some(RotatingParticle { speed: 10.0 }),
        fading: Some(FadingParticle {}),
        radial: Some(RadialParticle { speed: 1.0 }),
    };

    create_new_rect_emitter(
        &mut commands,
        particle_desc,
        Vec2::new(0.0, 6.5),
        Vec2::new(11.5, 0.5),
        8.0,
        4,
        0.01,
    );
}
