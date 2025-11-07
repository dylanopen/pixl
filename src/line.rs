//! `LineNode` struct - represents a node for a rectangle shape in a
//! texture.

use crate::{Color, component::DrawComponent};

/// A node representing a line shape in a texture.
/// Implemented components:
/// - `DrawComponent`
/// - `StrokeColorComponent`
/// - `StrokeWidthComponent`
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "struct should be called 'LineNode' as it is standard.")]
pub struct LineNode {
    /// The x-coordinate of the start point of the line.
    pub x1: usize,

    /// The y-coordinate of the start point of the line.
    pub y1: usize,

    /// The x-coordinate of the end point of the line.
    pub x2: usize,

    /// The y-coordinate of the end point of the line.
    pub y2: usize,

    /// The color of the line.
    pub color: Color,
}

impl LineNode {
    /// Create a new `LineNode` with the specified start and end points and color.
    /// # Parameters
    /// - `x1`: The x-coordinate of the start point of the line.
    /// - `y1`: The y-coordinate of the start point of the line.
    /// - `x2`: The x-coordinate of the end point of the line.
    /// - `y2`: The y-coordinate of the end point of the line.
    /// - `color`: The stroke color of the line.
    /// # Returns
    /// A new `LineNode` instance with the specified properties.
    #[must_use]
    pub const fn new(
        x1: usize, y1: usize, x2: usize, y2: usize, color: Color
    ) -> LineNode {
        LineNode { x1, y1, x2, y2, color }
    }
}

impl DrawComponent for LineNode {
    fn draw(&self, texture: &mut crate::Texture) {
        // Bresenham's line algorithm
        let dx = (self.x2 as isize - self.x1 as isize).abs();
        let dy = -(self.y2 as isize - self.y1 as isize).abs();
        let sx = if self.x1 < self.x2 { 1 } else { -1 };
        let sy = if self.y1 < self.y2 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = self.x1 as isize;
        let mut y = self.y1 as isize;

        loop {
            if x >= 0 && y >= 0 {
                texture.set_pixel(x as usize, y as usize, self.color).unwrap_or(());
            }
            if x == self.x2 as isize && y == self.y2 as isize {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

