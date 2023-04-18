use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{dir::IDir, file::IFile, utils};
use anyhow::bail;
use lazy_static::lazy_static;
use log::*;
use regex::Regex;

lazy_static! {
// sled 存放解析结果
static ref FILE_DB_MAP: Mutex<HashMap<String, Arc<sled::Db>>> = Mutex::new(HashMap::new());
}

pub struct FileListDb {
    db: Arc<sled::Db>,
}

impl FileListDb {
    pub fn new(db: Arc<sled::Db>) -> FileListDb {
        Self { db }
    }

    #[allow(unused)]
    pub fn inner(self) -> Arc<sled::Db> {
        self.db.to_owned()
    }

    pub fn dir_info(&self, path: &str) -> anyhow::Result<IDir> {
        let mut root: Option<IDir> = None;
        let mut dirs: Vec<IDir> = vec![];
        let mut match_dir_count = 0_usize;
        for kv_result in self.db.scan_prefix(path) {
            match_dir_count += 1;
            if let Ok((ref k, ref v)) = kv_result {
                let ks = utils::ivec_to_str(k);
                let vs = utils::ivec_to_str(v);
                // 相等说明是本次查询的根路径
                if ks == path {
                    root = Some(serde_json::from_str(vs)?);
                    continue;
                }

                // 去掉本次路径的头, 必须是文件分隔符开头, 不然会出现以下情况：
                // A:\abc\git # 匹配git下的子目录
                // A:\abc\github # 该目录不是子目录，但是却命中了
                // A:\abc\git\hub # 这个才是子目录
                // 但是也有例外: A:\ 盘符，盘符后的路径可以不跟随
                if !ks[path.len()..].starts_with("\\") && !path.ends_with(":\\") {
                    continue;
                }

                // 去掉本次路径的头, 然后通过文件系统的分隔符, 通过剩下的路径段数查看是否是本次查询目录的子目录
                let is_sub_dir = ks[path.len()..]
                    .split("\\")
                    .filter(|t| !t.trim().is_empty())
                    .count()
                    == 1;

                if is_sub_dir {
                    dirs.push(IDir::new(ks));
                }
            }
        }

        info!("match path count: {}", match_dir_count);
        if root.is_none() {
            bail!("找不到该路径")
        }
        root.as_mut().unwrap().dirs = dirs;
        Ok(root.unwrap())
    }

    // 查找文件夹关键词
    pub fn find_dir(&self, reg_exp: &str) -> anyhow::Result<Vec<IDir>> {
        let reg = Regex::new(reg_exp)?;

        let mut result_dirs = vec![];
        for kv_result in self.db.iter() {
            if let Ok((ref k, _)) = kv_result {
                let ks = utils::ivec_to_str(k);

                let mut path_seq = ks.split("\\").collect::<Vec<&str>>();
                path_seq.reverse();

                if reg.is_match(path_seq[0]) {
                    result_dirs.push(IDir::new(ks));
                }
            }
        }

        Ok(result_dirs)
    }

    pub fn find_file(&self, reg_exp: &str) -> anyhow::Result<Vec<String>> {
        let reg = Regex::new(reg_exp)?;

        let mut result_file_path = vec![];
        for kv_result in self.db.iter() {
            if let Ok((ref k, ref v)) = kv_result {
                let ks = utils::ivec_to_str(k);
                let vs = utils::ivec_to_str(v);
                if reg.is_match(vs) {
                    let dir = serde_json::from_str::<IDir>(vs)?;
                    let match_files = dir
                        .files
                        .iter()
                        // 过滤出文件名包含关键词的文件信息
                        .filter(|t| reg.is_match(&t.name))
                        .collect::<Vec<&IFile>>();
                    for file in match_files {
                        result_file_path.push(format!("{}\\{}", ks, file.name))
                    }
                }
            }
        }
        Ok(result_file_path)
    }
}

// 创建sled，如果有旧的会直接删掉
pub fn create_file_db(file_path: &str) -> Result<(String, Arc<sled::Db>), anyhow::Error> {
    let db_key = path_to_db_key(file_path);
    let mut file_db = FILE_DB_MAP.lock().unwrap();

    let db_path_str = db_path_prefix(&*db_key);
    let db_path = PathBuf::from(db_path_str);
    if db_path.exists() {
        fs::remove_dir_all(&db_path)?;
    }

    // 这里参数可以调整，可能和解析速度有关系
    let db = Arc::new(
        sled::Config::default()
            .path(db_path)
            .mode(sled::Mode::HighThroughput) // 高新能模式，会占用更多的磁盘空间
            .flush_every_ms(Some(1000)) // 多少时间和硬盘同步一次
            .open()?,
    );

    info!("create sled: {}", db_key);
    file_db.insert(db_key.to_owned(), db.to_owned());
    Ok((db_key, db))
}

pub fn file_db(db_key: &str) -> Result<Arc<sled::Db>, anyhow::Error> {
    let mut file_db = FILE_DB_MAP.lock().unwrap();

    if !file_db.contains_key(db_key) {
        file_db.insert(
            db_key.to_owned(),
            Arc::new(sled::open(db_path_prefix(&*db_key))?),
        );
        info!("open sled: {}", db_key);
    };

    info!("find sled: {}", db_key);
    Ok(file_db.get(db_key).map(|t| t.to_owned()).unwrap())
}

#[inline]
pub fn db_path_prefix(path: &str) -> String {
    format!("{}{}", "db/", path)
}

#[inline]
pub fn path_to_db_key(path: &str) -> String {
    utils::hash(path)
}
