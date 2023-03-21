use std::f32::consts::PI;

use crate::prelude::*;
use bevy_easings::Lerp;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(death_animation);
    }
}

fn death_animation(mut deaths: Query<(&mut Transform, &Lifetime), With<DeathAnimation>>) {
    for (mut transform, lifetime) in &mut deaths {
        let angle = Lerp::lerp(&0.0, &(PI * 2.0), &lifetime.timer.percent());
        let scale = Lerp::lerp(&1.0, &0.0, &lifetime.timer.percent());
        transform.rotation = Quat::from_rotation_y(angle);
        transform.scale.y = scale;
    }
}
