use raylib::prelude::*;

pub struct Framebuffer {
    width: u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.background_color);
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
    Image::export_image(&self.color_buffer, file_path);
    }

}
