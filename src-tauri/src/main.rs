// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::future::Future;
use std::io::{BufReader, BufRead};
use std::{fs::OpenOptions, io::Read};

use anyhow::bail;
use dir::IDir;
use dir_s_parse::DirSParser;
use history::{all_history_records, remove_history_record, ParseRecordItem};
use kv::{create_file_db, file_db, FileListDb};
use log::*;
use simplelog::Config as LogConfig;
use simplelog::*;

use crate::command::ParseCommand;
use crate::history::{add_history_record, contains_history_record, get_history_record};
use crate::ls_alhr_parse::LsAlhrParser;

mod dir;
mod dir_s_parse;
mod file;
mod history;
mod i18n;
mod kv;
mod ls_alhr_parse;
mod os;
mod utils;
mod command;

pub trait Parser {

    fn parse(&mut self, f: File) -> anyhow::Result<()> {
        let buf_lines = BufReader::new(f).lines();
        // 行数计数器
        let mut line_number = 0_usize;
        for line_result in buf_lines {
                let line = &line_result?;
                line_number += 1;
                self.parse_line(line.trim(), &line_number)?;
        };
        Ok(())
    }
    
    fn parse_line(&mut self, line: &str, line_number: &usize) -> anyhow::Result<()>;
    fn root_path(&mut self) -> anyhow::Result<String>;
}

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
async fn result_package<T>(f: impl Future<Output = anyhow::Result<T>>) -> BackendResponse<T> {
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
async fn kv_parse(name: String, command: String, path: String) -> BackendResponse<ParseRecordItem> {
    result_package::<ParseRecordItem>(async {
        info!("kv_parse: path = {}", path);

        if contains_history_record(&name)? {
            info!("Duplicate name: {}", name);
            bail!("已经存在该名的解析结果.")
        }

        let f = OpenOptions::new().read(true).open(&path)?;
        let (db_key, db) = create_file_db(&path)?;

        let parse_command = ParseCommand::try_from(command)?;
        let mut parser: Box<dyn Parser> = match parse_command {
            ParseCommand::DirS => Box::new(DirSParser::new(db)),
            ParseCommand::LsAlhr => Box::new(LsAlhrParser::new(db)),
        };

        parser.parse(f)?;
        let root_path = parser.root_path()?;

        let new_parse_record = ParseRecordItem::new(
            &name, 
            &String::from(parse_command), 
            &root_path,
             &db_key
            );
        // 将本次解析保存到记录
        add_history_record(new_parse_record.to_owned())?;
        Ok(new_parse_record)
    })
    .await
}

#[tauri::command]
async fn db_select(name: String, path: String) -> BackendResponse<Option<IDir>> {
    result_package::<Option<IDir>>(async {
        let parse_record = match get_history_record(&name)? {
            Some(t) => t,
            None => bail!("该解析记录已经被删除或者丢失.."),
        };
        info!("db_key: {}, path: {}", &parse_record.db_key, path);
        let db = file_db(&parse_record.db_key)?;
        let file_list_db = FileListDb::new(db, &parse_record.command)?;
        Ok(Some(file_list_db.dir_info(&path)?))
    })
    .await
}

#[tauri::command]
async fn db_find_dir(name: String, reg_exp: String) -> BackendResponse<Vec<IDir>> {
    result_package::<Vec<IDir>>(async {
        let parse_record = match get_history_record(&name)? {
            Some(t) => t,
            None => bail!("该解析记录已经被删除或者丢失.."),
        };

        info!("db_key: {}, dir keyword: {}", &parse_record.db_key, reg_exp);
        let db = file_db(&parse_record.db_key)?;
        let file_list_db = FileListDb::new(db, &parse_record.command)?;
        Ok(file_list_db.find_dir(&reg_exp)?)
    })
    .await
}

#[tauri::command]
async fn db_find_file(name: String, reg_exp: String) -> BackendResponse<Vec<String>> {
    result_package::<Vec<String>>(async {
        let parse_record = match get_history_record(&name)? {
            Some(t) => t,
            None => bail!("该解析记录已经被删除或者丢失.."),
        };

        info!(
            "db_key: {}, file keyword: {}",
            &parse_record.db_key, reg_exp
        );
        let db = file_db(&parse_record.db_key)?;
        let file_list_db = FileListDb::new(db, &parse_record.command)?;
        Ok(file_list_db.find_file(&reg_exp)?)
    })
    .await
}

#[tauri::command]
async fn parse_records() -> BackendResponse<Vec<ParseRecordItem>> {
    result_package::<Vec<ParseRecordItem>>(async { Ok(all_history_records()) }).await
}

#[tauri::command]
async fn remove_record(name: String) -> BackendResponse<bool> {
    result_package::<bool>(async {
        remove_history_record(&name)?;
        Ok(true)
    })
    .await
}

fn main() {
    init_log();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
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
