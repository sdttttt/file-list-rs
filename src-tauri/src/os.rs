use crate::{command::ParseCommand};

#[derive(Debug, Default,PartialEq ,Clone, Copy)]
pub enum Os {
    Windows,
    #[default] Unix
}

impl Os {
    pub const WINDOWS_DISK_SYMBOL: &str = ":\\";

    const WINDOWS_PAT: &str = "\\";
    const UNIX_PAT: &str = "/";

    pub fn from_command_str(cmd: &str) -> anyhow::Result<Self> {
        let command = ParseCommand::try_from(cmd.to_owned())?;
        Ok(Self::from_command(&command))
    }

    pub fn from_command(command: &ParseCommand) -> Self {
        match command {
            ParseCommand::DirS => Os::Windows,
            ParseCommand::LsAlhr => Os::Unix,
        } 
    }

    #[inline]
    pub fn pat(&self) -> &'static str {
        match *self {
            Self::Windows => Self::WINDOWS_PAT,
            Self::Unix => Self::UNIX_PAT,
        }
    }
}
