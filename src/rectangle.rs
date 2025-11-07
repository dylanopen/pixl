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
                let px = self.x.saturating_add(dx);
                let py = self.y.saturating_add(dy);
                texture.set_pixel(px, py, self.fill_color)
                    .unwrap_or(());
            }
        }
    }
}

