use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::{shape::Circle, *},
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use crate::render::line::{LineMaterial, LineStrip};

use self::controller::{control_axis_rotation, control_translation, SettingsComponent, ThrustComponent};

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

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::AcesFitted,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            BloomSettings {
                intensity: 0.25, // the default is 0.3
                ..default()
            },
            FogSettings {
                color: Color::rgb(0.0, 0.36, 0.45),
                falloff: FogFalloff::from_visibility_color(256.0, Color::rgb(0.35, 0.5, 0.66)),
                ..default()
            },
            AtmosphereCamera::default(),
            // hud & controls
            VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
            SettingsComponent::default(),
            ThrustComponent::default(),
            // physics
            RigidBody::Dynamic,
            ExternalForce::default(),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(0.0),
            Collider::ball(3.0),
            AdditionalMassProperties::Mass(10.0),
        ))
        .with_children(|parent| {
            // hud
            parent.spawn(MaterialMeshBundle {
                mesh: meshes.add(LineStrip::from(Circle::new(0.002)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });

            parent.spawn(MaterialMeshBundle {
                // TODO: calculate circle size from options.movement_spot (0.075 fits an 125 spot)
                mesh: meshes.add(LineStrip::from(Circle::new(0.075)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });

        });
}
