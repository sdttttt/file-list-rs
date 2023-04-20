// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::future::Future;
use std::{fs::OpenOptions, io::Read};

use anyhow::bail;
use dir::IDir;
use history::{parse_history, ParseRecordItem};
use kv::{create_file_db, file_db, path_to_db_key, FileListDb};
use dir_s_parse::DirSKVParser;
use log::*;
use mem_parse::DirSMemParser;
use simplelog::Config as LogConfig;
use simplelog::*;

mod dir;
mod file;
mod history;
mod i18n;
mod kv;
mod dir_s_parse;
mod mem_parse;
mod utils;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BackendResponse<T> {
    raw: Option<T>,
    succ: bool,
    msg: String,
}

impl<T> BackendResponse<T> {
    pub fn succ(raw: T) -> Self {
        Self {
            raw: Some(raw),
            succ: true,
            msg: "".to_owned(),
        }
    }

    pub fn result(r: Result<T, anyhow::Error>) -> Self {
        match r {
            Ok(d) => Self::succ(d),
            Err(e) => Self::fail(e),
        }
    }

    pub fn fail(e: anyhow::Error) -> Self {
        Self {
            raw: None,
            succ: false,
            msg: e.to_string(),
        }
    }
}

// 包装
async fn result_package<T>(f:  impl Future<Output = anyhow::Result<T>>) -> BackendResponse<T> {
    BackendResponse::result(f.await)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> BackendResponse<String> {
    let mut f = OpenOptions::new()
        .read(true)
        .open("C:\\Users\\HP\\Downloads\\文件游览器需求单.md")
        .unwrap();
    let mut raw = String::new();
    f.read_to_string(&mut raw).unwrap();

    BackendResponse::succ(raw)
}

#[tauri::command]
async fn mem_parse(path: String) -> BackendResponse<Option<IDir>> {
    result_package::<Option<IDir>>(async {
        let f = OpenOptions::new().read(true).open(path)?;
        let parser = DirSMemParser::new();
        Ok(parser.parse(f)?)
    }).await
}


#[tauri::command]
async fn kv_parse(name: String, command: String, path: String) -> BackendResponse<ParseRecordItem> {
    result_package::<ParseRecordItem>(async {
        info!("kv_parse: path = {}", path);
        let his = parse_history();
        {
            // 解析记录, 这里划出一个生命周期是因为解析一般要很长时间，不能一直拿着锁
            let his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
            // 有记录说明已经解析完成了
            if  his_lock.contains_name(&name) {
                info!("Duplicate name: {}", name);
                bail!("已经存在该名的解析结果.")
            }
        }

        let f = OpenOptions::new().read(true).open(&path)?;
        let (db_key, db) = create_file_db(&path)?;
        let parser = DirSKVParser::new(db);
        let root_path = parser.parse(f)?;
        // 将本次解析保存到记录
        let mut his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
        let new_parse_record = ParseRecordItem::new(&name, &command, &root_path, &db_key);
        his_lock.add_parse_result(new_parse_record.to_owned());
        Ok(new_parse_record)
    }).await
}

#[tauri::command]
async fn db_select(db_key: String, path: String) -> BackendResponse<Option<IDir>> {
    result_package::<Option<IDir>>(async {
        info!("db_key: {}, path: {}", db_key, path);
        let db = file_db(&db_key)?;
        let file_list_db = FileListDb::new(db);
        Ok(Some(file_list_db.dir_info(&path)?))
    }).await
}

#[tauri::command]
async fn db_find_dir(db_key: String, reg_exp: String) -> BackendResponse<Vec<IDir>> {
    result_package::<Vec<IDir>>(async {
        info!("db_key: {}, dir keyword: {}", db_key, reg_exp);
        let db = file_db(&db_key)?;
        let file_list_db = FileListDb::new(db);
        Ok(file_list_db.find_dir(&reg_exp)?)
    }).await
}

#[tauri::command]
async fn db_find_file(db_key: String, reg_exp: String) -> BackendResponse<Vec<String>> {
    result_package::<Vec<String>>(async {
        info!("db_key: {}, file keyword: {}", db_key, reg_exp);
        let db = file_db(&db_key)?;
        let file_list_db = FileListDb::new(db);
        Ok(file_list_db.find_file(&reg_exp)?)
    }).await
}

#[tauri::command]
async fn parse_records() -> BackendResponse<Vec<ParseRecordItem>> {
    result_package::<Vec<ParseRecordItem>>(async {
        let his = parse_history();
        let his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
        let result =his_lock.h.clone();
        Ok(result)
    }).await
}

#[tauri::command]
async fn remove_record(db_key: String) -> BackendResponse<bool> {
    result_package::<bool>(async {
        let his = parse_history();
        let mut his_lock = his.lock().expect("获取解析历史锁出错，这也行啊");
        his_lock.remove_root(&db_key);
        Ok(true)
    }).await
}

fn main() {
    init_log();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            mem_parse,
            kv_parse,
            db_select,
            db_find_dir,
            db_find_file,
            parse_records,
            remove_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn init_log() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

#[cfg(not(debug_assertions))]
fn init_log() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            LogConfig::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            LogConfig::default(),
            std::fs::File::create("file-list-rs.log").unwrap(),
        ),
    ])
    .unwrap();
}
