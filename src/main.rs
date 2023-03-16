use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};
use terrain::spawn_youbu_bay;

mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_systems((spawn_youbu_bay.on_startup(), setup.on_startup()))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 320000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0).with_rotation(Quat::from_rotation_x(PI)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.33,
    });

    commands
        .spawn((
            Camera3dBundle::default(),
            FogSettings {
                color: Color::rgb_u8(210, 220, 240),
                falloff: FogFalloff::Exponential { density: 0.001 },
                ..default()
            },
        ))
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ));
}
