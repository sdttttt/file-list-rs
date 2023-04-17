use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // 中文匹配
    static ref REGEX_ZH: Regex = Regex::new(r"驱动器|系统|卷").unwrap();

    // 英文匹配
    static ref REGEX_EN: Regex = Regex::new(r"Volume in drive").unwrap();
}

pub trait KeywordLibray {
    fn match_lang(line: &str) -> bool where Self: Sized; 
    fn file_count(&self) -> &'static str;
    fn dir(&self) -> &'static str;
}

// 根据字符串返回所属的语言
pub fn match_lang(line: &str) -> Option<Box<dyn KeywordLibray>> {
    if Zh::match_lang(line) {
        Some(Box::new(Zh::default()))
    }else if En::match_lang(line) {
        Some(Box::new(En::default()))
    } else {
        None
    }
}


#[derive(Debug, Default)]
pub struct Zh;

impl KeywordLibray for Zh {

    #[inline]
    fn match_lang(line: &str) -> bool where Self: Sized {
        REGEX_ZH.is_match(line)
    }

    #[inline]
    fn file_count(&self) -> &'static str {
        "个文件"
    }

    #[inline]
    fn dir(&self) -> &'static str {
        "的目录"
    }
}

#[derive(Debug, Default)]
pub struct En;

impl KeywordLibray for En {
    #[inline]
    fn match_lang(_: &str) -> bool where Self: Sized {
        false
    }

    fn file_count(&self) -> &'static str {
        "File(s)"
    }

    fn dir(&self) -> &'static str {
        "Directory of"
    }
}


