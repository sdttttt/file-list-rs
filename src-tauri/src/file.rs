use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IFile {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub size_text: String,
    #[serde(rename = "t")]
    pub time: String,
}

impl IFile {
    pub fn from_file_line_vec(line_vec: Vec<&str>) -> Self {
        let name = if line_vec.len() > 4 {
            line_vec[3..].join(" ")
        } else {
            line_vec[3].to_owned()
        };

        Self {
            name,
            size_text: line_vec[2].to_owned(),
            time: format!("{} {}", line_vec[0], line_vec[1]),
        }
    }
}
