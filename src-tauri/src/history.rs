use std::{sync::{Mutex, Arc}, fs::{OpenOptions, File}, io::{Read, Write}, path::PathBuf};

use lazy_static::lazy_static;

const HISTORY_FILE_NAME: &str = "history.json"; 
use log::*;

lazy_static! {
    static ref HISTORY: Arc<Mutex<ParseRecord>> = {
        Arc::new(
        Mutex::new(
            ParseRecord::from_local()
        )
    )
    };
}

pub fn parse_history() -> Arc<Mutex<ParseRecord>>  {
   HISTORY.to_owned()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParseRecord {
    pub h: Vec<ParseRecordItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ParseRecordItem {
    pub name: String,
    pub command: String,
    pub root: String,
    #[serde(rename = "dbKey")]
    pub db_key: String,
}

impl ParseRecord {
    pub fn from_local() -> Self {

        let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(HISTORY_FILE_NAME)
        .expect("打开个文件都能出错？");

        let mut s = "".to_owned();
        f.read_to_string(&mut s).unwrap();

        if s.trim().is_empty() {
            Self {
                h: vec![],
            }
        } else {
            match serde_json::from_str(&s) {
                Ok(s) =>s,
                Err(e) => {
                    error!("序列化错误: {:?}",e);
                    Self {
                        h: vec![],
                    }
                },
            }
        }
    }

    pub fn add_parse_result(&mut self,t: ParseRecordItem) {
        self.h.push(t);
        self.sync();
    }

    pub fn get(&self, name: &str) -> Option<&ParseRecordItem> {
        let result = self.h.iter().filter(|t| t.name == name).collect::<Vec<&ParseRecordItem>>();

        if result.is_empty() {
            None
        } else {
            Some(result[0])
        }
    }

    pub fn contains_name(&self, name: &str) -> bool {
        let result = self.h.iter().filter(|t| t.name == name).collect::<Vec<&ParseRecordItem>>();
        !result.is_empty()
    }

    pub fn remove_root(&mut self, db_key: &str) {
        for (idx, t) in self.h.iter().enumerate() {
            if t.db_key == db_key {
                self.h.remove(idx);
                break;
            }
        }
        self.sync();
    }

    fn sync(&mut self) {
        info!("History Self: {:?}", self);

        let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(HISTORY_FILE_NAME)
        .expect("打开个文件都能出错？");

        match serde_json::to_string(&self) {
            Ok(s) => {
                info!("serde self: {}", s);
                use std::io::Seek;
                let _ = f.rewind();
                if let Err(e) = f.write_all(s.as_bytes()) {
                    error!("写入错误: {:?}",e);
                }
            },
            Err(e) => {  error!("序列化错误: {:?}",e); }
        }
    }
}

impl ParseRecordItem {
    pub fn new(name: &str, command: &str, root: &str ,db_key: &str) -> Self {
        let name = name.to_owned();
        let command = command.to_owned();
        let root = root.to_owned();
        let db_key = db_key.to_owned();

        Self { name, command, root, db_key }
    }
}
