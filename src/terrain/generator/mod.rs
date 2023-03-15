use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use image::{ImageBuffer, Luma};

use self::rtin::generate_mesh_with_rtin;

mod rtin;
mod u32;

const GROUND_MULTIPLIER: f32 = 0.5;
const HEIGHT_MULTIPLIER: f32 = 33.0;

type HeightMap = ImageBuffer<Luma<u16>, Vec<u16>>;

pub fn generate_mesh(height_map_path: &str) -> Mesh {
    let height_map = retrieve_heigth_map(height_map_path);
    let (vertices, indices) = generate_mesh_with_rtin(height_map);

    generate_mesh_from_vertices_indices(vertices, indices)
}

fn generate_mesh_from_vertices_indices(vertices: Vec<Vec3>, indices: Vec<u32>) -> Mesh {
    let mut converted_vertices: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    let gradient = colorgrad::plasma();

    for vertex in vertices {
        converted_vertices.push([
            vertex.x * GROUND_MULTIPLIER,
            vertex.y * HEIGHT_MULTIPLIER,
            vertex.z * GROUND_MULTIPLIER,
        ]);

        let color: Vec<f32> = gradient
            .at(vertex.y.into())
            .to_rgba16()
            .into_iter()
            .map(|x| x as f32)
            .collect();

        colors.push([color[0], color[1], color[2], color[3]]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, converted_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh
}

fn retrieve_heigth_map(height_map_path: &str) -> HeightMap {
    image::open(height_map_path).unwrap().to_luma16()
}
