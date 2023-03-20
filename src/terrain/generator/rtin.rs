use bevy::{
    prelude::{info, Vec3},
    utils::HashMap,
};
use nalgebra::Vector2;

use super::{
    u32::{get_most_significant_bit, log_2, subtract_abs},
    HeightMap,
};

const ERROR_THRESHOLD: f32 = 0.0525;

type Vector = Vector2<u32>;
type Triangle = (u32, Vector, Vector, Vector);

// FIX: one triangle (last?) will not get added to mesh
pub fn generate_mesh_with_rtin(height_map: HeightMap) -> (Vec<Vec3>, Vec<[u32; 3]>, Vec<[f32; 3]>) {
    assert_height_map_for_rtin(&height_map);

    let side_length = height_map.width();
    let grid_size = side_length + 1;

    let errors = generate_errors(&height_map, side_length, grid_size);
    let relevant_triangles = get_relevant_triangles(&errors, grid_size);

    generate_mesh_data(&height_map, grid_size, &relevant_triangles)
}

fn generate_mesh_data(
    height_map: &HeightMap,
    grid_size: u32,
    triangles: &[Triangle],
) -> (Vec<Vec3>, Vec<[u32; 3]>, Vec<[f32; 3]>) {
    let mut vertices = Vec::<Vec3>::new();
    let mut indices = Vec::<[u32; 3]>::new();

    let mut added_vertex_by_errors_index = HashMap::<usize, usize>::new();

    for triangle in triangles {
        let mut triangle_indices: [u32; 3] = [0; 3];

        for (index, vertex) in [triangle.1, triangle.2, triangle.3].into_iter().enumerate() {
            let vertex_errors_index = get_errors_index(grid_size, vertex);

            if let Some(vertex_index) = added_vertex_by_errors_index.get(&vertex_errors_index) {
                triangle_indices[index] = vertex_index.clone() as u32;
            } else {
                let vertex_index = vertices.len();
                added_vertex_by_errors_index.insert(vertex_errors_index, vertex_index);

                let height = get_height_from_height_map(height_map, vertex);

                vertices.push(Vec3::new(vertex[0] as f32, height, vertex[1] as f32));
                triangle_indices[index] = vertex_index as u32;
            }
        }

        indices.push(triangle_indices);
    }

    let normals = calculate_normals(&vertices, &indices);

    info!("{}/{}/{}", vertices.len(), indices.len(), normals.len());

    (vertices, indices, normals)
}

fn calculate_normals(vertices: &[Vec3], indices: &[[u32; 3]]) -> Vec<[f32; 3]> {
    let mut normals = Vec::<Vec3>::new();
    normals.resize(vertices.len(), Vec3::from([0.0; 3]));

    for vertex_indicies in indices {
        let a = vertices[vertex_indicies[0] as usize];
        let b = vertices[vertex_indicies[1] as usize];
        let c = vertices[vertex_indicies[2] as usize];
        let normal = (b - a).cross(c - a).normalize();

        for vertex_index in vertex_indicies {
            let current_normal = normals[vertex_index.clone() as usize];
            normals[vertex_index.clone() as usize] = (normal + current_normal).normalize();
        }
    }

    normals.into_iter().map(|v| [v.x, v.y, v.z]).collect()
}

fn get_relevant_triangles(errors: &[f32], grid_size: u32) -> Vec<Triangle> {
    let mut triangles = Vec::<Triangle>::new();

    populate_triangle_ids(
        &errors,
        &mut triangles,
        grid_size,
        generate_bottom_left_triangle(grid_size),
    );

    populate_triangle_ids(
        &errors,
        &mut triangles,
        grid_size,
        generate_top_right_triangle(grid_size),
    );

    triangles
}

fn populate_triangle_ids(
    errors: &[f32],
    triangles: &mut Vec<Triangle>,
    grid_size: u32,
    triangle: Triangle,
) {
    let midpoint = (triangle.1 + triangle.2) / 2;
    let midpoint_vector = Vector::new(midpoint[0], midpoint[1]);

    let a_side_length = subtract_abs(triangle.1[0], triangle.3[0]);
    let b_side_length = subtract_abs(triangle.1[1], triangle.3[1]);
    let triangle_size = a_side_length + b_side_length;

    let error_vector_index = get_errors_index(grid_size, midpoint_vector);
    let error = errors[error_vector_index];

    if triangle_size > 1 && error > ERROR_THRESHOLD {
        populate_triangle_ids(
            errors,
            triangles,
            grid_size,
            generate_left_triangle(triangle),
        );

        populate_triangle_ids(
            errors,
            triangles,
            grid_size,
            generate_right_triangle(triangle),
        );
    } else {
        triangles.push(triangle);
    }
}

fn generate_errors(height_map: &HeightMap, side_length: u32, grid_size: u32) -> Vec<f32> {
    let triangle_count = side_length * side_length * 2 - 2;

    let mut errors = Vec::new();
    errors.resize((grid_size * grid_size) as usize, 0.0f32);

    let level_count = log_2(side_length) * 2;
    let last_level_index = get_first_triangle_index(level_count - 1);

    for triangle_index in (0..triangle_count).rev() {
        let id = triangle_index + 2;

        let (triangle, midpoint_vector) = get_triangle_and_midpoint_vector(id, grid_size);

        let triangle_error = get_triangle_error(height_map, triangle, midpoint_vector);
        let error_vector_index = get_errors_index(grid_size, midpoint_vector);

        if triangle_index >= last_level_index {
            errors[error_vector_index] = triangle_error;
        } else {
            let left_child_triangle_index = get_left_child_triangle_index(id);
            let (_, left_child_midpoint_vector) =
                get_triangle_and_midpoint_vector(left_child_triangle_index, grid_size);

            let right_child_triangle_index = get_right_child_triangle_index(id);
            let (_, right_child_midpoint_vector) =
                get_triangle_and_midpoint_vector(right_child_triangle_index, grid_size);

            let previous_error = errors[error_vector_index];
            let left_error = errors[get_errors_index(grid_size, left_child_midpoint_vector)];
            let right_error = errors[get_errors_index(grid_size, right_child_midpoint_vector)];

            errors[error_vector_index] = triangle_error
                .max(previous_error)
                .max(left_error)
                .max(right_error);
        }
    }

    errors
}

fn get_right_child_triangle_index(index: u32) -> u32 {
    let id = index + 2;
    let level = get_level_by_id(id);

    (id + (1 << level + 2) - (1 << (level + 1))) - 2
}

fn get_left_child_triangle_index(index: u32) -> u32 {
    let id = index + 2;
    let level = get_level_by_id(id);

    (id + (1 << (level + 2))) - 2
}

fn get_errors_index(grid_size: u32, vector: Vector) -> usize {
    (vector[1] * grid_size + vector[0]) as usize
}

fn get_triangle_error(height_map: &HeightMap, triangle: Triangle, midpoint_vector: Vector) -> f32 {
    let vector0_height = get_height_from_height_map(height_map, triangle.1);
    let vector1_height = get_height_from_height_map(height_map, triangle.2);

    let midpoint_interpolated_height = (vector0_height + vector1_height) / 2.0;
    let midpoint_height = get_height_from_height_map(height_map, midpoint_vector);

    (midpoint_interpolated_height - midpoint_height).abs()
}

fn get_height_from_height_map(height_map: &HeightMap, vector: Vector) -> f32 {
    let mut a = vector[0];
    if a >= height_map.width() {
        a = height_map.width() - 1;
    }

    let mut b = vector[1];
    if b >= height_map.height() {
        b = height_map.width() - 1;
    }

    height_map.get_pixel(a, b).0[0] as f32 / u16::MAX as f32
}

fn get_triangle_and_midpoint_vector(id: u32, grid_size: u32) -> (Triangle, Vector) {
    let triangle = get_triangle_by_id(id, grid_size);
    let midpoint = (triangle.1 + triangle.2) / 2;

    (triangle, Vector::new(midpoint[0], midpoint[1]))
}

fn get_triangle_by_id(id: u32, grid_size: u32) -> Triangle {
    let mut triangle: Triangle;

    if id & 1 > 0 {
        triangle = generate_top_right_triangle(grid_size);
    } else {
        triangle = generate_bottom_left_triangle(grid_size);
    }

    for i in 1..(get_level_by_id(id) + 1) {
        if id & (1 << i) > 0 {
            triangle = generate_left_triangle(triangle);
        } else {
            triangle = generate_right_triangle(triangle);
        }
    }

    triangle
}

fn generate_left_triangle(triangle: Triangle) -> Triangle {
    (
        get_left_child_triangle_index(triangle.0),
        triangle.3,
        triangle.1,
        (triangle.1 + triangle.2) / 2,
    )
}

fn generate_right_triangle(triangle: Triangle) -> Triangle {
    (
        get_right_child_triangle_index(triangle.0),
        triangle.2,
        triangle.3,
        (triangle.1 + triangle.2) / 2,
    )
}

fn generate_bottom_left_triangle(grid_size: u32) -> Triangle {
    let corner = grid_size - 1;
    (
        1,
        Vector::new(corner, corner),
        Vector::new(0, 0),
        Vector::new(0, corner),
    )
}

fn generate_top_right_triangle(grid_size: u32) -> Triangle {
    let corner = grid_size - 1;
    (
        0,
        Vector::new(0, 0),
        Vector::new(corner, corner),
        Vector::new(corner, 0),
    )
}

fn get_level_by_id(id: u32) -> u32 {
    get_most_significant_bit(id) - 2
}

fn get_first_triangle_index(level: u32) -> u32 {
    ((2 << level) - 1) & (!1u32)
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
