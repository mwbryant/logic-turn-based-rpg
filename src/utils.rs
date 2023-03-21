use std::time;

use crate::prelude::*;

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

pub fn update_lifetimes(
    mut commands: Commands,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn despawn_with<T: Component>(mut commands: Commands, matches: Query<Entity, With<T>>) {
    for entity in &matches {
        commands.entity(entity).despawn_recursive();
    }
}
