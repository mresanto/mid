use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MidConfigFile {
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
            databases: Vec::new(),
        }
    }
}
