//! `PixelNode` struct - represents a node for a single pixel of a `Texture`.

use crate::Color;


/// A node representing a rectangle shape to be drawn on a texture.
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "struct should be called 'RectangleNode' as it is standard.")]
pub struct RectangleNode {
    /// The x-coordinate of the top-left corner of the rectangle.
    /// Assumes (0,0) is the top-left corner of the texture.
    pub x: usize,
    /// The y-coordinate of the top-left corner of the rectangle.
    /// Assumes (0,0) is the top-left corner of the texture.
    pub y: usize,
    /// The width of the rectangle.
    pub width: usize,
    /// The height of the rectangle.
    pub height: usize,
    /// The fill color of the rectangle.
    /// This (in the future) may have an alpha channel.
    pub fill_color: Color,
}

