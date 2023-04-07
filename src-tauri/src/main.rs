// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Read};

use dir::IDir;
use kv::{create_force_file_db, file_db, FileListDb};
use kv_parse::DirSKVParser;
use mem_parse::DirSMemParser;

mod dir;
mod file;
mod i18n;
mod kv;
mod kv_parse;
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
fn result_package<T>(f: impl FnOnce() -> anyhow::Result<T>) -> BackendResponse<T> {
    BackendResponse::result(f())
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
fn mem_parse(path: String) -> BackendResponse<Option<IDir>> {
    result_package::<Option<IDir>>(|| {
        let f = OpenOptions::new().read(true).open(path)?;
        let parser = DirSMemParser::new();
        Ok(parser.parse(f)?)
    })
}

#[tauri::command]
fn kv_parse(path: String) -> BackendResponse<String> {
    result_package::<String>(|| {
        let f = OpenOptions::new().read(true).open(&path)?;
        let db = create_force_file_db(&path)?;
        let parser = DirSKVParser::new(db);
        let root = parser.parse(f)?;
        Ok(root)
    })
}

#[tauri::command]
fn db_select(root: String, path: String) -> BackendResponse<Option<IDir>> {
    result_package::<Option<IDir>>(|| {
        let db = file_db(&root)?;
        let file_list_db = FileListDb::new(db);
        Ok(Some(file_list_db.select_dir(&path)?))
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, mem_parse, kv_parse, db_select
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
