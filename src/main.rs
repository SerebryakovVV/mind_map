use raylib::prelude::*;



struct Node<'a> {
    x: i32,
    y: i32,
    color: Color,
    width: i32,
    height: i32,
    header: &'a str,
    text: &'a str
}


impl Node<'_> {
    fn draw(&self, handle: &mut RaylibDrawHandle) -> () {
        handle.draw_rectangle(
            self.x, 
            self.y, 
            self.width, 
            self.height, 
            self.color
        );

        handle.draw_text(self.header, self.x+5, self.y+5, 15, Color::RED); // add text color field

        handle.draw_text(self.text, self.x+5, self.y+25, 15, Color::RED); // add clipping
    }
}




fn panning_handler(delta: Vector2, node: &mut Node) {
    // one rectangle for now
    node.x += delta.x as i32;
    node.y += delta.y as i32;  
}

// take cursor position and a node
fn zoom_handler(delta: f32, node: &mut Node, zoom: &mut f32, mouse_position: Vector2) {


    // when it gets too small it cant grow back


    if delta > 0.0 {
        *zoom += 0.1;
    } else {
        *zoom -= 0.1;
    }


    // move part
    node.x -= mouse_position.x as i32;
    node.y -= mouse_position.y as i32;  


    // zoom part
    node.x = (node.x as f32 * (*zoom)) as i32;  // mental illness
    node.y = (node.y as f32 * (*zoom)) as i32;  
    node.width = (node.width as f32 * (*zoom)) as i32;  
    node.height = (node.height as f32 * (*zoom)) as i32; 


    // return part
    node.x += mouse_position.x as i32;
    node.y += mouse_position.y as i32; 

    // when it gets too small it cant grow back
}



fn main() {


    let mut nodes: Vec<Node> = vec![]; // maybe create with given capacity

    let mut first_node = Node {
        x: 100,
        y: 200,
        color: Color::BLACK,
        width: 300,
        height: 200,
        header: "title one",
        text: "this is the text, main content of the node one"  // add all the nodes to the array, then run the updates on pan
    }; // they will be anyway, sql

    let mut second_node = Node {
        x: 500,
        y: 500,
        color: Color::BLACK,
        width: 350,
        height: 350,
        header: "title two",
        text: "this is the text, main content of the node one"
    };

    nodes.push(first_node);
    nodes.push(second_node);


   
    let (mut rl, thread) = raylib::init()
        .size(900, 900)
        .title("Mind Map")
        .build();

    
    rl.set_target_fps(60);

    
    


    while !rl.window_should_close() {

        let mut zoom_factor: f32 = 1.0;


        let mut d = rl.begin_drawing(&thread);

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_position_delta = d.get_mouse_delta(); 
            match mouse_position_delta {
                Vector2 {x:0.0, y:0.0} => (),
                _ => panning_handler(mouse_position_delta, &mut first_node)  // check for left click and maybe some key together, or flag
            };
        }
        


        let mouse_wheel_delta = d.get_mouse_wheel_move();
        if mouse_wheel_delta != 0.0 {
            zoom_handler(mouse_wheel_delta, &mut second_node, &mut zoom_factor, d.get_mouse_position());
        }
        println!("{}", mouse_wheel_delta);
       
        d.clear_background(Color::WHITE);
       

        d.draw_text_codepoints(d.get_font_default(), "hellow", Vector2 {x:10.0,y:20.0}, 25.0, 1.0, Color::BLACK);

        first_node.draw(&mut d);
        second_node.draw(&mut d);

        


    }
}















































// JUST EXAMPLES











// use std::fs;


// fn open_and_read(path: &str) -> Result<String, std::io::Error> {
//     let content = fs::read_to_string(path)?;
//     Ok(content)
// }











// use rusqlite as rsql;

// mod expressions;

// #[derive(Debug)]
// struct User<'a> {
//     // name: String
//     name: & 'a str
// }


// const COMMAND_CAPACITY: usize = 500;
// const DB_PATH: &str = "tasks.db3";


// struct Task {
//     id: i32,
//     name: String   // learn lifetimes, get rid of the allocations
// }


// fn add_task(arg: &str, con: &rsql::Connection) {
//     println!("add task {}", arg);

//     if false {
        
//     }

// }


// fn delete_task(arg: &str, con: &rsql::Connection) {
//     println!("delete_task {}", arg);

//     if false {
        
//     }

// }


// fn list_tasks(con: &rsql::Connection) {
//     let mut statement = match con.prepare("SELECT id, name FROM tasks") {
//         Ok(s) => s,
//         Err(e) => {
//             println!("Error preparing ls query! {}", e);
//             return;
//         }
//     };
//     match statement.query_map([], |row| Ok(
//         Task {
//             id: row.get(0)?,
//             name: row.get(1)?
//         }
//     )) {
//         Ok(rows) => {
//             for (index, row) in rows.enumerate() {
//                 match row {
//                     Ok(r) => {
//                         println!("{}) {}", index + 1, r.name);
//                     },
//                     Err(e) => {
//                         println!("Error in rows transfer in ls, {}", e);
//                         return;
//                     }
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Error mapping the rows in ls, {}", e);
//             return;
//         }
//     };
// }


// fn main() -> rsql::Result<()> {
//     let db = rsql::Connection::open(DB_PATH).expect("error connecting to db");
//     let mut command: String = String::with_capacity(COMMAND_CAPACITY);
//     let input = std::io::stdin();
//     loop {
//         if let Err(e) = input.read_line(&mut command) {
//             println!("Error reading the input! {}", e);
//             continue;
//         };
//         let command_parts: Vec<&str> = command
//                                               .trim()
//                                               .split_ascii_whitespace()
//                                               .collect();                    
//         match command_parts.get(0) {
//             Some(&c) => {
//                 match c {
//                     "delete" => {
//                         match command_parts.get(1) {
//                             Some(&a) => delete_task(a, &db),
//                             None => println!("No argument provided!")
//                         }
//                     },
//                     "add" => {
//                         match command_parts.get(1) {
//                             Some(&a) => add_task(a, &db),
//                             None => println!("No argument provided!") // this should actually just call the function and 
//                         }                                             // then read the task inside to that function
//                     },
//                     "ls" => list_tasks(&db),
//                     "q" => {return Ok(());},
//                     _ => println!("Unknown command")
//                 }
//             },
//             None => println!("No command provided!")
//         }
//         command.clear();
//         // print!("> ");
//     }
// }




// db.execute("DROP TABLE users", ())?;

    // let query_result = match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY,name  TEXT NOT NULL)", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let query_result = match db.execute("insert into users (name) values ('valentin')", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let mut statement = db.prepare("select name from users")?;
    // let query_result = statement.query_map([], |el| {
    //     Ok(User{name:el.get(0)?})
    // })?;

  

    // for u in query_result {
    //     println!("{:?}", u);
        
    // }
