#[cfg(target_os = "windows")]
const FS_PAT: &str = "\\";

#[cfg(target_os = "unix")]
const FS_PAT: &str = "/";

#[inline]
pub fn split_path(path: &str) -> Vec<String> {
    path.split(FS_PAT)
        .map(|t| t.to_owned())
        .filter(|t| !t.is_empty())
        .collect()
}

#[inline]
pub fn join_path_vec(vec: Vec<String>) -> String {
    vec.join(FS_PAT)
}

// 分割文件行
#[inline]
pub fn split_file_line(line: &str) -> Vec<&str> {
    line.split(" ")
        .filter(|t| !t.trim().is_empty()) // 排除空
        .map(|t| t.trim())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_filename_line() {
        let line = "2022/10/14  11:20             1,033 composite-literals-leading-lines.txt";
        let r = split_file_line(line);

        assert_eq!("2022/10/14", r[0]);
        assert_eq!("11:20", r[1]);
        assert_eq!("1,033", r[2]);
        assert_eq!("composite-literals-leading-lines.txt", r[3]);
    }
}
