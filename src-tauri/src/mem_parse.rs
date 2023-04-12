use std::{fs::File, io::BufRead, io::BufReader};

use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    dir::IDir,
    file::IFile,
    i18n::{KeywordLibray, Zh},
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
// 英文匹配
static ref REGEX_EN: Regex = Regex::new(r"[\u4e00-\u9fa5]+").unwrap();
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

// dir /s *.* 内存解析器
pub struct DirSMemParser {
    keywords: Option<Box<dyn KeywordLibray>>,
    mode: ParseMode,
    current_path: Option<String>,
    root: Option<IDir>,
}

impl DirSMemParser {
    pub fn new() -> Self {
        Self {
            current_path: None,
            keywords: None,
            mode: ParseMode::Empty,
            root: None,
        }
    }

    pub fn parse(mut self, f: File) -> Result<Option<IDir>, anyhow::Error> {
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
                        self.load_language(line);
                        if self.keywords.is_none() {
                            if line_count > 3 {
                                bail!("前三行都没有检测到该输出所使用的语言，退出。")
                            }
                            continue;
                        }
                    }

                    // 匹配到目录路径了
                    if let Some(mat) = REGEX_DIR_PATH.find(line) {
                        let dir_path = mat.as_str();
                        // 记录当前目录
                        self.current_path = Some(dir_path.to_owned());
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

        Ok(self.root)
    }

    fn find_dir(&mut self) -> Result<(), anyhow::Error> {
        match &mut self.root {
            Some(d) => {
                // 如果存在路径了，交给root去解析
                d.find_path(&*self.current_path.to_owned().unwrap())?;
            }
            // 如果是第一次，就将该目录路径作为root
            None => self.root = Some(IDir::new(&*self.current_path.to_owned().unwrap())),
        }
        Ok(())
    }

    fn write_dir_size(&mut self, size: &str) -> Result<(), anyhow::Error> {
        if let Some(root) = &mut self.root {
            root.write_size(&*self.current_path.to_owned().unwrap(), size)?;
        }
        Ok(())
    }

    fn insert_file(&mut self, file: IFile) -> Result<(), anyhow::Error> {
        if let Some(root) = &mut self.root {
            root.insert_file(&*self.current_path.to_owned().unwrap(), file)?;
        }
        Ok(())
    }

    fn load_language(&mut self, line: &str) {
        // 检查用的系统语言
        if REGEX_ZH.is_match(line) {
            self.keywords = Some(Box::new(Zh::default()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        static ref TEST_DATA_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    }

    #[test]
    fn test_file_list() {
        // let mut d = TEST_DATA_PATH.clone();
        // d.push("test/list.txt");
        let d = PathBuf::from("Z:\\list.txt");
        let file = File::open(d).unwrap();
        let f = DirSMemParser::new();
        let r = f.parse(file).unwrap();
        println!("{:#?}", r.unwrap());
    }

    #[test]
    fn test_dir_path_match() {
        let mut d = TEST_DATA_PATH.clone();
        d.push("test/path_match.txt");
        let file = File::open(d).unwrap();
        let buf_lines = BufReader::new(file).lines();

        let mut rs = vec![];
        for ref line_result in buf_lines {
            if let Ok(line) = line_result {
                if let Some(s) = REGEX_DIR_PATH.find(line) {
                    rs.push(s.as_str().to_owned());
                }
            }
        }

        assert_eq!(4, rs.len());
        assert_eq!("Z:\\111", rs[0]);
        assert_eq!("Z:\\aaaa\\aaaaadd", rs[1]);
        assert_eq!("Z:\\pppppp\\好好", rs[2]);
        assert_eq!("Z:\\对\\aaa\\好玩", rs[3]);
    }

    #[test]
    fn test_regex_size() {
        let s = "17 个文件     17,641,724 字节";
        let mats = REGEX_SIZE.find_iter(s);
        let v = mats.map(|t| t.as_str()).collect::<Vec<&str>>();
        assert_eq!("17", v[0]);
        assert_eq!("17,641,724", v[1]);
    }
}
