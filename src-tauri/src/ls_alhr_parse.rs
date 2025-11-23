use std::sync::Arc;
use anyhow::bail;
use log::{warn, info};

use crate::{i18n::{KeywordLibray, match_lang}, os::Os, Parser, command::ParseCommand, dir::IDir, utils, file::IFile};

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
enum LsAlhrParseToken {
    Path, // .:
    Total, // 总用量 24K
    Item, // drwxrwxr-x  4 sdtttttt sdtttttt 4.0K  4月 19 15:56 .
}

pub struct LsAlhrParser {
    keywords: Option<Box<dyn KeywordLibray>>,
    mode: LsAlhrParseToken,
    root_path: String,
    current_path: Option<String>,
    current_dir: Option<IDir>,
    db: Arc<sled::Db>,
    command: ParseCommand
}

impl LsAlhrParser {

    pub fn new(db: Arc<sled::Db>) -> Self {
        let command = ParseCommand::LsAlhr;
        Self {
            root_path: ".".into(),
            current_path: None,
            current_dir: None,
            keywords: None,
            mode: LsAlhrParseToken::Path,
            db,
            command,
        }
    }

    fn init_dir(&mut self) -> Result<(), anyhow::Error> {
        let path = self.current_path.as_ref().unwrap();
        // 初始化一个新的Dir
        let dir = IDir::new(path, Os::from_command(&self.command));
        self.current_dir = Some(dir);
        Ok(())
    }

    fn init_dir_size(&mut self, size: &str) -> Result<(), anyhow::Error> {
        match self.current_dir {
            Some(ref mut dir) => {
                dir.size = Some(size.to_owned());
            }
            None => {
                warn!("init_dir_size: 目录不存在？离谱！");
                bail!("init_dir_size: 目录不存在？离谱！")
            }
        }
        Ok(())
    }

    fn add_file(&mut self, file: IFile) -> Result<(), anyhow::Error> {
        match self.current_dir {
            Some(ref mut dir) => {
                dir.files.push(file);
            }

            None => { 
                warn!("add_file: {:?} 目录不存在？离谱！", file);
                bail!("add_file: {:?} 目录不存在？离谱！", file)
            }
        }
        Ok(())
    }

    fn save_dir(&mut self) -> Result<(), anyhow::Error> {
        let path = self.current_path.as_ref().unwrap();
        let dir = self.current_dir.as_ref().unwrap();
        self.db.insert(path, serde_json::to_vec(dir)?)?;
        self.current_dir = None; 
        Ok(())
    }

    //fn find_dir(&mut self) -> Result<(), anyhow::Error> {
    //    let path = self.current_path.as_ref().unwrap();
    //        // 初始化一个新的Dir，序列化插入
    //        let dir = IDir::new(path, Os::from_command(&self.command));
    //        self.db.insert(path, serde_json::to_vec(&dir)?)?;
    //    Ok(())
    //}

    //fn write_dir_size(&mut self, size: &str) -> Result<(), anyhow::Error> {
    //    let path = self.current_path.as_ref().unwrap();
    //    match self.db.get(path)? {
    //        Some(ref iv) => {
    //            let s = utils::ivec_to_str(iv);
    //            let mut dir = serde_json::from_str::<IDir>(s)?;
    //            dir.size = Some(size.to_owned());
    //            self.db.insert(path, serde_json::to_vec(&dir)?)?;
    //        }
    //        None => {
    //            bail!("目录键不存在？离谱！{}", path)
    //        }
    //    }
    //    Ok(())
    //}

    //fn insert_file(&mut self, file: IFile) -> Result<(), anyhow::Error> {
    //    let path = self.current_path.as_ref().unwrap();
    //    match self.db.get(path)? {
    //        Some(ref iv) => {
    //            let s = utils::ivec_to_str(iv);
    //            let mut dir = serde_json::from_str::<IDir>(s)?;
    //            dir.files.push(file);
    //            self.db.insert(path, serde_json::to_vec(&dir)?)?;
    //        }

    //        None => {
    //            bail!("目录键不存在？离谱！{}", path)
    //        }
    //    }
    //    Ok(())
    //}

    fn try_load_language(&mut self, text: &str) -> bool{
        // 对改行语言匹配，装载对应的关键词库
        if let Some(k) = match_lang(text, &self.command) {
            self.keywords = Some(k);
            return true
         }
    false
}
}

impl Parser for LsAlhrParser {
    fn parse_line(&mut self, line: &str, line_number: &usize) -> anyhow::Result<()> {
        // 跳过空行
        if line.is_empty() {
            if self.mode == LsAlhrParseToken::Item {
                self.save_dir()?;
            }
            // 同时匹配模式改为路径
            self.mode = LsAlhrParseToken::Path;
            return Ok(());
        }

        // 只检查前3行
        if self.keywords.is_none() && *line_number < 3 {
            // 对改行语言匹配，装载对应的关键词库
            self.try_load_language(line);
            // 没检测到就不退出了, ls -alhr 暂时没有对语言的依赖
        }

        if *line_number > 3 && self.current_path.is_none() {
            bail!("3行没有找到起始路径，退出")
        }

        match self.mode {
            LsAlhrParseToken::Path => {
                if line.ends_with(":") {
                    // 去掉结尾的冒号
                    let path = line[..line.len() -1].into();
                    info!("[{}] {}", line_number, path);
                    self.current_path = Some(path);
                    self.init_dir()?;
                    self.mode = LsAlhrParseToken::Total;
                }
            },

            LsAlhrParseToken::Total => {
                let total_line_vec = line.split(" ").filter(|t| !t.trim().is_empty()).collect::<Vec<&str>>();
                // 长度必须是2
                debug_assert_eq!(total_line_vec.len(), 2);
                self.init_dir_size(total_line_vec[1])?;
                self.mode = LsAlhrParseToken::Item;
            },

            LsAlhrParseToken::Item => {
                // 说明是文件夹，跳过
                if line.starts_with("d") {
                    return Ok(());
                }
                let item_line_vec = line.split(" ").filter(|t| !t.trim().is_empty()).collect::<Vec<&str>>();
                // 文件名可能有空格，或者是链接的形式
                debug_assert!(item_line_vec.len() >= 9);
                self.add_file(IFile::from_line_vec_for_ls_alhr(item_line_vec))?;
            },
        }

        Ok(())
    }

    fn root_path(&mut self) -> anyhow::Result<String> {
        self.db.flush()?;
        Ok(self.root_path.to_owned())
    }
}


#[cfg(test)]
mod tests  {

    use std::{path::PathBuf, fs::File};

    use lazy_static::lazy_static;

    use crate::{kv::{create_file_db, FileListDb}, dir_s_parse::DirSParser, command::ParseCommand, ls_alhr_parse::LsAlhrParser};
    use crate::Parser;

    lazy_static! {
        static ref TEST_DATA_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    }

    #[test]
    fn test_total_line() {
        let line = " 总用量 24K";
        let total_line_vec = line.split(" ").filter(|t| !t.trim().is_empty()).collect::<Vec<&str>>();
        assert_eq!("24K", total_line_vec[1]);
    }

    #[test]
    fn test_file_list() {
        let mut d = TEST_DATA_PATH.clone();
        d.push("test/ls-alhr-dist.txt");
        let (_, db) = create_file_db(d.to_str().unwrap()).unwrap();
        let file = File::open(d).unwrap();
        let mut f = LsAlhrParser::new(db.to_owned());
        f.parse(file).unwrap();
        let root = f.root_path().unwrap();
        let file_list = FileListDb::new(db, ParseCommand::LS_ALHR_COMMAND).unwrap();
        println!("{:#?}", file_list.dir_info(&root).unwrap());
        println!("{:#?}", file_list.find_file("index"));
    }
}
