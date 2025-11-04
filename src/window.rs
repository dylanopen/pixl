use crate::texture::Texture;

pub struct Window {
    minifb_window: minifb::Window,
}

impl Window {
    pub fn new(width: usize, height: usize, title: String) -> Window {
        let mut minifb_window = minifb::Window::new(
            &title, width, height,
            minifb::WindowOptions::default()
        ).expect("Pixl: failed to create window");

        minifb_window.set_target_fps(60);

        Window {
            minifb_window,
        }
    }

    pub fn is_open(&self) -> bool {
        self.minifb_window.is_open()
    }

    pub fn draw(&mut self, texture: &Texture) {
        self.minifb_window.update_with_buffer(&texture.to_u32_buffer(), texture.get_width(), texture.get_height())
            .expect("Pixl: failed to draw texture to window");
    }
}

