use crate::{Color, Texture};


pub trait NodeDraw {
    fn draw(&self, texture: &mut Texture);
}

pub trait NodePosition {
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
    fn set_x(&mut self, x: usize);
    fn set_y(&mut self, y: usize);

    fn set_position(&mut self, x: usize, y: usize) {
        self.set_x(x);
        self.set_y(y);
    }
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

