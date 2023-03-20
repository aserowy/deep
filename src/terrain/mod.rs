use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy::utils::default;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use self::generator::generate_mesh;
use self::sky::SkyPlugin;

mod generator;
mod sky;

const GROUND_MULTIPLIER: f32 = 1.0;
const HEIGHT_MULTIPLIER: f32 = 50.0;

pub struct TerrainPlugin {}

impl Default for TerrainPlugin {
    fn default() -> Self {
        TerrainPlugin {}
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SkyPlugin::default())
            .add_system(spawn_youbu_bay.on_startup());
    }
}

fn spawn_youbu_bay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (mesh_vertices, mesh_indices, colors) = generate_mesh(
        "assets/height_maps/youbu_bay.png",
        HEIGHT_MULTIPLIER,
        GROUND_MULTIPLIER,
    );

    let mesh = generate_mesh_from_base_vectors(mesh_vertices.clone(), mesh_indices.clone(), colors);
    let mesh_handle = meshes.add(mesh);

    commands
        .spawn(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 1.0).into(),
                metallic: 10.0,
                perceptual_roughness: 10.0,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(
                -256.0,
                HEIGHT_MULTIPLIER * -0.75,
                -256.0,
            )),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::trimesh(mesh_vertices, mesh_indices));
}

fn generate_mesh_from_base_vectors(
    vertices: Vec<Vec3>,
    indices: Vec<[u32; 3]>,
    colors: Vec<[f32; 4]>,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices.into_iter().flatten().collect())));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh
}
