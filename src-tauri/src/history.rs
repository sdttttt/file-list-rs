use std::{sync::{Mutex, Arc}, fs::{OpenOptions, File}, io::{Read, Write}, path::PathBuf};

use anyhow::{bail};
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

fn history_store() -> Arc<Mutex<ParseRecord>>  {
   HISTORY.to_owned()
}

pub fn all_history_records() -> Vec<ParseRecordItem> {
    let his = history_store();
    let his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
    let result = his_lock.h.to_owned();
    result
}

pub fn get_history_record(name: &str) -> anyhow::Result<Option<ParseRecordItem>> {
    let store = history_store();
        let parse_record = {
            let his_lock = store.lock().unwrap();
            match  his_lock.get(&name) {
                Some(t) => {
                    Some(t.to_owned())
                },
                None => None,
            }
        };
    Ok(parse_record)
}

pub fn contains_history_record(name: &str) -> anyhow::Result<bool> {
    let store = history_store();

            // 解析记录, 这里划出一个生命周期是因为解析一般要很长时间，不能一直拿着锁
            let his_lock = store.lock().expect("获取解析历史锁出错，这也行啊");
            // 有记录说明已经解析完成了
             Ok(his_lock.contains_name(&name)) 
}

pub fn remove_history_record(name: &str) -> anyhow::Result<()> {
    let his = history_store();
        let mut his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
        his_lock.remove_root(&name);
        Ok(())
}

pub fn add_history_record(item: ParseRecordItem) -> anyhow::Result<()> {
    let his = history_store();
    
    // 将本次解析保存到记录
    let mut his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
    his_lock.add_parse_result(item);
    Ok(())
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

    pub fn remove_root(&mut self, name: &str) {
        for (idx, t) in self.h.iter().enumerate() {
            if t.name == name {
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
