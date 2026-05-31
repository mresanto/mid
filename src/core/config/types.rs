use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MidConfigFile {
    pub active_remote: Option<String>,
    pub databases: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub name: String,
    pub connection_string: String,
}

impl Default for MidConfigFile {
    fn default() -> Self {
        Self {
            active_remote: None,
            databases: Vec::new(),
        }
    }
}
