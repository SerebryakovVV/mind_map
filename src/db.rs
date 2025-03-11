use rusqlite::{self as rsql, ffi::Error};


pub struct DB<'a> {
    path: &'a str,
    connection: Option<rsql::Connection>
}

impl<'a> DB<'a> {

    pub fn new(path: &'a str) -> Self {
        Self {
            path: path,
            connection: None
        }
    }

    pub fn connect(&mut self) {
        self.connection = Some(rsql::Connection::open(&self.path).unwrap_or_else(|e| {
            println!("DB connection error: {}", e); 
            panic!();
        }));
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
