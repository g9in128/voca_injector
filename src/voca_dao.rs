use std::{collections::HashMap, fs};

use rusqlite::{params, Connection};

use crate::voca_injector::{Dict, Voca};

pub struct VocaDao {
    connection: Connection,
    vocas: HashMap<usize, Voca>,
    dicts: HashMap<String, Dict>,
}

impl VocaDao {
    pub fn connect() -> VocaDao {
        let _ = fs::create_dir_all("target/dbs");
        let conn = Connection::open("target/dbs/vocabulary.sqlite").expect("Database Open Fail!");

        let mut dao = VocaDao {
            connection: conn,
            vocas: HashMap::new(),
            dicts: HashMap::new(),
        };

        let is_db_empty = dao.create_table();

        if (!is_db_empty) {
            dao.fetch_data_from_db();
        }

        dao
    }

    fn create_table(&self) -> bool {
        let a = self.connection.execute(
            "CREATE TABLE IF NOT EXISTS vocabularies (
                num INTEGER PRIMARY KEY,
                eng TEXT NOT NULL,
                kor TEXT NOT NULL
        )",
            [],
        );

        let b = self.connection.execute(
            "
			CREATE TABLE IF NOT EXISTS tags (
                num INTEGER,
                name TEXT,
                PRIMARY KEY (num,name),
                FOREIGN KEY (num) REFERENCES vocabularies(num)
				)",
            [],
        );

        !a.is_err() && a.unwrap() > 0 && !b.is_err() && b.unwrap() > 0
    }

    fn fetch_data_from_db(&mut self) {
        self.vocas.clear();
        self.dicts.clear();

        let mut stmt = self
            .connection
            .prepare("SELECT num,eng,kor FROM vocabularies")
            .unwrap();
        let vocas = stmt
            .query_map([], |row| {
                Ok(Voca::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();

        for voca in vocas {
            let voca = voca.unwrap();
            self.vocas.insert(voca.num, voca);
        }

        let mut stmt = self
            .connection
            .prepare("SELECLT num,name FROM tags")
            .unwrap();
        let tags = stmt
            .query_map([], |row| {
                Ok({
                    let tag: (usize, String) = (row.get(0).unwrap(), row.get(1).unwrap());
                    tag
                })
            })
            .unwrap();
        for tag in tags {
            let tag = tag.unwrap();
            let voca = self.vocas.get_mut(&tag.0).unwrap();
            voca.tags.push(tag.1);
        }

        for voca in self.vocas.values() {
            for tag in voca.tags.clone() {
                let dict: &mut Dict;
                if (!self.dicts.contains_key(&tag)) {
                    let dict2 = Dict::new(&tag);
                    self.dicts.insert(tag.clone(), dict2);
                }
                dict = self.dicts.get_mut(&tag).unwrap();
                dict.vocas.push(voca.num);
            }
        }
    }

    pub fn insert_voca(&self, voca: &Voca) {
        let a = self.connection.execute(
            "INSERT INTO vocabularies VALUES (?1,?2,?3)",
            params![&voca.num, &voca.eng, &voca.kor],
        );
        println!("{:?}", a);
    }

    pub fn insert_tag(&self, voca: &Voca) {
        for tag in &voca.tags {
            let _ = self
                .connection
                .execute("INSERT INTO tags VALUES (?1,?2)", params![voca.num, tag]);
        }
    }

    pub fn get_voca_eng(&self, eng: &str) -> Box<[&Voca]> {
        let mut vocas: Vec<&Voca> = vec![];
        for (_i, voca) in &self.vocas {
            if (voca.eng == eng) {
                vocas.push(voca);
            }
        }
        vocas.into_boxed_slice()
    }

    pub fn get_voca_kor(&self, kor: &str) -> Box<[&Voca]> {
        let mut vocas: Vec<&Voca> = vec![];
        for (_i, voca) in &self.vocas {
            if (voca.kor == kor) {
                vocas.push(voca);
            }
        }
        vocas.into_boxed_slice()
    }

    pub fn get_voca_tag(&self, tag: &str) -> Box<[&Voca]> {
        let mut vocas: Vec<&Voca> = vec![];
        let dict = self.dicts.get(tag);
        let tags = match dict {
            Some(tag) => &tag.vocas,
            None => &vec![],
        };
        for tag in tags {
            vocas.push(self.get_voca_num(*tag).expect("Tag MisMatch!"));
        }
        vocas.into_boxed_slice()
    }

    fn get_voca_num(&self, num: usize) -> Option<&Voca> {
        self.vocas.get(&num)
    }
}
