use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system(spawn_map);
    }
}

pub fn spawn_hit_box(
    commands: &mut Commands,
    box_size: Vec2,
    bottom_left_position: Vec2,
) -> Entity {
    commands
        .spawn((
            Collider::cuboid(box_size.x / 2.0, box_size.y / 2.0),
            TransformBundle::from(Transform::from_xyz(
                bottom_left_position.x + box_size.x / 2.0,
                bottom_left_position.y + box_size.y / 2.0,
                0.0,
            )),
            Name::new("Hitbox"),
        ))
        .id()
}
