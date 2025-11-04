use crate::color::Color;

pub struct Texture {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Texture {
        Texture {
            pixels: vec![Color::BLACK; width*height],
            width, height,
        }
    }

    pub fn get_pixel_hex(&self, x: usize, y: usize) -> Option<u32> {
        Some(self.pixels.get(y*self.width + x)?.to_hex())
    }

    pub fn set_pixel_hex(&mut self, x: usize, y: usize, color: u32) -> Result<(), String> {
        if x >= self.width {
            return Err("Pixl: set_pixel_hex: x was out of bounds for texture width".to_string());
        }
        if y >= self.height {
            return Err("Pixl: set_pixel_hex: y was out of bounds for texture height".to_string());
        }
        self.pixels[y*self.width + x] = Color::from_hex(color);
        Ok(())
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buf = Vec::with_capacity(self.width*self.height);
        for pixel in &self.pixels {
            buf.push(pixel.to_hex());
        }
        buf
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

