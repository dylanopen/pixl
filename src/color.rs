//! Module defining the Color struct and related functionality.

/// The `Color` struct represents a color with red, green, blue, and alpha
/// components.
///
/// Each component is an 8-bit unsigned integer (0-255).
/// This is used by the Texture struct to define the color of each pixel.
/// Currently, only the RGBA format is properly supported.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct Color {
    /// The red component of the color, ranging from 0 to 255.
    pub r: u8,
    /// The green component of the color, ranging from 0 to 255.
    pub g: u8,
    /// The blue component of the color, ranging from 0 to 255.
    pub b: u8,
    /// The alpha (transparency) component of the color, ranging from 0 to 255.
    /// 255 is fully opaque, and 0 is fully transparent.
    pub a: u8,
}

impl Color {
    /// Creates a new `Color` instance with the specified red, green, blue, and
    /// alpha components.
    /// # Arguments
    /// * `r` - The red component (0-255).
    /// * `g` - The green component (0-255).
    /// * `b` - The blue component (0-255).
    /// * `a` - The alpha component (0-255).
    /// # Returns
    /// A `Color` instance with the specified RGBA values.
    /// # Example
    /// ```rust
    /// let semi_transparent_red = Color::rgba(255, 0, 0, 128);
    /// ```
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }


    /// Creates a new `Color` instance with the specified red, green and blue
    /// components and full opacity (alpha = 255). This means no transparency.
    /// # Arguments
    /// * `r` - The red component (0-255).
    /// * `g` - The green component (0-255).
    /// * `b` - The blue component (0-255).
    /// # Returns
    /// A `Color` instance with the specified RGB values, with `a` set to 255.
    /// # Example
    /// ```rust
    /// let opaque_blue = Color::rgb(0, 63, 255);
    /// ```
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    /// Creates a new `Color` instance from a hexadecimal representation.
    /// The hex value should be in the format 0xRRGGBB, where RR is the red
    /// component, GG is the green component, and BB is the blue component.
    /// The alpha component is assumed to be 255 (fully opaque).
    /// # Arguments
    /// * `hex` - The hexadecimal color value (0xRRGGBB) as a u32.
    /// # Returns
    /// A `Color` instance with the corresponding RGB values and `a` set to 255.
    /// # Example
    /// ```rust
    /// let color = Color::from_hex(0xFF00FF); // magenta
    /// ```
    #[must_use]
    pub const fn from_hex(hex: u32) -> Color {
        // we assume no transparency
        let r = hex / 65536;
        let hex = hex - r*65536;
        let g = hex / 256;
        let hex = hex - g*256;
        let b = hex;
        let a = 255;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a
        }
    }

    /// Converts the `Color` instance to its hexadecimal representation.
    /// The resulting hex value is in the format 0xRRGGBB (as a u32), where RR
    /// is the red component, GG is the green component, and BB is the blue component.
    /// The alpha component is not included in the hex representation.
    /// # Arguments
    /// * `self` - The `Color` instance to convert.
    /// # Returns
    /// A u32 representing the color in hexadecimal format (0xRRGGBB).
    /// # Example
    /// ```rust
    /// let color = Color::rgb(255, 165, 0); // orange
    /// let hex = color.to_hex(); // hex will be 0xFFA500
    /// ```
    #[must_use]
    #[expect(clippy::as_conversions, reason = "cannot fail, and required in const fn")]
    #[expect(clippy::arithmetic_side_effects, reason = "no side effects as r,g,b <= 255")]
    pub const fn to_hex(&self) -> u32 {
        self.r as u32 * 0x10000 + self.g as u32 * 0x100 + self.b as u32
    }

    /// A constant for the fully opaque color black (RGB: 0, 0, 0).
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// A constant for the fully opaque color white (RGB: 255, 255, 255).
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// A constant for the fully opaque color red (RGB: 255, 0, 0).
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// A constant for the fully opaque color green (RGB: 0, 255, 0).
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// A constant for the fully opaque color blue (RGB: 0, 0, 255).
    pub const BLUE: Color = Color::rgb(0, 0, 255);

}
