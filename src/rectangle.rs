//! `RectangleNode` struct - represents a node for a rectangle shape in a
//! texture.

use crate::{Color, component::{DrawComponent, FillColorComponent, PositionComponent, SizeComponent}};


/// A node representing a rectangle shape to be drawn on a texture.
/// The rectangle has flat sides, parallel to the texture edges.
/// ## Implemented components:
/// - `DrawComponent`
/// - `PositionComponent`
/// - `SizeComponent`
/// - `FillColorComponent`
#[non_exhaustive]
#[expect(clippy::module_name_repetitions, reason = "struct should be called 'RectangleNode' as it is standard.")]
pub struct RectangleNode {

    /// The x-coordinate of the top-left corner of the rectangle.
    /// Assumes (0,0) is the top-left corner of the texture.
    pub x: f64,

    /// The y-coordinate of the top-left corner of the rectangle.
    /// Assumes (0,0) is the top-left corner of the texture.
    pub y: f64,

    /// The width of the rectangle.
    pub width: usize,

    /// The height of the rectangle.
    pub height: usize,

    /// The fill color of the rectangle.
    /// This (in the future) may have an alpha channel.
    pub fill_color: Color,
}

impl RectangleNode {
    /// Create a new `RectangleNode` with the specified position, size, and fill
    /// color.
    /// # Parameters
    /// - `x`: The x-coordinate of the top-left corner of the rectangle.
    /// - `y`: The y-coordinate of the top-left corner of the rectangle.
    /// - `width`: The width of the rectangle, in pixels.
    /// - `height`: The height of the rectangle in pixels.
    /// - `fill_color`: The fill color of the rectangle.
    /// # Returns
    /// A new `RectangleNode` instance with the specified properties.
    #[must_use]
    pub const fn new(
        x: f64, y: f64, width: usize, height: usize, fill_color: Color
    ) -> RectangleNode {
        RectangleNode { x, y, width, height, fill_color }
    }
}

impl PositionComponent for RectangleNode {
    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

impl SizeComponent for RectangleNode {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}

impl FillColorComponent for RectangleNode {
    fn get_fill_color(&self) -> &Color {
        &self.fill_color
    }

    fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
    }
}

impl DrawComponent for RectangleNode {
    fn draw(&self, texture: &mut crate::Texture) {
        for dy in 0..self.height {
            for dx in 0..self.width {
                let px = self.x as usize + dx;
                let py = self.y as usize + dy;
                texture.set_pixel(px, py, self.fill_color)
                    .unwrap_or(());
            }
        }
    }
}

