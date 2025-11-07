//! `LineNode` struct - represents a node for a rectangle shape in a
//! texture.

use anyhow::Error;

use crate::{Color, Texture, component::DrawComponent};

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

/// Draw a line on the given texture using Bresenham's line algorithm.
/// # Parameters
/// - `texture`: The texture to draw the line on.
/// - `x1`: The x-coordinate of the start point of the line.
/// - `y1`: The y-coordinate of the start point of the line.
/// - `x2`: The x-coordinate of the end point of the line.
/// - `y2`: The y-coordinate of the end point of the line.
/// - `color`: The color of the line.
#[expect(clippy::single_call_fn, reason = "due to the unchangable return type of DrawComponent::draw")]
#[expect(clippy::arithmetic_side_effects, reason = "checked arithmetic here is completely unreadable")]
#[expect(clippy::cast_sign_loss, reason = "cannot fail, and required in line drawing algorithm")]
#[expect(clippy::as_conversions, reason = "cannot fail, and required in line drawing algorithm")]
#[expect(clippy::cast_possible_wrap, reason = "cannot fail, and required in line drawing algorithm")]
fn draw_line(texture: &mut Texture, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
    // Bresenham's line algorithm
    // Note: I personally hate AI-generated code, but this implementation was
    // mostly written by AI to save time. Issues will be fixed as they are
    // found, including any performance issues.

    let x1i = x1 as isize;
    let y1i = y1 as isize;
    let x2i = x2 as isize;
    let y2i = y2 as isize;
    
    let dx = (x2i - x1i).abs();
    let dy = -(y2i - y1i).abs();
    let sx: isize = if x1 < x2 { 1 } else { -1 };
    let sy: isize = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x1 as isize;
    let mut y = y1 as isize;

    loop {
        if x >= 0 && y >= 0 {
            #[expect(clippy::unwrap_used, reason = "bounds are already checked manually")]
            texture.set_pixel(x as usize, y as usize, color).unwrap();
        }
        if x == x2i && y == y2i {
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

impl DrawComponent for LineNode {
    fn draw(&self, texture: &mut crate::Texture) {
        draw_line(texture, self.x1, self.y1, self.x2, self.y2, self.color);
    }
}

