use bevy::prelude::Color;

/*
--french-violet: #7400b8;
--grape: #6930c3;
--slate-blue: #5e60ce;
--united-nations-blue: #5390d9;
--picton-blue: #4ea8de;
--aero: #48bfe3;
--sky-blue: #56cfe1;
--tiffany-blue: #64dfdf;
--turquoise: #72efdd;
--aquamarine: #80ffdb;
 */

#[allow(dead_code)]
pub const TRANSPARENT: Color = Color::Rgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 0.0,
};
#[allow(dead_code)]
pub const FRENCH_VIOLET: Color = Color::Hsla {
    hue: 278.0,
    saturation: 1.0,
    lightness: 0.36,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const FRENCH_VIOLET_50: Color = Color::Hsla {
    hue: 278.0,
    saturation: 1.0,
    lightness: 0.36,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const FRENCH_VIOLET_25: Color = Color::Hsla {
    hue: 278.0,
    saturation: 1.0,
    lightness: 0.36,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const GRAPE: Color = Color::Hsla {
    hue: 263.0,
    saturation: 0.60,
    lightness: 0.48,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const GRAPE_50: Color = Color::Hsla {
    hue: 263.0,
    saturation: 0.60,
    lightness: 0.48,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const GRAPE_25: Color = Color::Hsla {
    hue: 263.0,
    saturation: 0.60,
    lightness: 0.48,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const SLATE_BLUE: Color = Color::Hsla {
    hue: 239.0,
    saturation: 0.53,
    lightness: 0.59,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const SLATE_BLUE_50: Color = Color::Hsla {
    hue: 239.0,
    saturation: 0.53,
    lightness: 0.59,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const SLATE_BLUE_25: Color = Color::Hsla {
    hue: 239.0,
    saturation: 0.53,
    lightness: 0.59,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const UNITED_NATIONS_BLUE: Color = Color::Hsla {
    hue: 213.0,
    saturation: 0.64,
    lightness: 0.59,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const UNITED_NATIONS_BLUE_50: Color = Color::Hsla {
    hue: 213.0,
    saturation: 0.64,
    lightness: 0.59,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const UNITED_NATIONS_BLUE_25: Color = Color::Hsla {
    hue: 213.0,
    saturation: 0.64,
    lightness: 0.59,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const PICTON_BLUE: Color = Color::Hsla {
    hue: 203.0,
    saturation: 0.69,
    lightness: 0.59,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const PICTON_BLUE_50: Color = Color::Hsla {
    hue: 203.0,
    saturation: 0.69,
    lightness: 0.59,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const PICTON_BLUE_25: Color = Color::Hsla {
    hue: 203.0,
    saturation: 0.69,
    lightness: 0.59,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const AERO: Color = Color::Hsla {
    hue: 194.0,
    saturation: 0.73,
    lightness: 0.59,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const AERO_50: Color = Color::Hsla {
    hue: 194.0,
    saturation: 0.73,
    lightness: 0.59,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const AERO_25: Color = Color::Hsla {
    hue: 194.0,
    saturation: 0.73,
    lightness: 0.59,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const SKY_BLUE: Color = Color::Hsla {
    hue: 188.0,
    saturation: 0.70,
    lightness: 0.61,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const SKY_BLUE_50: Color = Color::Hsla {
    hue: 188.0,
    saturation: 0.70,
    lightness: 0.61,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const SKY_BLUE_25: Color = Color::Hsla {
    hue: 188.0,
    saturation: 0.70,
    lightness: 0.61,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const TIFFANY_BLUE: Color = Color::Hsla {
    hue: 180.0,
    saturation: 0.66,
    lightness: 0.63,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const TIFFANY_BLUE_50: Color = Color::Hsla {
    hue: 180.0,
    saturation: 0.66,
    lightness: 0.63,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const TIFFANY_BLUE_25: Color = Color::Hsla {
    hue: 180.0,
    saturation: 0.66,
    lightness: 0.63,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const TURQUOISE: Color = Color::Hsla {
    hue: 171.0,
    saturation: 0.80,
    lightness: 0.69,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const TURQUOISE_50: Color = Color::Hsla {
    hue: 171.0,
    saturation: 0.80,
    lightness: 0.69,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const TURQUOISE_25: Color = Color::Hsla {
    hue: 171.0,
    saturation: 0.80,
    lightness: 0.69,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const AQUAMARINE: Color = Color::Hsla {
    hue: 163.0,
    saturation: 1.0,
    lightness: 0.75,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const AQUAMARINE_50: Color = Color::Hsla {
    hue: 163.0,
    saturation: 1.0,
    lightness: 0.75,
    alpha: 0.50,
};
#[allow(dead_code)]
pub const AQUAMARINE_25: Color = Color::Hsla {
    hue: 163.0,
    saturation: 1.0,
    lightness: 0.75,
    alpha: 0.25,
};
#[allow(dead_code)]
pub const GREEN: Color = Color::Hsla {
    hue: 133.0,
    saturation: 1.0,
    lightness: 0.73,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const RED: Color = Color::Hsla {
    hue: 1.0,
    saturation: 1.0,
    lightness: 0.70,
    alpha: 1.0,
};
#[allow(dead_code)]
pub const GRAY: Color = Color::Hsla {
    hue: 1.0,
    saturation: 0.0,
    lightness: 0.80,
    alpha: 1.0,
};
