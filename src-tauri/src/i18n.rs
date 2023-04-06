pub trait FsKeyword {
    fn size(&self) -> &'static str;
    fn dir(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct Zh;

impl FsKeyword for Zh {
    fn size(&self) -> &'static str {
        "个文件"
    }

    fn dir(&self) -> &'static str {
        "的目录"
    }
}

#[derive(Debug, Default)]
pub struct En;

impl FsKeyword for En {
    fn size(&self) -> &'static str {
        ""
    }

    fn dir(&self) -> &'static str {
        ""
    }
}
