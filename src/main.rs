use std::f32::consts::PI;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;

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
        .add_plugin(TerrainPlugin::default())
        .add_plugin(SubmarinePlugin::default())
        .add_system(setup.on_startup())
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb_u8(2, 75, 134),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(2000.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}
