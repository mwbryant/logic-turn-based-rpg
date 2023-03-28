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

//FIXME windows uses \ .. fix in macro
#[macro_export]
macro_rules! comp_from_config {
    ($comp_type:ty) => {
        ron::from_str::<$comp_type>(
            &std::fs::read_to_string(
                "config/".to_owned() + &stringify!($comp_type).to_lowercase() + ".ron",
            )
            .unwrap(),
        )
        .expect(&("Failed to load ".to_owned() + &stringify!($comp_type).to_lowercase() + ".ron"))
    };
    ($comp_type:ty,$file_name:expr) => {
        ron::from_str::<$comp_type>(&std::fs::read_to_string(($file_name)).unwrap())
            .expect(&("Failed to load ".to_owned() + $file_name))
    };
}
