use std::fs;

use rusqlite::{params, Connection};

use crate::voca_injector::Voca;

pub struct VocaDao {
    connection:Connection
}

impl VocaDao {
    pub fn connect() -> VocaDao {
        let _ = fs::create_dir_all("target/dbs");
        let conn = Connection::open("target/dbs/vocabulary.sqlite").expect("Database Open Fail!");

        let a = conn.execute(r#"

        CREATE TABLE IF NOT EXISTS vocabularies (
                num INTEGER PRIMARY KEY,
                eng TEXT NOT NULL,
                kor TEXT NOT NULL
        )

        "#, []);
        println!("{:?}",a);

        let b = conn.execute("
        CREATE TABLE IF NOT EXISTS tags (
                num INTEGER,
                name TEXT,
                PRIMARY KEY (num,name),
                FOREIGN KEY (num) REFERENCES vocabularies(num)
        )", []);
        println!("{:?}",b);
        VocaDao {connection:conn}
    }

    pub fn insert_voca(&self,voca:&Voca) {
        let a = self.connection.execute("INSERT INTO vocabularies VALUES (?1,?2,?3)", params![&voca.num,&voca.eng,&voca.kor]);
        println!("{:?}",a);
    }

    pub fn insert_tag(&self,voca:&Voca){
        for tag in &voca.tags {
            // let mut stmt = self.connection.prepare("SELECT COUNT(*) FROM tags WHERE num = ?1 AND name = ?2").unwrap();
            // let cnt:isize = stmt.query_row(params![voca.num,tag],|row| row.get(0)).unwrap();
            // if (cnt == 0) {
                let _ = self.connection.execute("INSERT INTO tags VALUES (?1,?2)", params![voca.num,tag]);
            // }
        }
    }
}