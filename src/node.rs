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

        // we take the string of the node
        // we divide it into words
        // we make lines
        // we add word by word until line is wider than the node
        // we get to the next line

        let mut line = String::new();
        let mut y_offset = 5;

        let words = self.text.split_whitespace();
        for w in words {
            line.push_str(w);
            line.push_str(" ");
            // one short line wouldnt be rendered
            // if the current is too long, obviously i should render the previous
            match font.measure_text(&line, 28.0, 1.0).x > self.w as f32 {  
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
                        Color::WHITE
                    );
                    y_offset += 15;
                    line.clear();
                },
                false => {}
            };
        }



        // rldh.draw_text_ex(
        //     font, 
        //     &self.text, 
        //     Vector2 {
        //         x: (self.x+10) as f32, 
        //         y: (self.y+5) as f32
        //     }, 
        //     28.0, 
        //     1.0, 
        //     Color::WHITE
        // );
    }
}