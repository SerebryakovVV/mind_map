use raylib::{prelude::*};

pub struct Node {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub text: String
}

impl Node {
    pub fn new(x:i32, y:i32, w:i32, h:i32, text:String) -> Self {
        Self {x, y, w, h, text}
    }

    pub fn draw_body(&self, rldh: &mut RaylibDrawHandle) {
        rldh.draw_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            Color::BLACK
        );
    }

    pub fn draw_text(&self, rldh: &mut RaylibDrawHandle, font: &Font) {
        rldh.draw_text_ex(
            font, 
            &self.text, 
            Vector2 {
                x: (self.x+10) as f32, 
                y: (self.y+5) as f32
            }, 
            28.0, 
            1.0, 
            Color::WHITE
        );
    }
}