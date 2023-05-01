use std::collections::HashMap;

use heightmap::HeightMap;
use nalgebra::Vector3;
use rtin::{Triangle, Vector2u32};
use u32_extensions::subtract_abs;

mod error;
mod heightmap;
mod rtin;
mod u32_extensions;

type Vector3f32 = Vector3<f32>;

pub fn retrieve_heigth_map(height_map_path: &str) -> HeightMap {
    let height_map = image::open(height_map_path).unwrap().to_luma16();

    assert_height_map_for_rtin(&height_map);

    height_map
}

fn assert_height_map_for_rtin(height_map: &HeightMap) {
    assert!(
        height_map.width() == height_map.height(),
        "HeightMap must be square!"
    );
    assert!(
        is_power_of_2(height_map.height()),
        "Both sides must have a base of 2"
    );
}

// https://codegolf.stackexchange.com/questions/44680/showcase-of-languages
fn is_power_of_2(x: u32) -> bool {
    (x & !(x & (x - 1))) > 0
}

pub fn get_errors(height_map: &HeightMap) -> Vec<f32> {
    error::generate(height_map)
}

pub fn get_triangles(error_threshold: f32, errors: &[f32]) -> Vec<Triangle> {
    let grid_size = (errors.len() as f32).sqrt() as u32;
    let mut triangles = Vec::<Triangle>::new();

    populate_triangle_ids(
        error_threshold,
        errors,
        &mut triangles,
        grid_size,
        rtin::generate_bottom_left_triangle(grid_size),
    );

    populate_triangle_ids(
        error_threshold,
        errors,
        &mut triangles,
        grid_size,
        rtin::generate_top_right_triangle(grid_size),
    );

    triangles
}

fn populate_triangle_ids(
    error_threshold: f32,
    errors: &[f32],
    triangles: &mut Vec<Triangle>,
    grid_size: u32,
    triangle: Triangle,
) {
    let midpoint = (triangle.1 + triangle.2) / 2;
    let midpoint_vector = Vector2u32::new(midpoint[0], midpoint[1]);

    let a_side_length = subtract_abs(triangle.1[0], triangle.3[0]);
    let b_side_length = subtract_abs(triangle.1[1], triangle.3[1]);
    let triangle_size = a_side_length + b_side_length;

    let error_vector_index = error::get_index(grid_size, midpoint_vector);
    let error = errors[error_vector_index];

    if triangle_size > 1 && error > error_threshold {
        populate_triangle_ids(
            error_threshold,
            errors,
            triangles,
            grid_size,
            rtin::generate_left_triangle(triangle),
        );

        populate_triangle_ids(
            error_threshold,
            errors,
            triangles,
            grid_size,
            rtin::generate_right_triangle(triangle),
        );
    } else {
        triangles.push(triangle);
    }
}

pub fn generate_mesh_data(
    height_map: &HeightMap,
    triangles: &[Triangle],
) -> (Vec<Vector3f32>, Vec<[u32; 3]>, Vec<[f32; 3]>) {
    let side_length = height_map.width();
    let grid_size = side_length + 1;

    let mut vertices = Vec::<Vector3f32>::new();
    let mut indices = Vec::<[u32; 3]>::new();

    let mut added_vertex_by_errors_index = HashMap::<usize, usize>::new();

    for triangle in triangles {
        let mut triangle_indices: [u32; 3] = [0; 3];

        for (index, vertex) in [triangle.1, triangle.2, triangle.3].into_iter().enumerate() {
            let vertex_errors_index = error::get_index(grid_size, vertex);

            if let Some(vertex_index) = added_vertex_by_errors_index.get(&vertex_errors_index) {
                triangle_indices[index] = *vertex_index as u32;
            } else {
                let vertex_index = vertices.len();
                added_vertex_by_errors_index.insert(vertex_errors_index, vertex_index);

                let height = heightmap::get_height(height_map, vertex);

                vertices.push(Vector3f32::new(vertex[0] as f32, height, vertex[1] as f32));
                triangle_indices[index] = vertex_index as u32;
            }
        }

        indices.push(triangle_indices);
    }

    let normals = calculate_normals(&vertices, &indices);

    (vertices, indices, normals)
}

fn calculate_normals(vertices: &[Vector3f32], indices: &[[u32; 3]]) -> Vec<[f32; 3]> {
    let mut normals = Vec::<Vector3f32>::new();
    normals.resize(vertices.len(), Vector3f32::zeros());

    for vertex_indicies in indices {
        let a = vertices[vertex_indicies[0] as usize];
        let b = vertices[vertex_indicies[1] as usize];
        let c = vertices[vertex_indicies[2] as usize];
        let normal = (b - a).cross(&(c - a)).normalize();

        for vertex_index in vertex_indicies {
            let current_normal = normals[*vertex_index as usize];
            normals[*vertex_index as usize] = (normal + current_normal).normalize();
        }
    }

    normals.into_iter().map(|v| [v.x, v.y, v.z]).collect()
}
