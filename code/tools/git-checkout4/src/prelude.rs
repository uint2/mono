pub(crate) use crate::context::AppCtx;
pub(crate) use crate::data::{Branch, Bundle, Worktree};
pub(crate) use crate::git_config::GitConfig;

pub(crate) use core::{fmt, str};

pub(crate) use std::collections::HashMap;
pub(crate) use std::io::Write;
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::process::{Command, ExitCode};
pub(crate) use std::sync::Mutex;
pub(crate) use std::{env, io};

#[derive(Clone, Copy)]
pub struct AppConfig {
    pub enable_logging: bool,
    pub log_level: log::LevelFilter,
    pub interactive: bool,
}
