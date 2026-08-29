use crate::{
    app::remote_add_screen::RemoteAddScreen,
    core::{
        config::{manage, types::DatabaseConfig},
        editor::open_editor::open_editor_recover_text,
        globals::get_global_config_file_path,
    },
};

pub fn list() {
    let file_path = get_global_config_file_path();
    let res = manage::read_databases(file_path.to_owned());

    match res {
        Ok(databases) => {
            println!("Databases: ");
            for database in databases {
                println!(" {}", database.name);
            }
        }
        Err(e) => eprintln!("Failed to list remote configs: {e}"),
    }
}

pub fn remove(name: &str) {
    let file_path = get_global_config_file_path();
    let res = manage::remove_database(file_path.to_owned(), name.to_owned());

    match res {
        Ok(_) => println!("Remote config removed successfully. Database: {}", name),
        Err(e) => eprintln!("Failed to remove remote config: {e}"),
    }

    return;
}

pub fn switch(name: &str) {
    let file_path = get_global_config_file_path();
    let res = manage::change_active_database(file_path.to_owned(), name.to_owned());

    match res {
        Ok(_) => println!("Switched active connection to {}", name),
        Err(e) => eprintln!("Failed to switch active connection: {e}"),
    }

    return;
}

pub fn add(name: &str, connection_string: Option<&str>, database_type: Option<&str>) {
    let connection_string = match connection_string {
        Some(connection_string) => connection_string.to_owned(),
        None if database_type.is_some() => {
            let template = match database_type {
                Some("mysql") => "mysql://username:password@localhost:3306/database",
                Some("postgres" | "postgresql") => {
                    "postgres://username:password@localhost:5432/database"
                }
                error => {
                    let error = error.unwrap_or_default().to_string();
                    eprintln!("Unsupported database type: {error}");
                    return;
                }
            };

            match open_editor_recover_text(template) {
                Ok(Some(connection_string)) if !connection_string.trim().is_empty() => {
                    connection_string.trim().to_owned()
                }
                Ok(_) => return,
                Err(error) => {
                    eprintln!("Failed to open connection string editor: {error}");
                    return;
                }
            }
        }
        None => {
            let mut screen = RemoteAddScreen::new();
            match ratatui::run(|terminal| screen.run(terminal)) {
                Ok(Some(connection_string)) => connection_string,
                Ok(None) => return,
                Err(error) => {
                    eprintln!("Failed to open remote connection form: {error}");
                    return;
                }
            }
        }
    };

    let file_path = get_global_config_file_path();
    let res = manage::add_database(
        file_path.to_owned(),
        DatabaseConfig {
            name: name.to_owned(),
            connection_string,
        },
    );

    match res {
        Ok(_) => println!("Remote config added successfully. Database: {}", name),
        Err(e) => eprintln!("Failed to add remote config: {e}"),
    }
}
