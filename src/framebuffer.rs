use raylib::prelude::*;
use raylib::texture::Image;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color;
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer
                .draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, filename: &str) {
        if Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("bmp"))
        {
            self.render_to_bmp_file(filename);
        } else {
            self.color_buffer.export_image(filename);
        }
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            let mut d = window.begin_drawing(thread);
            d.clear_background(self.background_color);
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    fn render_to_bmp_file(&self, filename: &str) {
        let width = self.width as usize;
        let height = self.height as usize;
        let row_stride = width * 3;
        let row_padding = (4 - (row_stride % 4)) % 4;
        let image_size = (row_stride + row_padding) * height;
        let file_size = 14 + 40 + image_size;

        let file = File::create(filename).expect("failed to create BMP file");
        let mut writer = BufWriter::new(file);

        // BMP file header.
        writer.write_all(&[0x42, 0x4D]).expect("failed to write BMP signature");
        writer
            .write_all(&(file_size as u32).to_le_bytes())
            .expect("failed to write BMP file size");
        writer
            .write_all(&[0u8; 4])
            .expect("failed to write BMP reserved fields");
        writer
            .write_all(&(54u32).to_le_bytes())
            .expect("failed to write BMP pixel offset");

        // DIB header (BITMAPINFOHEADER).
        writer
            .write_all(&(40u32).to_le_bytes())
            .expect("failed to write BMP DIB header size");
        writer
            .write_all(&(self.width as i32).to_le_bytes())
            .expect("failed to write BMP width");
        writer
            .write_all(&(self.height as i32).to_le_bytes())
            .expect("failed to write BMP height");
        writer
            .write_all(&(1u16).to_le_bytes())
            .expect("failed to write BMP planes");
        writer
            .write_all(&(24u16).to_le_bytes())
            .expect("failed to write BMP bpp");
        writer
            .write_all(&(0u32).to_le_bytes())
            .expect("failed to write BMP compression");
        writer
            .write_all(&(image_size as u32).to_le_bytes())
            .expect("failed to write BMP image size");
        writer
            .write_all(&(2835u32).to_le_bytes())
            .expect("failed to write BMP x ppm");
        writer
            .write_all(&(2835u32).to_le_bytes())
            .expect("failed to write BMP y ppm");
        writer
            .write_all(&(0u32).to_le_bytes())
            .expect("failed to write BMP palette colors");
        writer
            .write_all(&(0u32).to_le_bytes())
            .expect("failed to write BMP important colors");

        let padding = [0u8; 3];
        for y in (0..height).rev() {
            for x in 0..width {
                let color = self.color_buffer.get_color(x as i32, y as i32);
                writer
                    .write_all(&[color.b, color.g, color.r])
                    .expect("failed to write BMP pixel data");
            }
            writer
                .write_all(&padding[..row_padding])
                .expect("failed to write BMP row padding");
        }

        writer.flush().expect("failed to flush BMP file");
    }
}
