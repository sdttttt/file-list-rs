use std::{fs::File, io::BufRead, io::BufReader, sync::{Arc}};

use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    dir::IDir,
    file::IFile,
    i18n::{KeywordLibray, Zh, match_lang},
    utils::{self},
};
// 有效的解析文本
//C:\Users\HP\.vscode\extensions\ms-python.vscode-pylance-2022.11.20\dist\typeshed-fallback\stdlib\email 的目录

//2022/11/10  10:59    <DIR>          .
//2022/11/10  10:59    <DIR>          ..
//2022/11/10  10:59               400 base64mime.pyi
//2022/11/10  10:59             1,067 charset.pyi
//2022/11/10  10:59               480 contentmanager.pyi
//2022/11/10  10:59               293 encoders.pyi
//2022/11/10  10:59             1,532 errors.pyi
//2022/11/10  10:59               951 feedparser.pyi
//2022/11/10  10:59             1,329 generator.pyi
//2022/11/10  10:59             1,158 header.pyi
//2022/11/10  10:59             6,260 headerregistry.pyi
//2022/11/10  10:59               641 iterators.pyi
//2022/11/10  10:59             5,095 message.pyi
//2022/11/10  10:59    <DIR>          mime
//2022/11/10  10:59               952 parser.pyi
//2022/11/10  10:59             3,055 policy.pyi
//2022/11/10  10:59               735 quoprimime.pyi
//2022/11/10  10:59             2,107 utils.pyi
//2022/11/10  10:59            11,332 _header_value_parser.pyi
//2022/11/10  10:59             1,060 __init__.pyi
//              17 个文件         38,447 字节

lazy_static! {
// 中文匹配
static ref REGEX_ZH: Regex = Regex::new(r"[\u4e00-\u9fa5]+").unwrap();
// 目录匹配
static ref REGEX_DIR_PATH: Regex = Regex::new(r"(\w:\\){1}(\S){0,}").unwrap();
// 匹配底部的数字部分
static ref REGEX_SIZE: Regex = Regex::new(r"[\d,]+").unwrap();

}

#[derive(Debug, PartialEq)]
enum ParseMode {
    Empty,
    MatchDir,
}

// dir /s *.* KVDB解析器
// 别问我两个解析器为什么不抽象，因为抽象太难了，除了核心部分的解析逻辑是相同的，数据保存以及读取的逻辑，以及提供给前端使用的接口完全不一样
//基本可以看作一个独立的实现
pub struct DirSKVParser {
    keywords: Option<Box<dyn KeywordLibray>>,
    mode: ParseMode,
    root_path: Option<String>,
    current_path: Option<String>,
    db: Arc<sled::Db>,
}

impl DirSKVParser {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Self {
            root_path: None,
            current_path: None,
            keywords: None,
            mode: ParseMode::Empty,
            db,
        }
    }

    pub fn parse(mut self, f: File) -> anyhow::Result<String> {
        let buf_lines = BufReader::new(f).lines();
        // 行数计数器
        let mut line_count = 0;
        for line_result in buf_lines {
            match line_result {
                Err(e) => bail!("{}", e),

                Ok(ref line) => {
                    // 跳过空行
                    if line.trim().is_empty() {
                        continue;
                    }

                    // 空行不算一行
                    line_count += 1;
                    // 没有加载语言
                    if self.keywords.is_none() {
                        // 对改行语言匹配，装载对应的关键词库
                        match match_lang(line) {
                            Some(k) => self.keywords = Some(k),
                            None => {
                                if line_count > 3 {
                                    bail!("前三行都没有检测到该输出所使用的语言，退出。")
                                }
                                continue;
                            }
                        }
                    }

                    // 匹配到目录路径了
                    if let Some(mat) = REGEX_DIR_PATH.find(line) {
                        let dir_path = mat.as_str();
                        // 记录当前目录
                        self.current_path = Some(dir_path.to_owned());

                        // 如果根路径为空或者，当前匹配的路径长度比根路径小，就换成该路径
                        if self.root_path.is_none()
                            || self.root_path.as_ref().unwrap().len() > dir_path.len()
                        {
                            self.root_path = Some(dir_path.to_owned());
                        }
                        // 模式改为目录模式
                        self.mode = ParseMode::MatchDir;
                        // 处理该目录
                        self.find_dir()?;
                        // 进入下一个循环
                        continue;
                    }

                    // 匹配到文件夹大小
                    if line.contains(self.keywords.as_ref().unwrap().size())
                        && self.mode == ParseMode::MatchDir
                    {
                        let sizes = REGEX_SIZE
                            .find_iter(line)
                            .map(|t| t.as_str().trim())
                            .collect::<Vec<&str>>();
                        // 必须是2个，一个是文件数量，一个是文件夹
                        debug_assert_eq!(sizes.len(), 2);
                        self.mode = ParseMode::Empty;
                        self.write_dir_size(sizes[1])?;
                        continue;
                    }

                    if self.mode == ParseMode::MatchDir {
                        // 如果上面判断都没中，说明是文件行
                        let files_line_vec = utils::split_file_line(line);

                        // 必须是>=4，不然就是无效的文件行
                        debug_assert!(files_line_vec.len() >= 4);
                        if files_line_vec.contains(&"<DIR>") {
                            // 目录的话上面就记录了, 下一个循环
                            continue;
                        }
                        let file_info = IFile::from_file_line_vec(files_line_vec);
                        self.insert_file(file_info)?;
                    }
                }
            }
        }

        self.db.flush()?;
        Ok(self.root_path.expect("没有根路径？绝对不可能！"))
    }

    fn find_dir(&mut self) -> Result<(), anyhow::Error> {
        let path = self.current_path.as_ref().unwrap();
        if !self.db.contains_key(path)? {
            // 初始化一个新的Dir，序列化插入
            let dir = IDir::new(path);
            self.db.insert(path, serde_json::to_vec(&dir)?)?;
        }
        Ok(())
    }

    fn write_dir_size(&mut self, size: &str) -> Result<(), anyhow::Error> {
        let path = self.current_path.as_ref().unwrap();
        match self.db.get(path)? {
            Some(ref iv) => {
                let s = utils::ivec_to_str(iv);
                let mut dir = serde_json::from_str::<IDir>(s)?;
                dir.size = Some(size.to_owned());
                self.db.insert(path, serde_json::to_vec(&dir)?)?;
            }
            None => {
                bail!("目录键不存在？离谱！{}", path)
            }
        }
        Ok(())
    }

    fn insert_file(&mut self, file: IFile) -> Result<(), anyhow::Error> {
        let path = self.current_path.as_ref().unwrap();
        match self.db.get(path)? {
            Some(ref iv) => {
                let s = utils::ivec_to_str(iv);
                let mut dir = serde_json::from_str::<IDir>(s)?;
                dir.files.push(file);
                self.db.insert(path, serde_json::to_vec(&dir)?)?;
            }
            None => {
                bail!("目录键不存在？离谱！{}", path)
            }
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::kv::{create_file_db, FileListDb};

    use super::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        static ref TEST_DATA_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    }

    #[test]
    fn test_file_list() {
        let mut d = TEST_DATA_PATH.clone();
        d.push("test/list.txt");
        let (_, db) = create_file_db(d.to_str().unwrap()).unwrap();
        let file = File::open(d).unwrap();
        let f = DirSKVParser::new(db.to_owned());
        let root = f.parse(file).unwrap();
        let file_list = FileListDb::new(db);
        // println!("{:#?}", file_list.dir_info(&root).unwrap());
        println!("{:#?}", file_list.find_dir("git"));
        println!("{:#?}", file_list.find_file("nps"));
    }
}
