//! `LineNode` struct - represents a node for a rectangle shape in a
//! texture.

use crate::Color;

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


