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
        let mut line = String::new();
        let mut y_offset = 5;

        let words = self.text.split_whitespace();
        for w in words {
 
            println!("{w}");
            match 
                (font.measure_text(&line, 28.0, 1.0).x 
                +font.measure_text(w, 28.0, 1.0).x) > self.w as f32 - 15.0 {  
                    true => {
                        rldh.draw_text_ex(
                            font,
                            &line,
                            Vector2 {
                                x: (self.x + 5) as f32,
                                y: (self.y + y_offset) as f32
                            },
                            28.0,
                            1.0,
                            Color::RED
                        );
                        y_offset += 20;
                        line.clear();
                        line.push_str(w);
                        line.push_str(" ");
                    },
                    false => {
                        line.push_str(w);
                        line.push_str(" ");
                    }
                };
        }
        if line.len() != 0 {
            rldh.draw_text_ex(
                font,
                &line,
                Vector2 {
                    x: (self.x + 5) as f32,
                    y: (self.y + y_offset) as f32
                },
                28.0,
                1.0,
                Color::RED
            );
        }
    }
}