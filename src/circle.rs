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


impl DrawComponent for CircleNode {
    #[expect(clippy::similar_names, reason = "no clippy, x and y are not similar.")]
    fn draw(&self, texture: &mut crate::Texture) {
        let start_x = self.x.saturating_sub(self.radius);
        let start_y = self.y.saturating_sub(self.radius);
        let end_x = self.x.saturating_add(self.radius);
        let end_y = self.y.saturating_add(self.radius);
        let radius_squared = self.radius
            .checked_pow(2).expect("overflow: radius ^ 2")
            .try_into().expect("overflow: converting radius from usize to isize");
        let xi: isize = self.x.try_into().expect("overflow: circle's X is too large");
        let yi: isize = self.x.try_into().expect("overflow: circle's X is too large");
        for py in start_y..=end_y {
            let pyi: isize = py.try_into().expect("overflow: looping py was too large in circle");
            let circle_equation_y_part = pyi
                .checked_sub(yi).expect("overflow: pyi*yi")
                .checked_pow(2).expect("overflow: pyi*yi ^ 2");
            for px in start_x..=end_x {
                let pxi: isize = px.try_into().expect("overflow: looping px was too large in circle");
                let circle_equation_x_part = pxi.checked_sub(xi).expect("overflow: pxi*xi")
                .checked_pow(2).expect("overflow: pxi*xi ^ 2");
                #[expect(clippy::arithmetic_side_effects, reason = "unreadable otherwise")]
                if circle_equation_x_part + circle_equation_y_part <= radius_squared {
                    texture.set_pixel(px, py, self.fill_color).unwrap_or(());
                }
            }
        }
    }
}

