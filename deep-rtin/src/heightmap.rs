use image::{ImageBuffer, Luma};

use crate::Vector2u32;

pub type HeightMap = ImageBuffer<Luma<u16>, Vec<u16>>;

pub fn get_height(height_map: &HeightMap, vector: Vector2u32) -> f32 {
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
