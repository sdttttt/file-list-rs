use std::sync::Arc;
use crate::i18n::KeywordLibray;



// 有效的解析结构：
//.:
//总用量 24K
//drwxrwxr-x  4 sdtttttt sdtttttt 4.0K  4月 19 15:56 .
//drwxrwxr-x 10 sdtttttt sdtttttt 4.0K  4月 19 15:55 ..
//drwxrwxr-x  2 sdtttttt sdtttttt 4.0K 12月 20 15:11 assets
//-rw-rw-r--  1 sdtttttt sdtttttt  825 12月 20 15:11 favicon.svg
//-rw-rw-r--  1 sdtttttt sdtttttt  969 12月 20 15:11 index.html
//-rw-rw-r--  1 sdtttttt sdtttttt    0  4月 19 15:56 list.txt
//drwxrwxr-x  2 sdtttttt sdtttttt 4.0K 12月 20 15:11 loading


#[derive(Debug, PartialEq)]
enum LsAlhrParseMode {
    Empty,
    MatchDir,
}

pub struct LsAlhrParser {
    keywords: Option<Box<dyn KeywordLibray>>,
    mode: LsAlhrParseMode,
    root_path: Option<String>,
    current_path: Option<String>,
    db: Arc<sled::Db>,
}

impl LsAlhrParser {
    pub const COMMAND: &str = "ls -alhR";

    pub fn new(db: Arc<sled::Db>) -> Self {
        Self {
            root_path: None,
            current_path: None,
            keywords: None,
            mode: LsAlhrParseMode::Empty,
            db,
        }
    }
}

