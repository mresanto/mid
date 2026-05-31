use serde::{Deserialize, Serialize};

use crate::core::databases::adapters::DatabaseType;

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

impl MidConfigFile {
    pub fn get_active_database(&self) -> Option<&DatabaseConfig> {
        let active_db_name = self.active_remote.as_ref()?;

        self.databases.iter().find(|db| db.name == *active_db_name)
    }

    pub fn connection_exists(&self, name: &str) -> bool {
        self.databases.iter().any(|db| db.name == name)
    }

    pub fn set_active_database(&mut self, name: String) {
        self.active_remote = Some(name);
    }

    pub fn get_database_type(&self) -> Option<DatabaseType> {
        let active_db = self.get_active_database()?;

        let database_type = active_db.connection_string.split(':').next()?;

        match database_type {
            "postgres" | "postgresql" => Some(DatabaseType::Postgres),
            "mysql" => Some(DatabaseType::MySQL),
            "sqlite" => Some(DatabaseType::SQLite),
            _ => None,
        }
    }
}
