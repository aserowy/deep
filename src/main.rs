use std::f32::consts::PI;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use submarine::controller::{camera_controller, CameraController};
use terrain::spawn_youbu_bay;

mod submarine;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // debug
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EditorPlugin)
        // game
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems((spawn_youbu_bay.on_startup(), setup.on_startup()))
        .add_systems((camera_controller,))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.25,
    });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        // .looking_at(Vec3::new(-0.15, -0.05, 1.25), Vec3::Y),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(2000.0)),
            ..default()
        },
        NotShadowCaster,
    ));

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
                    512.0,
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
