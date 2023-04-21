use anyhow::bail;

pub enum ParseCommand {
    DirS,
    LsAlhr
}

impl ParseCommand {
    pub const DIR_S_COMMAND: &str =  "dir /s *.*";
    pub const LS_ALHR_COMMAND: &str = "ls -alhR";
}

impl From<ParseCommand> for String {  
    fn from(platform: ParseCommand) -> Self {  
        match platform{  
            ParseCommand::DirS => ParseCommand::DIR_S_COMMAND.into(),  
			ParseCommand::LsAlhr => ParseCommand::LS_ALHR_COMMAND.into(),  
		}  
    }  
}

impl TryFrom<String> for ParseCommand{  
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &*value {  
            Self::DIR_S_COMMAND => Ok(ParseCommand::DirS),
			Self::LS_ALHR_COMMAND => Ok(ParseCommand::LsAlhr),
            _ => bail!(format!("未知的命令: {}", value))
		 }  
    }  
}
