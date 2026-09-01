use std::ffi::OsStr;

use clap_complete::CompletionCandidate;

use crate::core::{config::manage, globals::get_global_config_file_path};

pub fn complete_remotes(current: &OsStr) -> Vec<CompletionCandidate> {
    let prefix = current.to_string_lossy();
    let config = manage::read_config(get_global_config_file_path()).ok();

    config
        .into_iter()
        .flat_map(|config| config.databases)
        .filter(|database| database.name.starts_with(prefix.as_ref()))
        .map(|database| {
            CompletionCandidate::new(database.name).help(Some("Database remote".into()))
        })
        .collect()
}
