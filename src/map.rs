use raylib::{prelude::*};
use crate::node::Node;

pub struct Map {
    pub nodes: Vec<Node>,
    // lines: Vec<Line>
}

impl Map {
    pub fn handle_panning(&mut self, delta: Vector2) {
        for n in self.nodes.iter_mut() {
            n.x += delta.x as i32;
            n.y += delta.y as i32;
            println!("panning");
        }
    }

    pub fn draw_nodes(&self, rldh: &mut RaylibDrawHandle, font: &Font) {
        for n in self.nodes.iter() {
            n.draw_body(rldh);
            n.draw_text(rldh, font);
        }
    }
}