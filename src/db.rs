use rusqlite::{self as rsql, ffi::Error};


pub struct DB {
    connection: rsql::Connection
}

impl DB {

    pub fn new(path: & str) -> Self {
        let connection = rsql::Connection::open(path).unwrap_or_else(|e| {
            println!("DB connection error: {}", e); 
            panic!();
        });
        Self {
            connection
        }
    }

    pub fn get_nodes(&self) {
        let query_result = match self.connection.execute("select * from nodes", ()) {
            Ok(_) => {},
            Err(_) => {}
        };
    }

}








// pub fn get_nodes(&self) {
//     let mut statement = self.connection.prepare("select * from nodes")?;
//     let query_result = statement.query_map([], |el| {
//         Ok(User{name:el.get(0)?})
//     })?;
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

   

  

    // for u in query_result {
    //     println!("{:?}", u);
        
    // }
