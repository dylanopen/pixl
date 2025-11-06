#![expect(clippy::module_name_repetitions, reason = "nodes should be explicitly defined as Node to avoid name conflicts")]

use crate::{Color, Texture};



/// This node provides functions for drawing a node to the screen.
/// This should be implemented by any node that can be drawn to a texture.
pub trait NodeDraw {
    
    /// Draw the node to the surface of the passed `Texture` reference.
    /// The node (`self`) chooses how to draw itself onto the texture. This is
    /// generally based on the fields of the node drawn (e.g. the internal
    /// fields, such as position, size, color, etc.)
    fn draw(&self, texture: &mut Texture);
}

/// This trait provides functions for getting and setting the position of a node.
/// This should be implemented by any node that has a set position.
/// The position is the top-left corner of the node.
pub trait NodePosition {

    /// Get the X position of the node.
    /// This function should return the X coordinate of the top-left corner of
    /// the node's position.
    /// # Returns
    /// * `usize` - The X coordinate of the top-left corner of the node.
    fn get_x(&self) -> usize;

    /// Get the Y position of the node.
    /// This function should return the Y coordinate of the top-left corner of
    /// the node's position.
    /// # Returns
    /// * `usize` - The Y coordinate of the top-left corner of the node.
    fn get_y(&self) -> usize;

    /// Set the X position of the node.
    /// The position is the top-left corner of the node, so the passed `x` value
    /// should be the X coordinate of the top-left corner.
    /// # Arguments
    /// * `x` - The new X coordinate of the top-left corner of the node
    fn set_x(&mut self, x: usize);

    /// Set the Y position of the node.
    /// The position is the top-left corner of the node, so the passed `y` value
    /// should be the Y coordinate of the top-left corner.
    /// # Arguments
    /// * `y` - The new Y coordinate of the top-left corner of the node
    fn set_y(&mut self, y: usize);

    /// Set the position of the node to the given `(x, y)` tuple.
    /// The position is the top-left corner of the node.
    /// # Arguments
    /// * `position` - A tuple containing the X and Y coordinates of the new
    ///   position, in the form (x, y).
    fn set_position(&mut self, position: (usize, usize)) {
        self.set_x(position.0);
        self.set_y(position.1);
    }

    /// Get the coordinates of the top-left corner of the node as a
    /// (usize, usize) tuple in the form (x, y).
    /// This method does not need to be implemented manually by a struct
    /// implementing this trait, as it, by default, calls the user-defined
    /// `get_x` and `get_y` methods.
    /// # Returns
    /// * `(usize, usize)` - A tuple containing the X and Y coordinates of the
    ///   top-left corner of the node, in the form (x, y).
    fn get_position(&self) -> (usize, usize) {
        (self.get_x(), self.get_y())
    }
}

pub trait NodeSize {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn set_width(&mut self, width: usize);
    fn set_height(&mut self, height: usize);

    fn set_size(&mut self, width: usize, height: usize) {
        self.set_width(width);
        self.set_height(height);
    }
    fn get_size(&self) -> (usize, usize) {
        (self.get_width(), self.get_height())
    }
}

pub trait NodeFillColor {
    fn get_fill_color(&self) -> &Color;
    fn set_fill_color(&mut self, color: Color);
}

pub trait NodeStrokeColor {
    fn get_border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color);
}

pub trait NodeStrokeWidth {
    fn get_border_width(&self) -> usize;
    fn set_border_width(&mut self, width: usize);
}

