use crate::quat::Quat;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub background_disabled: Color, // V00 (00)
    pub background_normal: Color,   // V01 (01)
    pub background_hovered: Color,  // V10 (10)
    pub background_active: Color,   // V11 (11)
}

impl Theme {
    pub fn default_dark() -> Self {
        Self {
            background_disabled: Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
            background_normal:   Color { r: 0.3, g: 0.3, b: 0.35, a: 1.0 },
            background_hovered:  Color { r: 0.4, g: 0.4, b: 0.45, a: 1.0 },
            background_active:   Color { r: 0.2, g: 0.5, b: 0.8, a: 1.0 },
        }
    }

    /// Automatically maps a Quat state to a theme color
    pub fn resolve_color(&self, state: Quat) -> Color {
        match state {
            Quat::V00 => self.background_disabled,
            Quat::V01 => self.background_normal,
            Quat::V10 => self.background_hovered,
            Quat::V11 => self.background_active,
        }
    }
}