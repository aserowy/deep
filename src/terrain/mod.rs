use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy_rapier3d::prelude::{Collider, RigidBody};

use self::generator::generate_mesh;
use self::sky::SkyPlugin;

mod generator;
mod sky;

const GROUND_MULTIPLIER: f32 = 1.0;
const HEIGHT_MULTIPLIER: f32 = 64.0;

#[derive(Default)]
pub struct TerrainPlugin {}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SkyPlugin::default())
            .add_system(spawn_terrain.on_startup());
    }
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    testing_voronoi(&mut commands, &mut meshes, &mut materials);

    let (mesh_vertices, mesh_indices, normals, colors) = generate_mesh(
        "assets/height_maps/youbu_bay.png",
        HEIGHT_MULTIPLIER,
        GROUND_MULTIPLIER,
    );

    let mesh = generate_mesh_from_base_vectors(
        mesh_vertices.clone(),
        mesh_indices.clone(),
        normals,
        colors,
    );
    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial { ..default() }),
            transform: Transform::from_translation(Vec3::new(
                -256.0,
                HEIGHT_MULTIPLIER * -0.75,
                -256.0,
            )),
            ..default()
        },
        RigidBody::Fixed,
        Collider::trimesh(mesh_vertices, mesh_indices),
    ));
}

use rand::distributions::Uniform;
use rand::Rng;

fn testing_voronoi(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let range1 = Uniform::new(0., 100.);
    let range2 = Uniform::new(0., 100.);

    let red_material_handle = materials.add(Color::RED.into());
    (0..10)
        .map(|_| (rng.sample(&range1), rng.sample(&range2)))
        .for_each(|pnt| {
            commands.spawn(PbrBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: red_material_handle.clone(),
                transform: Transform::from_xyz(pnt.0, 0.0, pnt.1),
                ..default()
            });
        });
    //
    // let red_material_handle = materials.add(Color::RED.into());
    // for point in &points {
    //     commands.spawn(PbrBundle {
    //         mesh: meshes.add(shape::UVSphere::default().into()),
    //         material: red_material_handle.clone(),
    //         transform: Transform::from_xyz(point.x as f32, 0.0, point.y as f32),
    //         ..default()
    //     });
    // }
    //
    // let diagram = CentroidDiagram::new(&points).unwrap();
    // let blue_material_handle = materials.add(Color::BLUE.into());
    // for cell in diagram.cells {
    //     let _points: Vec<Vec2> = cell
    //         .points()
    //         .into_iter()
    //         .map(|pnt| Vec2::new(pnt.x as f32, pnt.y as f32))
    //         .inspect(|pnt| {
    //             commands.spawn(PbrBundle {
    //                 mesh: meshes.add(shape::UVSphere::default().into()),
    //                 material: blue_material_handle.clone(),
    //                 transform: Transform::from_xyz(pnt.x, 0.0, pnt.y),
    //                 ..default()
    //             });
    //         })
    //         .collect();
    //
    //     // let polygon = Polygon {
    //     //     points: _points.into_iter().collect(),
    //     //     closed: true,
    //     // };
    //     //
    //     // commands.spawn((
    //     //     ShapeBundle {
    //     //         path: GeometryBuilder::build_as(&polygon),
    //     //         ..default()
    //     //     },
    //     //     Fill::color(Color::SEA_GREEN),
    //     // ));
    // }
}

fn generate_mesh_from_base_vectors(
    vertices: Vec<Vec3>,
    indices: Vec<[u32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices.into_iter().flatten().collect())));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh
}
