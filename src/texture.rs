//! A module defining a 2D texture structure with pixel manipulation
//! capabilities.
//! This struct is the basis for all of Pixl - everything is drawn to a Texture,
//! and a window simply displays a Texture.

use crate::color::Color;

/// A 2D texture represented as a grid of pixels, where each pixel is defined by
/// a `Color`.
/// The texture supports setting and getting pixel colors, as well as converting
/// the texture to a buffer of hexadecimal color values for usage in libraries
/// such as minifb.
pub struct Texture {
    /// A flat Vec of colors, representing the pixels in the texture.
    pixels: Vec<Color>,
    /// The width of the texture in pixels.
    width: usize,
    /// The height of the texture in pixels.
    height: usize,
}

impl Texture {
    /// Creates a new `Texture` instance with the specified width and height.
    /// All pixels are initialized to black.
    /// # Arguments
    /// * `width` - The width of the texture in pixels.
    /// * `height` - The height of the texture in pixels.
    /// # Returns
    /// A `Texture` instance with the specified dimensions, with all pixels set
    /// to black.
    /// # Example
    /// ```rust
    /// let texture = Texture::new(80, 60);
    /// ```
    pub fn new(width: usize, height: usize) -> Texture {
        Texture {
            pixels: vec![Color::BLACK; width*height],
            width, height,
        }
    }

    /// Gets the color of the pixel at the specified (x, y) coordinates.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// # Returns
    /// An `Option<Color>` containing the color of the pixel if the coordinates
    /// are within bounds, or `None` if they are out of bounds.
    /// # Example
    /// ```rust
    /// let color = texture.get_pixel(10, 10)
    ///     .expect("coordinates were out of bounds");
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        Some(*self.pixels.get(y*self.width + x)?)
    }

    /// Sets the color of the pixel at the specified (x, y) coordinates.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// * `color` - The color to set the pixel to.
    /// # Returns
    /// A `Result<>` indicating success or failure. Returns an error variant if
    /// the coordinates are out of bounds.
    /// # Example
    /// ```rust
    /// texture.set_pixel(10, 10, Color::rgb(255, 0, 0))
    ///     .expect("coordinates were out of bounds");
    /// ```
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), String> {
        if x >= self.width {
            return Err("Pixl: set_pixel: x was out of bounds for texture width".to_string());
        }
        if y >= self.height {
            return Err("Pixl: set_pixel: y was out of bounds for texture height".to_string());
        }
        self.pixels[y*self.width + x] = color;
        Ok(())
    }

    /// Gets the hexadecimal representation of the color of the pixel at the
    /// specified (x, y) coordinates.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// # Returns
    /// An `Option<u32>` containing the hexadecimal color of the pixel if the
    /// coordinates are within bounds, or `None` if they are out of bounds.
    /// # Example
    /// ```rust
    /// let hex_color = texture.get_pixel_hex(10, 10)
    ///     .expect("coordinates were out of bounds");
    /// // hex_color will be a u32 in 0xRRGGBB format
    /// ```
    pub fn get_pixel_hex(&self, x: usize, y: usize) -> Option<u32> {
        Some(self.pixels.get(y*self.width + x)?.to_hex())
    }

    /// Sets the color of the pixel at the specified (x, y) coordinates using
    /// a hexadecimal color value.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// * `color` - The hexadecimal color value to set the pixel to as a u32.
    /// # Returns
    /// A `Result<>` indicating success or failure. Returns an error variant if
    /// the coordinates are out of bounds. No return value on success.
    /// # Example
    /// ```rust
    /// texture.set_pixel_hex(10, 10, 0xFF0000)
    ///    .expect("coordinates were out of bounds");
    /// ```
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

    /// Converts the texture to a buffer of hexadecimal color values.
    /// This is useful for libraries like minifb that require a buffer of u32
    /// color values.
    /// # Returns
    /// A `Vec<u32>` containing the hexadecimal color values of all pixels in
    /// the texture.
    /// # Example
    /// ```rust
    /// let buffer = texture.to_u32_buffer();
    /// for hex_color in buffer {
    ///     ...
    /// }
    /// ```
    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buf = Vec::with_capacity(self.width*self.height);
        for pixel in &self.pixels {
            buf.push(pixel.to_hex());
        }
        buf
    }

    /// Gets the width of the texture in pixels.
    /// # Returns
    /// This simply returns the `width` field of the `Texture` struct.
    /// # Example
    /// ```rust
    /// let width = texture.get_width();
    /// ```
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the texture in pixels.
    /// # Returns
    /// This simply returns the `height` field of the `Texture` struct.
    /// # Example
    /// ```rust
    /// let height = texture.get_height();
    /// ```
    pub fn get_height(&self) -> usize {
        self.height
    }
}

