use bevy::prelude::{
    Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Transform, Vec3,
};
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
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 00.0, 0.0)),
        ..default()
    });
}

