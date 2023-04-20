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
}
