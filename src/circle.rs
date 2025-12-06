//! The `CircleNode` struct, storing components to represent a filled circle on
//! a texture.

use crate::{Color, component::{DrawComponent, FillColorComponent, PositionComponent}};

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
    pub x: f64,

    /// The Y coordinate of the **center** of the circle.
    /// This is not the same value as is returned by the `get_y` method.
    pub y: f64,

    /// The radius of the circle (distance from the edge to the center).
    pub radius: f64,

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
    pub const fn new(x: f64, y: f64, radius: f64, fill_color: Color) -> CircleNode {
        CircleNode { x, y, radius, fill_color }
    }
}


impl DrawComponent for CircleNode {
    fn draw(&self, texture: &mut crate::Texture) {
        // This code is likely incredibly suboptimal. I chose to go with a
        // mathematical representation of a circle here, similarly to how a
        // raytracer would work, instead of a triangle-based rasterisation
        // approach, but there are likely many things that could be improved
        // here: both performance-wise and related to code readability.

        let radius_squared = cast::isize(self.radius.powi(2)).unwrap_or(isize::MAX);
        let center_x = cast::isize(self.x).unwrap_or(isize::MAX);
        let center_y = cast::isize(self.y).unwrap_or(isize::MAX);
        let left_x = cast::isize(self.x - self.radius).unwrap();
        let right_x = cast::isize(self.x + self.radius).unwrap();
        let top_y = cast::isize(self.y - self.radius).unwrap();
        let bottom_y = cast::isize(self.y + self.radius).unwrap();
        for y in top_y..=bottom_y {
            for x in left_x..=right_x {
                let dx = x.checked_sub(center_x).expect("pixl: under/overflow in circle drawing");
                let dy = y.checked_sub(center_y).expect("pixl: under/overflow in circle drawing");
                #[expect(clippy::arithmetic_side_effects, reason = "else unreadable")]
                if dx * dx + dy * dy <= radius_squared {
                    #[expect(clippy::as_conversions, clippy::cast_sign_loss, reason = "bounds are checked above")]
                    texture.set_pixel(x as usize, y as usize, self.fill_color)
                        .expect("pixl: failed to set pixel in circle drawing");
                }
            }
        }
    }
}

impl FillColorComponent for CircleNode {
    fn get_fill_color(&self) -> &Color {
        &self.fill_color
    }

    fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
    }
}

impl PositionComponent for CircleNode {
    fn get_x(&self) -> f64 {
        self.x - self.radius
    }

    fn get_y(&self) -> f64 {
        self.y - self.radius
    }

    fn set_x(&mut self, x: f64) {
        self.x = x + self.radius;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y + self.radius;
    }
}

