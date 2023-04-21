use std::fmt::format;

use serde::{Deserialize, Serialize};

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct IFile {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub size_text: String,
    #[serde(rename = "t")]
    pub time: String,

    #[serde(rename = "c")]
    pub chmod: Option<String>, // unix only
    #[serde(rename = "u")]
    pub user: Option<String>, // unix only
    #[serde(rename = "g")]
    pub group: Option<String>, // unix only
}

impl IFile {

    //2022/11/10  10:59             2,107 utils.pyi
    //2022/11/10  10:59            11,332 _header_value_parser.pyi
    //2022/11/10  10:59             1,060 __init__.pyi
    pub fn from_line_vec_for_dir_s(line_vec: Vec<&str>) -> Self {
        let name = if line_vec.len() > 4 {
            line_vec[3..].join(" ")
        } else {
            line_vec[3].to_owned()
        };

        Self {
            name,
            size_text: line_vec[2].to_owned(),
            time: format!("{} {}", line_vec[0], line_vec[1]),
            ..Default::default()
        }
    }

    // drwxrwxr-x  2 sdtttttt sdtttttt 4.0K 12月 20 15:11 loading
    pub fn from_line_vec_for_ls_alhr(line_vec: Vec<&str>) -> Self {
        let chmod = line_vec[0].into();
        let user = line_vec[2].into();
        let group = line_vec[3].into();
        let size_text = line_vec[4].into();
        let time = format!("{} {} {}", line_vec[5], line_vec[6], line_vec[7]);
        let  name = line_vec[8..].join(" "); 

        Self {
            chmod: Some(chmod),
            user: Some(user),
            group: Some(group),
            size_text,
            time,
            name,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IFile;

    #[test]
    fn test_from_line_vec_for_ls_alhr() {
        let line = "drwxrwxr-x  2 sdtttttt sdtttttt 4.0K 12月 20 15:11 loading";
        let line_vec = line.split(" ").filter(|t| !t.trim().is_empty()).collect::<Vec<&str>>();
        let f = IFile::from_line_vec_for_ls_alhr(line_vec);
        println!("{:#?}", f);
        assert_eq!(f.chmod.as_deref(), Some("drwxrwxr-x"));
    }
}
