// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Read};

use dir::IDir;
use parse::DirSDParser;

mod dir;
mod file;
mod i18n;
mod parse;
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
fn parse_file_list_by_path(path: String) -> BackendResponse<Option<IDir>> {
    let r = {
        let f = OpenOptions::new().read(true).open(path).unwrap();
        let mut parser = DirSDParser::new();
        parser.parse(f)
    };

    BackendResponse::result(r)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, parse_file_list_by_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
