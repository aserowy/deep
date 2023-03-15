pub fn log_2(x: u32) -> u32 {
    get_most_significant_bit(x) - 1
}

pub fn get_most_significant_bit(x: u32) -> u32 {
    32 - x.leading_zeros()
}

pub fn subtract_abs(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}
