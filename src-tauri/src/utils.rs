use std::hash::{Hash, Hasher};

use sled::IVec;

use crate::{os::Os};

pub fn split_path(path: &str, os: &Os) -> Vec<String> {    
    path.split(os.pat())
        .map(|t| t.to_owned())
        .filter(|t| !t.is_empty())
        .collect()
}

pub fn join_path_vec(vec: Vec<String>, os: &Os) -> String {
    vec.join(os.pat())
}

// 分割文件行
pub fn split_space_line(line: &str) -> Vec<&str> {
    line.split(" ")
        .filter(|t| !t.trim().is_empty()) // 排除空
        .map(|t| t.trim())
        .collect()
}

pub fn hash(k: impl Hash) -> String {
    let mut buffer = itoa::Buffer::new();
    let mut hasher = std::collections::hash_map::DefaultHasher::default();
    k.hash(&mut hasher);
    buffer.format(hasher.finish()).to_owned()
}

pub fn ivec_to_str(vec: &IVec) -> &str {
    std::str::from_utf8(vec).expect("ivec转换出错")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_filename_line() {
        let line = "2022/10/14  11:20             1,033 composite-literals-leading-lines.txt";
        let r = split_space_line(line);

        assert_eq!("2022/10/14", r[0]);
        assert_eq!("11:20", r[1]);
        assert_eq!("1,033", r[2]);
        assert_eq!("composite-literals-leading-lines.txt", r[3]);
    }

    #[test]
    fn test_slice() {
        let line = "./awdawd/awdawd:";
        assert_eq!("./awdawd/awdawd", &line[..line.len() -1 ]);
    }
}
