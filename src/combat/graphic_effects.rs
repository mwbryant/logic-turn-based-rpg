use crate::prelude::*;

pub struct GraphicEffectsPlugin;

impl Plugin for GraphicEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hit_particles)
            .add_system(add_death_animation)
            .add_system(spawn_projectile_particles)
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

fn spawn_projectile_particles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    mut projectile_emitter: Query<
        (
            Entity,
            &mut Transform,
            &ProjectileParticleEmitter,
            &RectParticleEmitter,
        ),
        Without<Projectile>,
    >,
) {
    for (emitter_entity, mut emitter_transform, emitter_projectile, emitter) in
        &mut projectile_emitter
    {
        if let Ok((_projectile_entity, projectile_transform)) =
            projectiles.get(emitter_projectile.projectile)
        {
            //Emitter follows projectile
            emitter_transform.translation.x = projectile_transform.translation.x;
            emitter_transform.translation.y = projectile_transform.translation.y;
        } else {
            //If projectile doesn't exist kill emitter and set parent to die
            commands.entity(emitter_entity).despawn_recursive();
            commands.entity(emitter.particle_parent).insert(Lifetime {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            });
        }
    }

    for (projectile, projectile_transform) in &projectiles {
        //Create new emitters, lazily
        if projectile_emitter.get(projectile).is_err() {
            let texture_handle = assets.load("my_art/smoke_particles.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 1, None, None);
            let particle_atlas = texture_atlases.add(texture_atlas);

            let particle_desc = ParticleDesc {
                particle: Particle::new(1.0),
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::splat(0.53)),
                        ..default()
                    },
                    texture_atlas: particle_atlas,
                    ..default()
                },
                falling: Some(FallingParticle { speed: 1.0 }),
                rotating: Some(RotatingParticle { speed: 2.0 }),
                fading: Some(FadingParticle {}),
                radial: Some(RadialParticle { speed: 1.0 }),
                ..default()
            };

            let emitter = create_new_rect_emitter(
                &mut commands,
                particle_desc,
                projectile_transform.translation.truncate(),
                Vec2::new(0.2, 0.2),
                0.2,
                1,
                0.1,
            );
            commands
                .entity(emitter)
                .insert(ProjectileParticleEmitter { projectile });
        }
    }
}

fn spawn_hit_particles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut hit_event: EventReader<HitEvent>,
    target: Query<&Transform>,
) {
    for hit in hit_event.iter() {
        let translation = target.get(hit.target).expect("No target").translation;

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