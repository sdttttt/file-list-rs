use crate::{dir_s_parse::DirSParser, ls_alhr_parse::LsAlhrParser};
use anyhow::bail;
use log::*;
const WINDOWS_PAT: &str = "\\";
const UNIX_PAT: &str = "/";


#[derive(Debug, Default, Clone, Copy)]
pub enum Os {
    Windows,
    #[default] Unix
}

impl Os {
    pub fn from_command(command: &str) -> anyhow::Result<Self> {
        match &*command {
            DirSParser::COMMAND => Ok(Os::Windows),
            LsAlhrParser::COMMAND => Ok(Os::Unix),
            _ => {
                error!("意外的命令: {}", command);
                bail!(format!("意外的命令: {}", command));
            },
        } 
    }

    pub fn pat(&self) -> &'static str {
        match *self {
            Self::Windows => WINDOWS_PAT,
            Self::Unix => UNIX_PAT,
        }
    }
}
