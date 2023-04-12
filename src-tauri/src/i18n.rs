use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // 中文匹配
    static ref REGEX_ZH: Regex = Regex::new(r"[\u4e00-\u9fa5]+").unwrap();
}

pub trait KeywordLibray {
    fn match_lang(line: &str) -> bool where Self: Sized; 
    fn size(&self) -> &'static str;
    fn dir(&self) -> &'static str;
}

// 根据字符串返回所属的语言
pub fn match_lang(line: &str) -> Option<Box<dyn KeywordLibray>> {
    if Zh::match_lang(line) {
        Some(Box::new(Zh::default()))
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
    fn size(&self) -> &'static str {
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

    fn size(&self) -> &'static str {
        ""
    }

    fn dir(&self) -> &'static str {
        ""
    }
}
