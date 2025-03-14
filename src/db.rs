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


// db.connection.execute



