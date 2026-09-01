use std::ffi::OsStr;

use clap_complete::CompletionCandidate;

use crate::core::{config::manage, globals::get_global_config_file_path};

pub fn complete_remotes(current: &OsStr) -> Vec<CompletionCandidate> {
    let prefix = current.to_string_lossy();
    let Ok(config) = manage::read_config(get_global_config_file_path()) else {
        return Vec::new();
    };
    let active_remote = config.active_remote;

    config
        .databases
        .into_iter()
        .filter(|database| database.name.starts_with(prefix.as_ref()))
        .map(|database| {
            let help = database.get_database_type().ok().map(|db_type| {
                if active_remote.as_deref() == Some(database.name.as_str()) {
                    format!("{db_type} (Active)").into()
                } else {
                    db_type.to_string().into()
                }
            });

            CompletionCandidate::new(database.name).help(help)
        })
        .collect()
}
