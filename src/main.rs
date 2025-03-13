mod node;
mod db;
use node::*;
use raylib::{prelude::*};
use rusqlite as rsql;


const DB_PATH: &str = "tasks.db3";

fn main() {

    let test_node: Node = Node::new(
        25, 
        60, 
        300, 
        200, 
        String::from("hello world this is a very long line and it will be displayed without the spaces, very long string indeed, look how long it is, still going wow")
    );

    
    let db_inst = db::DB::new(DB_PATH);

    let (mut rlh, rlt) = raylib::init().size(800, 600)
                                       .resizable()
                                       .title("Mind Map")
                                       .build();

    let font = rlh.load_font_ex(&rlt, "OpenSans-Regular.ttf", 28, None).expect("font not loaded");

    rlh.set_target_fps(1);

    while !rlh.window_should_close() {

        let mut rlDrawH = rlh.begin_drawing(&rlt);
        
        test_node.draw_body(&mut rlDrawH);
        test_node.draw_text(&mut rlDrawH, &font);
        
        rlDrawH.clear_background(Color::WHITE);


    }
    
}









// fn draw_nodes(handle: &mut RaylibDrawHandle, nodes: &mut Vec<Node>, font: &Font) {
//     for n in nodes {
//         handle.draw_rectangle(
//             n.x, 
//             n.y, 
//             // n.width, 
//             // n.height, 
//             // n.color
//         );

        // add clipping
        // handle.draw_text(n.text, n.x, n.y, 15, Color::RED); 
        
    
        // handle.draw_text_ex(font, "text", Vector2 {x:800.0, y:800.0}, 12.0, 1.0,  Color::RED); 
        
        

        // the initial idea is to get the string, check the width word by word, on >node.width start from new line 
       
    //    let mut line: str = ;
    //    for word in n.text.split_whitespace() {

    //    }


        // let size = font.measure_text("text text thseiltsjh;lfha;ldfks ", 12.0, 1.0); // returns width and height
        // print!("{:#?}", size);
    
    
//     }
// }

// fn panning_handler(delta: Vector2, nodes: &mut Vec<Node>) {
//     for n in nodes {
//         n.x += delta.x as i32;
//         n.y += delta.y as i32;
//     }
// }

// fn move_node(node: &mut Node, delta: Vector2) {
//     node.x += delta.x as i32;
//     node.y += delta.y as i32;
// }



// fn mouse_target_check(delta: Vector2, position: Vector2, nodes: &mut Vec<Node>) {
//     // later can add flags, checking of borders, corners
//     let mut found_in_nodes = false;
//     for n in &mut *nodes {
//         if position.x > n.x as f32 
//         && position.x < (n.x + n.width) as f32 
//         && position.y > n.y as f32 
//         && position.y < (n.y + n.height) as f32 {
//             move_node(n, delta);
//             found_in_nodes = true;
//             break;
//         } 
//     }
//     if !found_in_nodes {
//         panning_handler(delta, nodes);
//     }
// }



// fn resize_node(delta: Vector2, position: Vector2, nodes: &mut Vec<Node>) {
//     for n in &mut *nodes {
//         if position.x > (n.x - 25) as f32
//         && position.x < (n.x + 25) as f32
//         && position.y > (n.y - 25) as f32
//         && position.y < (n.y + 25) as f32 {
//             n.x += delta.x as i32;                // maybe instead equalize the x and y to the x and y of the mouse
//             n.y += delta.y as i32;
//             n.width -= delta.x as i32;
//             n.height -= delta.y as i32;
            
//         }
//     }
// }



// https://www.raylib.com/examples/core/loader.html?name=core_2d_camera_mouse_zoom
// https://www.raylib.com/examples/core/loader.html?name=core_custom_frame_control
// https://www.raylib.com/examples/core/loader.html?name=core_scissor_test

// when it gets too small it cant grow back
// i am changing the position but i need to keep the inital x and y and then apply the zoom factor
// fn zoom_handler(delta: f32, nodes: &mut Vec<Node>, zoom: &mut f32, mouse_position: Vector2) {
//     println!("{}", *zoom);
    
//     if *zoom > 0.3 {
//         if delta > 0.0 {
//             *zoom += 0.05;
//         } else {
//             *zoom -= 0.05;
//         }
//         println!("{}", *zoom);
//         for n in nodes {
//             // move part
//             n.x -= mouse_position.x as i32;
//             n.y -= mouse_position.y as i32;  
//             // zoom part
//             n.x = (n.x as f32 * (*zoom)) as i32;  // mental illness
//             n.y = (n.y as f32 * (*zoom)) as i32;  
//             n.width = (n.width as f32 * (*zoom)) as i32;  
//             n.height = (n.height as f32 * (*zoom)) as i32; 
//             // return part
//             n.x += mouse_position.x as i32;
//             n.y += mouse_position.y as i32;
//         }
//     } else {
//         *zoom = 0.3;
//     }
// }





 
    


        

        // if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        //     let mouse_position_delta = d.get_mouse_delta();
        //     let mouse_position = d.get_mouse_position();              // i can add a variable that checks if one of the nodes is being dragged
        //     match mouse_position_delta {                              // 
        //         Vector2 {x:0.0, y:0.0} => (),
        //         // _ => panning_handler(mouse_position_delta, &mut nodes)
        //         // _ => mouse_target_check(mouse_position_delta, mouse_position, &mut nodes)
        //         // _ => resize_node(mouse_position_delta, mouse_position, &mut nodes)
        //     };
        // }
        
        // let mouse_wheel_delta = d.get_mouse_wheel_move();
        // if mouse_wheel_delta != 0.0 {
        //     zoom_handler(mouse_wheel_delta, &mut nodes, &mut zoom_factor, d.get_mouse_position());
        // }
       








































// JUST EXAMPLES











// use std::fs;


// fn open_and_read(path: &str) -> Result<String, std::io::Error> {
//     let content = fs::read_to_string(path)?;
//     Ok(content)
// }




// initial capacity collections










  // d.draw_text_ex(&font, "text", Vector2 {x:800.0, y:800.0}, 12.0, 1.0,  Color::RED); 
       
        // let size = font.measure_text("text text thseiltsjh;lfha;ldfks ", 12.0, 1.0); // returns width and height
        // print!("{:#?}", size);







