use bevy::prelude::{
    Assets, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Transform, Vec3,
};
use bevy::render::color::Color;
use bevy::utils::default;

use self::generator::generate_mesh;

mod generator;

pub fn spawn_youbu_bay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let height_map_path = "assets/height_maps/youbu_bay.png";
    let mesh = generate_mesh(height_map_path);
    let mesh_handle = meshes.add(mesh);

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0).into(),
            perceptual_roughness: 1.0,
            reflectance: 1.0,
            // metallic: 0.01,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(-128.0, 0.0, -128.0)),
        ..default()
    });
}
