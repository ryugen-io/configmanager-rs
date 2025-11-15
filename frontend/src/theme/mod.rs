pub mod container_list;
pub mod editor;
pub mod file_list;
pub mod menu;
pub mod status_line;

use ratzilla::ratatui::style::{Color, Modifier, Style};

/// # Theme Design Pattern
///
/// This module defines the standardized theme system for the Config Manager TUI.
///
/// ## Architecture
///
/// The theme system consists of three layers:
///
/// 1. **Base Colors** (`Theme` struct): Raw RGB colors loaded from `theme.toml` at build time
/// 2. **Semantic Colors** (`Theme` constants): Meaningful color mappings (e.g., ACCENT, ERROR)
/// 3. **Component Themes** (e.g., `FileListTheme`, `MenuTheme`): Style builders for UI components
///
/// ## Design Principles
///
/// All component theme modules should follow these conventions:
///
/// ### Standard Style Methods
///
/// Every focusable widget should implement:
/// - `border_focused()` - Border style when the widget has focus
/// - `border_unfocused()` - Border style when the widget is not focused
///
/// Every list-like widget should implement:
/// - `normal_item_style()` - Style for regular list items
/// - `selected_item_style()` - Style for the selected/highlighted item
/// - `selected_prefix()` - Text prefix for selected items (e.g., "> ")
///
/// ### Semantic Style Methods
///
/// Use descriptive method names that indicate purpose:
/// - `title_style()` - For widget titles
/// - `label_style()` - For field labels
/// - `value_style()` - For field values
/// - `error_style()` - For error messages
/// - `success_style()` - For success messages
/// - `dim_style()` or `muted_style()` - For less important text
///
/// ### Color Usage Guidelines
///
/// - **ACCENT**: Primary brand color, used for focused borders and important UI elements
/// - **SELECTED**: Highlight color for selected items (usually yellow/gold)
/// - **MODIFIED**: Indicator for modified/dirty state (usually orange/peach)
/// - **SUCCESS**: Positive feedback and successful operations (green)
/// - **ERROR**: Error messages and failed operations (red)
/// - **NORMAL_MODE / INSERT_MODE**: Vim mode indicators (blue/purple)
/// - **TEXT**: Default text color (white/light gray)
/// - **DIM / SUBTEXT0**: Secondary text, labels, help text
/// - **OVERLAY1**: Inactive/disabled elements
/// - **SURFACE1**: Subtle background highlights
/// - **MANTLE**: Background for status bars and panels
///
/// ### Example Component Theme
///
/// ```rust
/// use super::Theme;
/// use ratzilla::ratatui::style::{Modifier, Style};
///
/// pub struct MyWidgetTheme;
///
/// impl MyWidgetTheme {
///     // Standard border styles
///     pub fn border_focused() -> Style {
///         Style::default().fg(Theme::ACCENT)
///     }
///
///     pub fn border_unfocused() -> Style {
///         Style::default().fg(Theme::OVERLAY1)
///     }
///
///     // List item styles
///     pub fn normal_item_style() -> Style {
///         Style::default().fg(Theme::TEXT)
///     }
///
///     pub fn selected_item_style() -> Style {
///         Style::default()
///             .fg(Theme::SELECTED)
///             .add_modifier(Modifier::BOLD)
///     }
///
///     pub fn selected_prefix() -> &'static str {
///         "> "
///     }
///
///     // Component-specific styles
///     pub fn my_custom_style() -> Style {
///         Style::default().fg(Theme::SUCCESS)
///     }
/// }
/// ```
///
/// ## Per-File Theme Support
///
/// Config files can specify custom theme variants:
/// - Backend: Add `theme` field to `ConfigFile` in `config-manager.toml`
/// - Frontend: Theme variants can override specific colors while using the same base structure
/// - Theme switching is applied when loading a file in the editor
///
/// ## Modifying the Theme
///
/// To change colors:
/// 1. Edit `frontend/theme.toml` to adjust RGB values
/// 2. Rebuild the frontend: `just build-frontend` or `./rebuild.sh --frontend-only`
/// 3. Colors are injected at compile time via `build.rs`
///
pub struct Theme;

// Helper macro to parse RGB from build-time environment variable
macro_rules! rgb_from_env {
    ($env:literal) => {{
        const RGB_STR: &str = env!($env);
        const fn parse_rgb(s: &str) -> (u8, u8, u8) {
            let bytes = s.as_bytes();
            let mut r = 0u8;
            let mut g = 0u8;
            let mut idx = 0;
            let mut current = 0;
            let mut part = 0;

            while idx < bytes.len() {
                if bytes[idx] >= b'0' && bytes[idx] <= b'9' {
                    current = current * 10 + (bytes[idx] - b'0') as u8;
                } else if bytes[idx] == b',' {
                    match part {
                        0 => r = current,
                        1 => g = current,
                        _ => {}
                    }
                    current = 0;
                    part += 1;
                }
                idx += 1;
            }
            let b = current;
            (r, g, b)
        }

        const RGB: (u8, u8, u8) = parse_rgb(RGB_STR);
        Color::Rgb(RGB.0, RGB.1, RGB.2)
    }};
}

impl Theme {
    // Base colors loaded from theme.toml
    pub const LAVENDER: Color = rgb_from_env!("THEME_COLOR_LAVENDER");
    pub const MAUVE: Color = rgb_from_env!("THEME_COLOR_MAUVE");
    pub const SAPPHIRE: Color = rgb_from_env!("THEME_COLOR_SAPPHIRE");
    pub const GREEN: Color = rgb_from_env!("THEME_COLOR_GREEN");
    pub const YELLOW: Color = rgb_from_env!("THEME_COLOR_YELLOW");
    pub const PEACH: Color = rgb_from_env!("THEME_COLOR_PEACH");
    pub const RED: Color = rgb_from_env!("THEME_COLOR_RED");
    pub const TEXT: Color = rgb_from_env!("THEME_COLOR_TEXT");
    pub const SUBTEXT0: Color = rgb_from_env!("THEME_COLOR_SUBTEXT0");
    pub const OVERLAY1: Color = rgb_from_env!("THEME_COLOR_OVERLAY1");
    pub const SURFACE1: Color = rgb_from_env!("THEME_COLOR_SURFACE1");
    pub const MANTLE: Color = rgb_from_env!("THEME_COLOR_MANTLE");

    // Semantic colors (mapped to base colors)
    pub const ACCENT: Color = Self::LAVENDER;
    pub const SELECTED: Color = Self::YELLOW;
    pub const MODIFIED: Color = Self::PEACH;
    pub const SUCCESS: Color = Self::GREEN;
    pub const ERROR: Color = Self::RED;
    pub const NORMAL_MODE: Color = Self::SAPPHIRE;
    pub const INSERT_MODE: Color = Self::MAUVE;
    pub const DIM: Color = Self::SUBTEXT0;
}

// =============================================================================
// Common Style Builders
// =============================================================================
//
// These provide reusable style patterns that component themes can use.
// This ensures consistency across the application.

impl Theme {
    /// Standard style for focused borders
    pub fn standard_border_focused() -> Style {
        Style::default().fg(Self::ACCENT)
    }

    /// Standard style for unfocused borders
    pub fn standard_border_unfocused() -> Style {
        Style::default().fg(Self::OVERLAY1)
    }

    /// Standard style for selected/highlighted items
    pub fn standard_selected_item() -> Style {
        Style::default()
            .fg(Self::SELECTED)
            .add_modifier(Modifier::BOLD)
    }

    /// Standard style for normal items
    pub fn standard_normal_item() -> Style {
        Style::default().fg(Self::TEXT)
    }

    /// Standard style for titles
    pub fn standard_title() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Standard style for labels (dimmed text)
    pub fn standard_label() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// Standard style for values
    pub fn standard_value() -> Style {
        Style::default().fg(Self::YELLOW)
    }

    /// Standard background style
    pub fn standard_background() -> Style {
        Style::default().bg(Self::MANTLE)
    }

    /// Standard highlight background
    pub fn standard_highlight_bg() -> Style {
        Style::default().bg(Self::SURFACE1)
    }
}

// =============================================================================
// Default Theme Patterns
// =============================================================================

/// Common prefix for selected items in lists
pub const SELECTED_PREFIX: &str = "> ";

/// Common prefix for normal items in lists
pub const NORMAL_PREFIX: &str = "  ";
