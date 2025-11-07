//! `PixelNode` struct - represents a node for a single pixel of a `Texture`.

use crate::{Color, component::{PositionComponent, SizeComponent}};


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

impl PositionComponent for RectangleNode {
    fn get_x(&self) -> usize {
        self.x
    }

    fn get_y(&self) -> usize {
        self.y
    }

    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
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


