//! Traits to define functionality of Pixel's objects (nodes). Functionality is
//! added using components.
//!
//! Each node in Pixl is defined by its *components*. A component allows
//! functions to handle logic of your node independently of the inner workings
//! of the node. For example, a function `move_left` could take any node which
//! implemented the `PositionComponent` trait.
//! This module defines traits that provide a common interface for all nodes in
//! Pixl. By implementing these traits, different types of nodes can be treated
//! generically, so code can be made much more reusable and modular.
//! Any nodes you create should implement the relevant traits defined in this
//! module, or implement other traits if you need more specific functionality
//! than is provided here.


#![expect(clippy::module_name_repetitions, reason = "components should be explicitly defined as Components to avoid name conflicts")]

use crate::{Color, Texture};


/// This node provides functions for drawing a node to the screen.
/// This should be implemented by any node that can be drawn to a texture.
pub trait DrawComponent {
    
    /// Draw the node to the surface of the passed `Texture` reference.
    /// The node (`self`) chooses how to draw itself onto the texture. This is
    /// generally based on the fields of the node drawn (e.g. the internal
    /// fields, such as position, size, color, etc.)
    fn draw(&self, texture: &mut Texture);
}

/// This trait provides functions for getting and setting the position of a node.
/// This should be implemented by any node that has a set position.
/// The position is the top-left corner of the node.
pub trait PositionComponent {

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

/// This trait provides functions for getting and setting the size of a node.
/// This should be implemented by any node that has a size that can be defined
/// by a rectangle dimensions (width and height).
pub trait SizeComponent {

    /// Get the width of the node.
    /// # Returns
    /// * `usize` - The width of the node.
    fn get_width(&self) -> usize;

    /// Get the height of the node.
    /// # Returns
    /// * `usize` - The height of the node.
    fn get_height(&self) -> usize;

    /// Set the width of the node.
    /// # Arguments
    /// * `width: usize` - The new width of the node.
    fn set_width(&mut self, width: usize);

    /// Set the height of the node.
    /// # Arguments
    /// * `height: usize` - The new height of the node.
    fn set_height(&mut self, height: usize);

    /// Set the size of the node.
    /// This method does not need to be implemented manually by a struct
    /// implementing this trait, as it, by default, calls the user-defined
    /// `set_width` and `set_height` methods.
    /// # Arguments
    /// * `size` - A (usize, usize) tuple containing the width and height of the
    ///   new size, in the form (width, height).
    fn set_size(&mut self, size: (usize, usize)) {
        self.set_width(size.0);
        self.set_height(size.1);
    }

    /// Get the size of the node as a (usize, usize) tuple in the form
    /// (width, height).
    /// This method does not need to be implemented manually by a struct
    /// implementing this trait, as it, by default, calls the user-defined
    /// `get_width` and `get_height` methods.
    /// # Returns
    /// * `(usize, usize)` - A tuple containing the width and height of the
    ///   node, in the form (width, height).
    fn get_size(&self) -> (usize, usize) {
        (self.get_width(), self.get_height())
    }
}

/// This trait provides functions for getting and setting the fill color of a
/// node.
/// This should be implemented by any node that has a fill color.
pub trait FillColorComponent {

    /// Get the fill color of the node.
    /// # Returns
    /// * `&Color` - A reference to the fill color of the node.
    fn get_fill_color(&self) -> &Color;

    /// Set the fill color of the node.
    /// # Arguments
    /// * `color: Color` - The new fill color of the node.
    fn set_fill_color(&mut self, color: Color);
}

/// This trait provides functions for getting and setting the stroke color of
/// a node.
/// This should be implemented by any node that has a stroke color.
pub trait NodeStrokeColor {

    /// Get the stroke (border/outline) color of the node.
    /// # Returns
    /// * `&Color` - A reference to the border color of the node.
    fn get_stroke_color(&self) -> &Color;

    /// Set the stroke (border/outline) color of the node.
    /// # Arguments
    /// * `color: Color` - The new border color of the node.
    fn set_stroke_color(&mut self, color: Color);
}

/// This trait provides functions for getting and setting the stroke width of
/// a node. The stroke width is the width of the border/outline of a node, or,
/// for nodes without a fill color, this may be the thickness of the node (e.g.
/// thickness of a line).
/// This should be implemented by any node that has a stroke width.
pub trait NodeStrokeWidth {

    /// Get the stroke (border/outline) width of the node.
    /// # Returns
    /// * `usize` - The border width of the node.
    fn get_stroke_width(&self) -> usize;

    /// Set the stroke (border/outline) width of the node.
    /// # Arguments
    /// * `width: usize` - The new border width of the node.
    fn set_stroke_width(&mut self, width: usize);
}

