use nalgebra::Vector2;

use crate::u32_extensions::*;

pub type Vector2u32 = Vector2<u32>;
pub type Triangle = (u32, Vector2u32, Vector2u32, Vector2u32);

pub(crate) fn generate_left_triangle(triangle: Triangle) -> Triangle {
    (
        get_left_child_triangle_index(triangle.0),
        triangle.3,
        triangle.1,
        (triangle.1 + triangle.2) / 2,
    )
}

pub(crate) fn generate_right_triangle(triangle: Triangle) -> Triangle {
    (
        get_right_child_triangle_index(triangle.0),
        triangle.2,
        triangle.3,
        (triangle.1 + triangle.2) / 2,
    )
}

// reason = "Precedence is correct and cannot be changed!"
#[allow(clippy::precedence)]
pub(crate) fn get_right_child_triangle_index(index: u32) -> u32 {
    let id = index + 2;
    let level = get_level_by_id(id);

    (id + (1 << level + 2) - (1 << (level + 1))) - 2
}

// reason = "Precedence is correct and cannot be changed!"
#[allow(clippy::precedence)]
pub(crate) fn get_left_child_triangle_index(index: u32) -> u32 {
    let id = index + 2;
    let level = get_level_by_id(id);

    (id + (1 << (level + 2))) - 2
}

fn get_level_by_id(id: u32) -> u32 {
    get_most_significant_bit(id) - 2
}

pub(crate) fn get_first_triangle_index(level: u32) -> u32 {
    ((2 << level) - 1) & (!1u32)
}

pub(crate) fn generate_bottom_left_triangle(grid_size: u32) -> Triangle {
    let corner = grid_size - 1;
    (
        1,
        Vector2u32::new(corner, corner),
        Vector2u32::new(0, 0),
        Vector2u32::new(0, corner),
    )
}

pub(crate) fn generate_top_right_triangle(grid_size: u32) -> Triangle {
    let corner = grid_size - 1;
    (
        0,
        Vector2u32::new(0, 0),
        Vector2u32::new(corner, corner),
        Vector2u32::new(corner, 0),
    )
}

pub(crate) fn get_triangle_and_midpoint_vector(id: u32, grid_size: u32) -> (Triangle, Vector2u32) {
    let triangle = get_triangle_by_id(id, grid_size);
    let midpoint = (triangle.1 + triangle.2) / 2;

    (triangle, Vector2u32::new(midpoint[0], midpoint[1]))
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
