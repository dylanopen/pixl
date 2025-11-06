//! Contains the `Window` struct and its associated methods for creating and
//! managing an operating system window that can display a `Texture`.
//! It currently uses the `minifb` crate for window management and rendering.

use crate::texture::Texture;

/// The `Window` struct represents an operating system window which can display
/// a single `Texture`.
/// /// It uses the `minifb` crate for window management and rendering.
/// # Example
/// ```rust
/// use pixl::{window::Window, texture::Texture, color::Color};
/// let mut window = Window::new(800, 600, "Hello, Pixl!".to_string());
/// let mut texture = Texture::new(80, 60); // doesn't have to match window size
/// // Set some pixels in the texture:
/// texture.set_pixel(10, 10, Color::rgb(255, 0, 0)).unwrap(); // red pixel
/// texture.set_pixel(20, 20, Color::rgb(0, 255, 0)).unwrap(); // green pixel
/// texture.set_pixel(30, 30, Color::rgb(0, 0, 255)).unwrap(); // blue pixel
/// while window.is_open() {
///     window.draw(&texture);
/// }
/// ```
pub struct Window {
    /// The underlying minifb window instance.
    /// This field is private, as it does not need to be used outside of the 
    /// wrapper methods in this `Window` struct..
    minifb_window: minifb::Window,
}

impl Window {
    /// Creates a new `Window` instance with the specified width, height, and
    /// title.
    /// > The window width does **not** have to match the texture width that is
    /// > later drawn to it. Same for the height.
    /// # Arguments
    /// * `width` - The width of the window in pixels.
    /// * `height` - The height of the window in pixels.
    /// * `title` - The title of the window as a `String`.
    /// # Returns
    /// A `Window` instance with the specified dimensions and title.
    /// # Panics
    /// Panics if the window cannot be created by minifb (e.g., due to invalid
    /// dimensions or system limitations).
    /// This may also panic if the system is out of memory, or system packages
    /// like libglfw3 are missing.
    /// # Example
    /// ```rust
    /// use pixl::{window::Window, texture::Texture};
    /// let mut window = Window::new(800, 600, "Hello, Pixl!");
    /// let mut texture = Texture::new(80, 60); // doesn't have to match window size
    /// // ...
    /// ```
    #[must_use]
    pub fn new(width: usize, height: usize, title: &str) -> Window {
        #[expect(clippy::expect_used, reason = "the program is unable to continue without a window")]
        let mut minifb_window = minifb::Window::new(
            title, width, height,
            minifb::WindowOptions::default()
        ).expect("Pixl: failed to create window");

        minifb_window.set_target_fps(60);

        Window {
            minifb_window,
        }
    }

    /// Checks if the window is still open.
    /// # Returns
    /// A `bool` indicating whether the window is open (`true`) or closed
    /// (`false`).
    /// # Example
    /// ```rust
    /// use pixl::window::Window;
    /// let mut window = Window::new(800, 600, "Hello, Pixl!".to_string());
    /// while window.is_open() {
    ///    // ...
    /// }
    /// ```
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.minifb_window.is_open()
    }

    /// Draws the given `Texture` to the window.
    /// > The texture's dimensions do **not** have to match the window's. If
    /// > they do not match, it will be upscaled or downscaled to fit the
    /// > window.
    /// # Arguments
    /// * `texture` - A reference to the `Texture` to be drawn.
    /// # Errors
    /// Returns an Err variant if the texture cannot be drawn to the window by
    /// minifb.
    /// # Example
    /// ```rust
    /// use pixl::{window::Window, texture::Texture};
    /// let mut window = Window::new(800, 600, "Hello, Pixl!");
    /// let mut texture = Texture::new(80, 60);
    /// while window.is_open() {
    ///     window.draw(&texture).unwrap();
    /// }
    /// ```
    pub fn draw(&mut self, texture: &Texture) -> Result<(), minifb::Error> {
        self.minifb_window.update_with_buffer(&texture.to_u32_buffer(), texture.get_width(), texture.get_height())
    }
}

