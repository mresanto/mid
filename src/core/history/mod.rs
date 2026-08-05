use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct MidHistoryFile {
    pub requests: Vec<HistoryRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryRequest {
    pub id: u16,
    pub query: String,
    pub database: String,
    pub created_at: String,
}

impl Default for HistoryRequest {
    fn default() -> Self {
        Self {
            id: 0,
            query: String::new(),
            database: String::new(),
            created_at: String::new(),
        }
    }
}

impl Default for MidHistoryFile {
    fn default() -> Self {
        Self {
            requests: Vec::new(),
        }
    }
}

impl MidHistoryFile {
    pub fn request_exists(&self, id: u16) -> bool {
        self.requests.iter().any(|request| request.id == id)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse global history file: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Failed to serialize global history file: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Failed to read global history file: {0}")]
    Io(#[from] io::Error),

    #[error("History request already exists: {0}")]
    RequestAlreadyExists(u16),

    #[error("History request not found: {0}")]
    RequestNotFound(u16),
}

pub fn read_history(file_path: String) -> Result<MidHistoryFile, Error> {
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(MidHistoryFile::default()),
        Err(e) => return Err(Error::Io(e)),
    };

    let history = toml::from_str::<MidHistoryFile>(&contents)?;

    return Ok(history);
}

pub fn last_history_or_default(file_path: String) -> Result<HistoryRequest, Error> {
    let history = read_history(file_path)?;
    let last_or_default = history.requests.last();
    match last_or_default {
        Some(request) => Ok(request.clone()),
        None => Ok(HistoryRequest::default()),
    }
}

pub fn get_history_id(file_path: String, id: &u16) -> Result<Option<HistoryRequest>, Error> {
    let history = read_history(file_path)?;
    let request = history.requests.iter().find(|r| r.id == *id);
    Ok(request.cloned())
}

pub fn save_history(file_path: String, content: MidHistoryFile) -> Result<(), Error> {
    let history_string = toml::to_string_pretty(&content)?;
    fs::write(file_path, history_string)?;

    return Ok(());
}

pub fn add_request(file_path: String, request: HistoryRequest) -> Result<(), Error> {
    let mut history = read_history(file_path.clone())?;

    if history.request_exists(request.id) {
        return Err(Error::RequestAlreadyExists(request.id));
    }

    history.requests.push(request);

    save_history(file_path, history)?;

    return Ok(());
}

#[allow(dead_code)]
pub fn remove_request(file_path: String, id: u16) -> Result<(), Error> {
    let mut history = read_history(file_path.clone())?;

    if !history.request_exists(id) {
        return Err(Error::RequestNotFound(id));
    }

    history.requests.retain(|request| request.id != id);

    save_history(file_path, history)?;

    return Ok(());
}
