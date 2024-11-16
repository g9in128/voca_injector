use std::{collections::HashMap, vec};

use crate::voca_dao::VocaDao;

pub struct VocaInjector {
    vocas: HashMap<usize, Voca>,
    dicts: HashMap<String, Dict>,
    dao: VocaDao,
}

impl VocaInjector {
    pub fn new() -> VocaInjector {
        VocaInjector {
            vocas: HashMap::new(),
            dicts: HashMap::new(),
            dao: VocaDao::connect(),
        }
    }

    pub fn insert(&mut self, eng: &str, kor: &str) {
        let voca = Voca::new(self.vocas.len(), eng.to_string(), kor.to_string());
        self.dao.insert_voca(&voca);
        self.vocas.insert(voca.num, voca);
    }

    pub fn insert_tag(&mut self, eng: &str, kor: &str, tag: &str) {
        let mut voca = Voca::new(self.vocas.len(), eng.to_string(), kor.to_string());
        self.dao.insert_voca(&voca);
        self.tag_voca(&mut voca, tag);
        self.vocas.insert(voca.num, voca);
    }

    fn tag_voca(&mut self, voca: &mut Voca, tag: &str) {
        if (!self.dicts.contains_key(tag)) {
            let dict = Dict::new(tag);
            self.dicts.insert(tag.to_string(), dict);
        }
        let dict = self.dicts.get_mut(tag).expect("This will never happen");
        voca.tags.push(tag.to_string());
        dict.vocas.push(voca.num);
        self.dao.insert_tag(&voca);
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

#[derive(Debug)]
pub struct Dict {
    pub name: String,
    pub vocas: Vec<usize>,
}

impl Dict {
    pub fn new(name: &str) -> Dict {
        Dict {
            name: name.to_string(),
            vocas: vec![],
        }
    }
}
#[derive(Debug)]
pub struct Voca {
    pub num: usize,
    pub eng: String,
    pub kor: String,
    pub tags: Vec<String>, //나중에 &str로 바꾸기
}

impl Voca {
    pub fn new(num: usize, eng: String, kor: String) -> Voca {
        Voca {
            num,
            eng,
            kor,
            tags: vec![],
        }
    }
}
