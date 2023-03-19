use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::controller::{control_axis_rotation, control_translation, CameraController};

mod controller;

pub struct SubmarinePlugin {}

impl Default for SubmarinePlugin {
    fn default() -> Self {
        SubmarinePlugin {}
    }
}

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_camera.on_startup())
            .add_systems((control_axis_rotation, control_translation).chain());
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 20.0, 0.0),
                ..default()
            },
            CameraController::default(),
            FogSettings {
                color: Color::rgb_u8(2, 75, 134),
                directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
                directional_light_exponent: 75.0,
                falloff: FogFalloff::from_visibility_colors(
                    768.0,
                    Color::rgb(0.35, 0.5, 0.66),
                    Color::rgb(0.8, 0.844, 1.0),
                ),
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Collider::ball(1.0))
        .insert(AdditionalMassProperties::Mass(10.0));
}
