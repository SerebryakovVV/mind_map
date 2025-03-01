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
    }
}

fn main() {
   
    let (mut rl, thread) = raylib::init()
        .size(900, 900)
        .title("Mind Map")
        .build();

    
    rl.set_target_fps(60);

    let first_node = Node {
        x: 100,
        y: 200,
        color: Color::BLUE,
        width: 50,
        height: 300,
        header: "one",
        text: "two"
    };
    


    while !rl.window_should_close() {


        let mut d = rl.begin_drawing(&thread);
       
        d.clear_background(Color::WHITE);
       

        d.draw_text_codepoints(d.get_font_default(), "hellow", Vector2 {x:10.0,y:20.0}, 25.0, 1.0, Color::BLACK);

        first_node.draw(&mut d);

        


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
