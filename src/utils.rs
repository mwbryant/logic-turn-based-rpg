use crate::prelude::*;

pub fn despawn_with<T: Component>(mut commands: Commands, matches: Query<Entity, With<T>>) {
    for entity in &matches {
        commands.entity(entity).despawn_recursive();
    }
}
