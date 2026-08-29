use crate::core;

pub fn list() {
    let file_path = core::globals::get_global_history_file_path();
    let res = core::history::read_history(file_path);

    match res {
        Ok(history) => {
            for request in history.requests {
                println!("id: {}", request.id);
                println!("query: {}", request.query);
                println!("created_at: {}", request.created_at);
                println!("database: {}", request.database);
                println!("");
            }
        }
        Err(e) => eprintln!("No history found: {e}"),
    }
}

pub fn last() {
    let config_file_path = core::globals::get_global_config_file_path();
    let config = match core::config::manage::read_config(config_file_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read config: {e}");
            return;
        }
    };
    let active_database = match config.get_active_database() {
        Some(database) => database,
        None => {
            eprintln!("No active remote connection");
            return;
        }
    };

    let file_path = core::globals::get_global_history_file_path();
    let res = core::history::read_history(file_path);

    match res {
        Ok(history) => match history
            .requests
            .iter()
            .rev()
            .find(|request| request.database == active_database.name)
        {
            Some(request) => {
                println!("id: {}", request.id);
                println!("query: {}", request.query);
                println!("created_at: {}", request.created_at);
                println!("database: {}", request.database);
                println!("");
            }
            _ => println!("No history found for active database"),
        },
        Err(e) => eprintln!("No history found: {e}"),
    }
}
