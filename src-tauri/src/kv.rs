use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::bail;
use lazy_static::lazy_static;

use crate::{dir::IDir, utils};

lazy_static! {
// sled 存放解析结果
static ref FILE_DB: Mutex<HashMap<String, Arc<sled::Db>>> = Mutex::new(HashMap::new());
}

pub struct FileListDb {
    db: Arc<sled::Db>,
}

impl FileListDb {
    pub fn new(db: Arc<sled::Db>) -> FileListDb {
        Self { db }
    }

    pub fn inner(self) -> Arc<sled::Db> {
        self.db.to_owned()
    }

    pub fn select_dir(&self, path: &str) -> anyhow::Result<IDir> {
        let mut root: Option<IDir> = None;
        let mut dirs: Vec<IDir> = vec![];
        for kv_result in self.db.scan_prefix(path) {
            if let Ok((ref k, ref v)) = kv_result {
                let ks = utils::ivec_to_str(k);
                let vs = utils::ivec_to_str(v);
                // 相等说明是本次查询的根路径
                if ks == path {
                    root = Some(serde_json::from_str(vs)?);
                    continue;
                }
                // 去掉本次路径的头, 然后通过文件系统的分隔符, 通过剩下的路径段数查看是否是本次查询目录的子目录
                let is_sub_dir = ks[path.len()..]
                    .split("\\")
                    .filter(|t| !t.trim().is_empty())
                    .count()
                    == 1;
                if is_sub_dir {
                    dirs.push(serde_json::from_str(vs)?);
                }
            }
        }
        root.as_mut().unwrap().dirs = dirs;
        Ok(root.unwrap())
    }
}

pub fn create_force_file_db(file_path: &str) -> Result<Arc<sled::Db>, anyhow::Error> {
    let dbname = &utils::hash(file_path);
    let mut file_db = FILE_DB.lock().unwrap();

    let db_path = PathBuf::from(dbname);
    if db_path.exists() {
        fs::remove_dir_all(&db_path)?;
    }
    let db = Arc::new(sled::open(db_path)?);
    file_db.insert(dbname.to_owned(), db.to_owned());
    Ok(db)
}

pub fn file_db(file_path: &str) -> Result<Arc<sled::Db>, anyhow::Error> {
    let key = &utils::hash(file_path);
    let mut file_db = FILE_DB.lock().unwrap();

    if !file_db.contains_key(key) {
        file_db.insert(key.to_owned(), Arc::new(sled::open(key)?));
    };

    Ok(file_db.get(key).map(|t| t.to_owned()).unwrap())
}
