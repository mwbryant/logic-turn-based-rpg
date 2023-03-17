use std::f32::consts::PI;

use rand::Rng;

use crate::prelude::*;

pub struct PlayerWinsPlugin;

impl Plugin for PlayerWinsPlugin {
    fn build(&self, app: &mut App) {
        //TODO particle lifetimes
        app.add_system(spawn_player_win_particles.in_schedule(OnEnter(CombatState::PlayerWins)))
            .add_system(particles_fall.in_set(OnUpdate(CombatState::PlayerWins)));
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

    let mut rng = rand::thread_rng();

    for _i in 0..3000 {
        let (x_offset, y_offset) = (rng.gen::<f32>() * 16.0, rng.gen::<f32>() * 30.0);
        let angle = rng.gen::<f32>() * 2.0 * PI;
        let index = rng.gen_range(0..4);
        //Faster to spawn batch or not noticible?
        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index,
                    custom_size: Some(Vec2::splat(0.13)),
                    ..default()
                },
                texture_atlas: particle_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(
                    -8.0 + x_offset,
                    6.0 + y_offset,
                    PARTICLE_Z,
                ))
                .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
            VictoryParticle,
        ));
    }
}

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
