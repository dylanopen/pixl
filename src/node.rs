use crate::{Color, Texture};


pub trait NodeDraw {
    fn draw(&self, texture: &mut Texture);
}

