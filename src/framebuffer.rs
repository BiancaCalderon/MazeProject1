use rusttype::{Font, Scale};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, scale: Scale, color: u32) {
        let font_data = include_bytes!("assets/font.ttf") as &[u8];
        let font = Font::try_from_bytes(font_data).unwrap();

        let v_metrics = font.v_metrics(scale);

        let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);

        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, gv| {
                    if gv > 0.5 {
                        let gx = gx as i32 + bounding_box.min.x;
                        let gy = gy as i32 + bounding_box.min.y;
                
                        if gx >= 0 && gx < self.width as i32 && gy >= 0 && gy < self.height as i32 {
                            let gx = gx as usize;
                            let gy = gy as usize;
                
                            self.buffer[gy * self.width + gx] = color;
                        }
                    }
                });
                
            }
        }
    }
}
