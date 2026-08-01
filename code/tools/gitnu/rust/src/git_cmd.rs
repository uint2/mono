use crate::{Error, Result, prelude::Aliases};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum GitStatus {
    Short,
    Normal,
}

impl GitStatus {
    pub fn short(&mut self) {
        *self = GitStatus::Short;
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub (crate)enum GitCommand {
    Any(String),
    // full list found from running `git help --all`
    Status(GitStatus),
    Log,
    Version,
}

impl GitCommand {
    pub fn skip_next_arg(&self, arg: &str) -> bool {
        use GitCommand as G;
        match (self, arg) {
            (G::Log, "-n") => true,
            _ => false,
        }
    }

    pub fn from_arg(aliases: &Aliases, arg: String) -> Option<Self> {
        if arg.is_empty() {
            return None;
        }
        let bytes = arg.as_bytes();
        if bytes[0] == b'-' {
            return None; // it's a flag.
        }
        if let Some(arg) = Self::from_resolved_arg(arg.as_str()) {
            return Some(arg);
        }
        if let Some(arg) = aliases.get(&arg) {
            if let Some(arg) = Self::from_resolved_arg(arg.as_str()) {
                return Some(arg);
            }
        }
        Some(Self::Any(arg))
    }

    fn from_resolved_arg(arg: &str) -> Option<Self> {
        match arg {
            "status" => Some(Self::Status(GitStatus::Normal)),
            "log" => Some(Self::Log),
            "version" => Some(Self::Version),
            _ => None,
        }
    }
}

impl TryFrom<String> for GitCommand {
    type Error = Error;
    fn try_from(arg: String) -> Result<Self> {
        use GitCommand::*;
        if arg.is_empty() {
            return Err(Error::NotGitCommand);
        }
        let bytes = arg.as_bytes();
        if bytes[0] == b'-' {
            return Err(Error::NotGitCommand); // it's a flag.
        }
        Ok(match arg.as_str() {
            "status" => Status(GitStatus::Normal),
            "log" => Log,
            _ => Any(arg),
        })
    }
}
