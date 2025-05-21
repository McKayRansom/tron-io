use macroquad::color::{colors, Color};


const PLAYER_COLOR_LOOKUP: &[(Color, &str)] = &[
    // TEAM 0: BLUE
    (colors::BLUE, "Blue"),
    (colors::DARKBLUE, "Blue"),
    (colors::SKYBLUE, "Sky-Blue"),
    (colors::DARKPURPLE, "Dark-Purple"),
    // TEAM 1: RED
    (colors::RED, "Red"),
    (colors::PINK, "Pink"),
    (colors::MAROON, "Maroon"),
    (colors::PURPLE, "Purple"),
    // TEAM 2; GREEN
    (colors::GREEN, "Green"),
    (colors::LIME, "Lime"),
    (colors::DARKGREEN, "Dark-Green"),
    (colors::GREEN, "Green"),
    // TEAM 3: YELLOW
    (colors::YELLOW, "Yellow"),
    (colors::GOLD, "Yellow"),
    (colors::ORANGE, "Orange"),
    (colors::GOLD, "Yellow"),
    
    // UNUSED
    (colors::VIOLET, "Violet"),
    (colors::MAGENTA, "Magenta"),
];

pub fn get_color(id: u8) -> Color {
    PLAYER_COLOR_LOOKUP
        .get(id as usize)
        .map_or(colors::WHITE, |(color, _)| *color)
}

pub fn get_team_color(team: u8) -> (Color, &'static str) {
    PLAYER_COLOR_LOOKUP[(team * 4) as usize]
}

