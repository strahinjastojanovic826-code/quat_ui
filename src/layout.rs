use crate::quat::Quat;

/// Layout behavior flags encoded in quaternary states:
/// - 00: Collapsed (hidden, takes zero space)
/// - 01: Relative (standard document flow)
/// - 10: Absolute (floating, ignores parent flow)
/// - 11: Expanded (fills all available space)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMode {
    pub horizontal: Quat,
    pub vertical: Quat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self {
            horizontal: Quat::V01, // Standard Relative width
            vertical: Quat::V01,   // Standard Relative height
        }
    }
}

impl LayoutMode {
    pub fn new(horizontal: Quat, vertical: Quat) -> Self {
        Self { horizontal, vertical }
    }

    /// Checks if the element is completely hidden from layout calculations.
    pub fn is_collapsed(&self) -> bool {
        self.horizontal == Quat::V00 && self.vertical == Quat::V00
    }

    /// Checks if the element expands to fill space.
    pub fn is_expanded(&self) -> bool {
        self.horizontal == Quat::V11 || self.vertical == Quat::V11
    }

    /// Resolves a `LayoutRect` from parent dimensions using all four quaternary axis modes:
    /// - `V00`: Collapsed (`0.0`)
    /// - `V01`: Relative (`0.5` of parent dimension)
    /// - `V10`: Absolute fallback (`64.0` units when no explicit size is provided)
    /// - `V11`: Expanded (full parent dimension)
    pub fn resolve_layout(&self, parent_width: f32, parent_height: f32) -> LayoutRect {
        if self.is_collapsed() {
            return LayoutRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }

        let width = match self.horizontal {
            Quat::V00 => 0.0,
            Quat::V01 => parent_width * 0.5,
            Quat::V10 => 64.0_f32.min(parent_width.max(0.0)),
            Quat::V11 => parent_width.max(0.0),
        };

        let height = match self.vertical {
            Quat::V00 => 0.0,
            Quat::V01 => parent_height * 0.5,
            Quat::V10 => 32.0_f32.min(parent_height.max(0.0)),
            Quat::V11 => parent_height.max(0.0),
        };

        LayoutRect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    /// Resolves a `LayoutRect` with explicit relative ratios (`0.0..=1.0` for `V01`)
    /// and explicit fixed sizes (`V10`).
    pub fn resolve_detailed(
        &self,
        parent_width: f32,
        parent_height: f32,
        rel_w: f32,
        rel_h: f32,
        abs_w: f32,
        abs_h: f32,
    ) -> LayoutRect {
        if self.is_collapsed() {
            return LayoutRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }

        let width = match self.horizontal {
            Quat::V00 => 0.0,
            Quat::V01 => parent_width * rel_w,
            Quat::V10 => abs_w,
            Quat::V11 => parent_width,
        };

        let height = match self.vertical {
            Quat::V00 => 0.0,
            Quat::V01 => parent_height * rel_h,
            Quat::V10 => abs_h,
            Quat::V11 => parent_height,
        };

        LayoutRect {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }
}