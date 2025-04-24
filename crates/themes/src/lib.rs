// Theming system for VoidCLI
//
// This module provides functionality for managing terminal color schemes and styling

/// Represents an RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates a new color with the given RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Creates a new color from a hex string (e.g., "#ff0000" for red)
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        if !hex.starts_with('#') || hex.len() != 7 {
            return Err("Invalid hex color format");
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

        Ok(Color { r, g, b })
    }

    /// Returns the RGB components as a tuple
    pub fn as_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

/// Represents a terminal theme
#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    background: Color,
    foreground: Color,
    cursor: Color,
    selection: Color,
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
}

impl Theme {
    /// Creates a new theme with the given name and colors
    pub fn new(name: &str) -> Self {
        // Default to a dark theme
        Theme {
            name: name.to_string(),
            background: Color::new(0, 0, 0),
            foreground: Color::new(204, 204, 204),
            cursor: Color::new(255, 255, 255),
            selection: Color::new(64, 64, 64),
            black: Color::new(0, 0, 0),
            red: Color::new(204, 0, 0),
            green: Color::new(0, 204, 0),
            yellow: Color::new(204, 204, 0),
            blue: Color::new(0, 0, 204),
            magenta: Color::new(204, 0, 204),
            cyan: Color::new(0, 204, 204),
            white: Color::new(204, 204, 204),
        }
    }

    /// Returns the theme name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Provides a default dark theme
pub fn default_dark() -> Theme {
    Theme::new("Dark")
}

/// Provides a default light theme
pub fn default_light() -> Theme {
    let mut theme = Theme::new("Light");
    theme.background = Color::new(255, 255, 255);
    theme.foreground = Color::new(0, 0, 0);
    theme.selection = Color::new(179, 215, 255);
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.as_rgb(), (255, 0, 0));
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert_eq!(color.as_rgb(), (255, 0, 0));
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("Test Theme");
        assert_eq!(theme.name(), "Test Theme");
    }
}

