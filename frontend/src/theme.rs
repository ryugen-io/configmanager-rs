use ratzilla::ratatui::style::Color;

// Catppuccin Mocha color palette (official RGB values)
pub struct Theme;

impl Theme {
    // Base colors (Catppuccin Mocha - only used colors)
    pub const LAVENDER: Color = Color::Rgb(183, 189, 248); // Primary accent
    pub const MAUVE: Color = Color::Rgb(198, 160, 246); // Purple/Magenta
    pub const SAPPHIRE: Color = Color::Rgb(125, 196, 228); // Blue
    pub const GREEN: Color = Color::Rgb(166, 218, 149); // Green
    pub const YELLOW: Color = Color::Rgb(238, 212, 159); // Yellow
    pub const PEACH: Color = Color::Rgb(245, 169, 127); // Orange
    pub const TEXT: Color = Color::Rgb(202, 211, 245); // Main text
    pub const SUBTEXT0: Color = Color::Rgb(165, 173, 203); // Dimmed text
    pub const OVERLAY1: Color = Color::Rgb(128, 135, 162); // Medium dimmed
    pub const MANTLE: Color = Color::Rgb(30, 32, 48); // Darker background

    // Semantic colors
    pub const ACCENT: Color = Self::LAVENDER;
    pub const SELECTED: Color = Self::YELLOW;
    pub const MODIFIED: Color = Self::PEACH;
    pub const SUCCESS: Color = Self::GREEN;
    pub const NORMAL_MODE: Color = Self::SAPPHIRE;
    pub const INSERT_MODE: Color = Self::MAUVE;
    pub const DIM: Color = Self::SUBTEXT0;
}
