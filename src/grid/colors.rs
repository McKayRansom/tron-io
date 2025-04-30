use macroquad::color::{colors, Color};


const PLAYER_COLOR_LOOKUP: &[(Color, &str)] = &[
    (colors::YELLOW, "Yellow"),
    (colors::GOLD, "Yellow"),
    (colors::ORANGE, "Orange"),
    (colors::PINK, "Pink"),
    (colors::RED, "Red"),
    (colors::MAROON, "Maroon"),
    (colors::GREEN, "Green"),
    (colors::LIME, "Lime"),
    (colors::DARKGREEN, "Dark-Green"),
    (colors::SKYBLUE, "Sky-Blue"),
    (colors::BLUE, "Blue"),
    (colors::DARKBLUE, "Blue"),
    (colors::PURPLE, "Purple"),
    (colors::VIOLET, "Violet"),
    (colors::DARKPURPLE, "Dark-Purple"),
    (colors::MAGENTA, "Magenta"),
];

pub const DEFAULT_COLOR: u8 = 6; // Green

pub fn get_color(id: u8) -> Color {
    PLAYER_COLOR_LOOKUP
        .get(id as usize)
        .map_or(colors::WHITE, |(color, _)| *color)
}