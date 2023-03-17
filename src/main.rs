use std::f32::consts::PI;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::{prelude::*, rapier::prelude::RigidBodyType};
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
        .add_system(grab_mouse)
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
        color: Color::WHITE,
        brightness: 0.25,
    });

    commands
        .spawn((
            Camera3dBundle::default(),
            FogSettings {
                color: Color::rgb_u8(2, 75, 134),
                falloff: FogFalloff::Exponential { density: 0.002 },
                ..default()
            },
        ))
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(AdditionalMassProperties::Mass(10.0));
}

fn grab_mouse(mut windows: Query<&mut Window>, mouse: Res<Input<MouseButton>>) {
    let mut window = windows.single_mut();

    if mouse.pressed(MouseButton::Right) {
        info!("left mouse pressed");

        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if mouse.just_released(MouseButton::Right) {
        info!("left mouse just released");

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
