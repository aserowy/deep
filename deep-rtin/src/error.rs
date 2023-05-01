use crate::{
    heightmap::{self, HeightMap},
    rtin::{self, Triangle, Vector2u32},
    u32_extensions::log_2,
};

pub(crate) fn generate(height_map: &HeightMap) -> Vec<f32> {
    let side_length = height_map.width();
    let grid_size = side_length + 1;
    let triangle_count = side_length * side_length * 2 - 2;

    let mut errors = Vec::new();
    errors.resize((grid_size * grid_size) as usize, 0.0f32);

    let level_count = log_2(side_length) * 2;
    let last_level_index = rtin::get_first_triangle_index(level_count - 1);

    for triangle_index in (0..triangle_count).rev() {
        let id = triangle_index + 2;

        let (triangle, midpoint_vector) = rtin::get_triangle_and_midpoint_vector(id, grid_size);

        let triangle_error = get_triangle_error(height_map, triangle, midpoint_vector);
        let error_vector_index = get_index(grid_size, midpoint_vector);

        if triangle_index >= last_level_index {
            errors[error_vector_index] = triangle_error;
        } else {
            let left_child_triangle_index = rtin::get_left_child_triangle_index(id);
            let (_, left_child_midpoint_vector) =
                rtin::get_triangle_and_midpoint_vector(left_child_triangle_index, grid_size);

            let right_child_triangle_index = rtin::get_right_child_triangle_index(id);
            let (_, right_child_midpoint_vector) =
                rtin::get_triangle_and_midpoint_vector(right_child_triangle_index, grid_size);

            let previous_error = errors[error_vector_index];
            let left_error = errors[get_index(grid_size, left_child_midpoint_vector)];
            let right_error = errors[get_index(grid_size, right_child_midpoint_vector)];

            errors[error_vector_index] = triangle_error
                .max(previous_error)
                .max(left_error)
                .max(right_error);
        }
    }

    errors
}

fn get_triangle_error(
    height_map: &HeightMap,
    triangle: Triangle,
    midpoint_vector: Vector2u32,
) -> f32 {
    let vector0_height = heightmap::get_height(height_map, triangle.1);
    let vector1_height = heightmap::get_height(height_map, triangle.2);

    let midpoint_interpolated_height = (vector0_height + vector1_height) / 2.0;
    let midpoint_height = heightmap::get_height(height_map, midpoint_vector);

    (midpoint_interpolated_height - midpoint_height).abs()
}

pub(crate) fn get_index(grid_size: u32, vector: Vector2u32) -> usize {
    (vector[1] * grid_size + vector[0]) as usize
}
