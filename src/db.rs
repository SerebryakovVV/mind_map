use rusqlite::{self as rsql, Result as ResultSql};
use crate::node::Node;

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
        let mut statement = self.connection.prepare("SELECT x, y, w, h FROM nodes").unwrap();
        let nodes_iter = statement.query_map([], |row| {
            Ok(
                Node {
                    x: row.get(0)?,
                    y: row.get(0)?,
                    w: row.get(0)?,
                    h: row.get(0)?,
                    text: String::new()
                }
            )
        }).unwrap();

        println!("something here");

        for n in nodes_iter {
            println!("{:?}", n.unwrap());
        };

        // Ok(())
    }

}





// let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
// let person_iter = stmt.query_map([], |row| {
//     Ok(Person {
//         id: row.get(0)?,
//         name: row.get(1)?,
//         data: row.get(2)?,
//     })
// })?;

// for person in person_iter {
//     println!("Found person {:?}", person.unwrap());
// }


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
