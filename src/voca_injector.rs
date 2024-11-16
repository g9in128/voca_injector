use std::{collections::HashMap, vec};

use crate::voca_dao::VocaDao;

pub struct VocaInjector {
    dao: VocaDao,
}

impl VocaInjector {
    pub fn new() -> VocaInjector {
        VocaInjector {
            dao: VocaDao::connect(),
        }
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
