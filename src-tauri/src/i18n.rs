use std::fmt::{Formatter, Display};

use lazy_static::lazy_static;
use regex::Regex;

use crate::command::ParseCommand;

lazy_static! {
    // 中文匹配
    static ref REGEX_ZH_DIR_S: Regex = Regex::new(r"驱动器|系统|卷").unwrap();
    static ref REGEX_ZH_LS_ALHR: Regex = Regex::new(r"总用量").unwrap();
    // 英文匹配
    static ref REGEX_EN_DIR_S: Regex = Regex::new(r"Volume in drive").unwrap();
    static ref REGEX_EN_LS_ALHR: Regex = Regex::new(r"total").unwrap();
    // 日文匹配
    static ref REGEX_JA_DIR_S: Regex = Regex::new(r"のボリューム").unwrap();
}

pub trait KeywordLibray {
    // dir -s 的匹配只会在前5行执行
    fn dir_s_match_lang(line: &str) -> bool where Self: Sized; 
    fn dir_s_file_count(&self) -> &str;
    fn dir_s_dir(&self) -> &str;

    fn ls_alhr_match_lang(line: &str) -> bool where Self: Sized;
}

// 根据字符串返回所属的语言
pub fn match_lang(line: &str, command: &ParseCommand) -> Option<Box<dyn KeywordLibray>> {

    match command {
        ParseCommand::DirS => {
            if Zh::dir_s_match_lang(line) {
                return Some(Box::new(Zh::default()))
            };
            
            if En::dir_s_match_lang(line) {
              return  Some(Box::new(En::default()))
            };
        
            if Ja::dir_s_match_lang(line) {
                return Some(Box::new(Ja::default()))
            }

            None
        },

        ParseCommand::LsAlhr => {
            if Zh::ls_alhr_match_lang(line) {
                return Some(Box::new(Zh::default()))
            }

            if En::ls_alhr_match_lang(line) {
                return Some(Box::new(En::default()))
            }

            None
        },
    }
}


#[derive(Debug, Default)]
pub struct Zh;

impl KeywordLibray for Zh {

    fn dir_s_match_lang(line: &str) -> bool where Self: Sized {
        REGEX_ZH_DIR_S.is_match(line)
    }

    #[inline]
    fn dir_s_file_count(&self) -> &str {
        "个文件"
    }

    #[inline]
    fn dir_s_dir(&self) -> &str {
        "的目录"
    }

    fn ls_alhr_match_lang(line: &str) -> bool where Self: Sized {
       REGEX_ZH_LS_ALHR.is_match(line)
    }
}

#[derive(Debug, Default)]
pub struct En;

impl KeywordLibray for En {
    fn dir_s_match_lang(line: &str) -> bool where Self: Sized {
        REGEX_EN_DIR_S.is_match(line)
    }

    #[inline]
    fn dir_s_file_count(&self) -> &str {
        "File(s)"
    }

    #[inline]
    fn dir_s_dir(&self) -> &str {
        "Directory of"
    }

    fn ls_alhr_match_lang(line: &str) -> bool where Self: Sized {
        REGEX_EN_LS_ALHR.is_match(line)
    }
}

#[derive(Debug, Default)]
pub struct Ja;


impl KeywordLibray for Ja {
    fn dir_s_match_lang(line: &str) -> bool where Self: Sized {
        REGEX_JA_DIR_S.is_match(line)
    }

    #[inline]
    fn dir_s_file_count(&self) -> &str {
        "個のファイル"
    }

    #[inline]
    fn dir_s_dir(&self) -> &str {
        "のディレクトリ"
    }

    // TODO: 日文暂时不支持ls -alhr
    fn ls_alhr_match_lang(line: &str) -> bool where Self: Sized {
        REGEX_EN_LS_ALHR.is_match(line)
    }
}

impl Display for Zh {  
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {  
        write!(f, "中文")  
    }
}

impl Display for En {  
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {  
        write!(f, "英文")
    }
}  

impl Display for Ja {  
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {  
        write!(f, "日文")
    }
}  
