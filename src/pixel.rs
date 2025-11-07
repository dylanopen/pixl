//! `PixelNode` struct - represents a node for a single pixel of a `Texture`.


use crate::Color;
use crate::component::{DrawComponent, FillColorComponent, PositionComponent};


/// A node representing a single pixel in a texture.
/// Implemented components:
/// - `DrawComponent`
/// - `PositionComponent`
/// - `FillColorComponent`
#[expect(clippy::module_name_repetitions, reason = "struct should be called 'PixelNode' as it is standard.")]
#[non_exhaustive]
pub struct PixelNode {
    /// The x-coordinate of the pixel on the texture, as a `usize`.
    pub x: usize,
    /// The y-coordinate of the pixel on the texture.
    pub y: usize,
    /// The color of the pixel. This (in the future) may have an alpha channel.
    pub color: Color,
}

impl PixelNode {
    /// Create a new `Pixel` node with the specified `x` and `y` coordinates and
    /// the pixel's `color`.
    #[must_use]
    pub const fn new(x: usize, y: usize, color: Color) -> PixelNode {
        PixelNode { x, y, color }
    }
}

impl DrawComponent for PixelNode {
    fn draw(&self, texture: &mut crate::Texture) {
        texture.set_pixel(self.x, self.y, self.color).unwrap_or(());
        // TODO: error handling
        // TODO: consider the alpha channel of `self.color` and blend between
        // the existing color at (x,y) and the new color, depending on the alpha
        // channel.
    }
}

impl PositionComponent for PixelNode {
    fn get_x(&self) -> usize {
        self.x
    }

    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn get_y(&self) -> usize {
        self.y
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

impl FillColorComponent for PixelNode {
    fn get_fill_color(&self) -> &Color {
        &self.color
    }

    fn set_fill_color(&mut self, color: Color) {
        self.color = color;
    }
}

