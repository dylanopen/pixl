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

