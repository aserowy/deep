use bevy::prelude::Vec3;
use image::{ImageBuffer, Luma};

use self::rtin::generate_mesh_with_rtin;

mod rtin;
mod u32;

type HeightMap = ImageBuffer<Luma<u16>, Vec<u16>>;

pub fn generate_mesh(
    height_map_path: &str,
    height_multiplier: f32,
    ground_multiplier: f32,
) -> (Vec<Vec3>, Vec<[u32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>) {
    let height_map = retrieve_heigth_map(height_map_path);
    let (vertices, indices, normals) = generate_mesh_with_rtin(height_map);

    let mut colors = Vec::<[f32; 4]>::new();
    let mut converted_vertices: Vec<Vec3> = Vec::new();

    let gradient = colorgrad::CustomGradient::new()
        .html_colors(&[
            "#7400b8",
            "#6930c3",
            "#5e60ce",
            "#64dfdf",
        ])
        .build()
        .unwrap();

    for vertex in vertices {
        converted_vertices.push(Vec3::new(
            vertex.x * ground_multiplier,
            vertex.y * height_multiplier,
            vertex.z * ground_multiplier,
        ));

        let color: Vec<f32> = gradient
            .at(vertex.y as f64)
            .to_rgba16()
            .into_iter()
            .map(|x| (x as f32) / u16::MAX as f32)
            .collect();

        colors.push([color[0], color[1], color[2], color[3]]);
    }

    (converted_vertices, indices, normals, colors)
}

fn retrieve_heigth_map(height_map_path: &str) -> HeightMap {
    image::open(height_map_path).unwrap().to_luma16()
}
