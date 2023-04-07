use serde::{Deserialize, Serialize};

use crate::{
    file::IFile,
    utils::{self, join_path_vec},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IDir {
    #[serde(rename = "n")] // 尽可能减少JSON体积
    pub path: String, // 除了根路径，子路径一律不包含“/”路径分隔符
    #[serde(rename = "f")]
    pub files: Vec<IFile>,
    #[serde(rename = "d")]
    pub dirs: Vec<IDir>,
    #[serde(rename = "s")]
    pub size: Option<String>,
}

impl IDir {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            files: vec![],
            dirs: vec![],
            size: None,
        }
    }

    pub fn find_path(&mut self, path: &str) -> Result<(), anyhow::Error> {
        self.parse_path_operation(path, None::<Box<dyn FnOnce(&mut IDir)>>)?;
        Ok(())
    }

    pub fn insert_file(&mut self, path: &str, file: IFile) -> Result<(), anyhow::Error> {
        self.parse_path_operation(
            path,
            Some(Box::new(|i: &mut IDir| {
                i.files.push(file);
            })),
        )?;
        Ok(())
    }

    pub fn write_size(&mut self, path: &str, size: &str) -> Result<(), anyhow::Error> {
        self.parse_path_operation(
            path,
            Some(Box::new(|i: &mut IDir| {
                i.size = Some(size.to_owned());
            })),
        )?;
        Ok(())
    }

    // 返回bool: 代表的是是否命中
    fn parse_path_operation(
        &mut self,
        path: &str,
        op: Option<Box<impl FnOnce(&mut Self) + ?Sized>>,
    ) -> Result<bool, anyhow::Error> {
        // 路径的开头肯定包含自己的路径
        debug_assert!(path.starts_with(&self.path));
        // 去掉自己以外的
        let sub_path = &path[self.path.len()..];
        let sub_path_vec = utils::split_path(sub_path);
        // 为空，说明自己就是最后的子路径
        if sub_path.is_empty() || sub_path_vec.is_empty() {
            if let Some(op_fn) = op {
                op_fn(self);
            }
            return Ok(true);
        }

        // 获取目标子目录下标
        let mut sub_dir_index = -1;
        for (idx, sub_dir) in self.dirs.iter().enumerate() {
            if &sub_dir.path == &sub_path_vec[0] {
                sub_dir_index = idx as i32;
            }
        }

        if sub_dir_index == -1 {
            // 如果不存在该目录就创建，然后交给这个目录去解析下面的结构
            let dir = IDir::new(&sub_path_vec[0]);
            self.dirs.push(dir);
            // push进去下标肯定是最后一位
            sub_dir_index = (self.dirs.len() - 1) as i32;
        }
        self.dirs[sub_dir_index as usize].parse_path_operation(&join_path_vec(sub_path_vec), op)
    }
}
