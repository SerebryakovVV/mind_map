
// use rusqlite as rsql;

// mod expressions;

// #[derive(Debug)]
// struct User<'a> {
//     // name: String
//     name: & 'a str
// }

// pub struct DB {} 


// impl DB {
//     pub fn connect() {}
// }
// const COMMAND_CAPACITY: usize = 500;
// const DB_PATH: &str = "tasks.db3";


// struct Task {
//     id: i32,
//     name: String   
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
