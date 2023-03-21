use crate::prelude::*;

pub struct PlayerWinsPlugin;

impl Plugin for PlayerWinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_win_particles.in_schedule(OnEnter(CombatState::PlayerWins)));
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
        particle: Particle::new(1.0),
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
        radial: Some(RadialParticle { speed: 5.0 }),
    };

    create_new_rect_emitter(
        &mut commands,
        particle_desc,
        Vec2::new(1.0, 1.5),
        Vec2::new(0.5, 0.5),
        0.2,
        4,
        0.01,
    );
}

/*
fn particles_fall(mut particles: Query<&mut Transform, With<VictoryParticle>>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for mut transform in &mut particles {
        transform.translation.y -= 3.0 * time.delta_seconds();
        transform.rotation *= Quat::from_rotation_z(10.0 * time.delta_seconds());

        if transform.translation.y < -6.0 {
            let (x_offset, y_offset) = (rng.gen::<f32>() * 16.0, rng.gen::<f32>() * 30.0);
            transform.translation.x = -8.0 + x_offset;
            transform.translation.y = 6.0 + y_offset;
        }
    }
}

*/
