use bevy::{
    core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings},
    prelude::{shape::Circle, *},
};
use bevy_atmosphere::prelude::AtmosphereCamera;
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

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut materials: ResMut<Assets<OutlineMaterial>>,
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
            CameraController::default(),
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
                mesh: meshes.add(Circle::new(0.01).into()),
                material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
}
