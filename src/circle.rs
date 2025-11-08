//! The `CircleNode` struct, storing components to represent a filled circle on
//! a texture.

use crate::{Color, component::DrawComponent};

/// A node representing a circle shape to be drawn on a texture.
/// It has a position (top left), size (width and height, must be equal and must
/// be twice the radius) and fill color.
/// ## Implemented components:
/// - `DrawComponent`
/// - `PositionComponent`
/// - `SizeComponent`
/// - `FillColorComponent`
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "struct should be called 'CircleNode' as it is standard.")]
pub struct CircleNode {

    /// The X coordinate of the **center** of the circle.
    /// This is not the same value as is returned by the `get_x` method.
    pub x: usize,

    /// The Y coordinate of the **center** of the circle.
    /// This is not the same value as is returned by the `get_y` method.
    pub y: usize,

    /// The radius of the circle (distance from the edge to the center).
    pub radius: usize,

    /// The color of the circle. It will be filled.
    pub fill_color: Color,
}


impl CircleNode {
    /// Create a new `CircleNode` with the specified `x`, `y`, `radius` and
    /// `fill_color`.
    /// # Parameters:
    /// - `x` - a usize containing the X coordinate of the center of the circle.
    /// - `y` - a usize containing the Y coordinate of the center of the circle.
    /// - `radius` - a usize containing the raidus of the circle.
    /// - `fill_color` - a `Color` containing the color to fill in the circle.
    /// # Returns
    /// A `CircleNode` with the specified properties.
    #[must_use]
    pub const fn new(x: usize, y: usize, radius: usize, fill_color: Color) -> CircleNode {
        CircleNode { x, y, radius, fill_color }
    }
}

