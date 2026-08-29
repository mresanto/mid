use crate::core::config::{manage, types::DatabaseConfig};

pub fn list(file_path: &str) {
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

pub fn remove(file_path: &str, name: &str) {
    let res = manage::remove_database(file_path.to_owned(), name.clone().to_owned());

    match res {
        Ok(_) => println!("Remote config removed successfully. Database: {}", name),
        Err(e) => eprintln!("Failed to remove remote config: {e}"),
    }

    return;
}

pub fn switch(file_path: &str, name: &str) {
    let res = manage::change_active_database(file_path.to_owned(), name.clone().to_owned());

    match res {
        Ok(_) => println!("Switched active connection to {}", name),
        Err(e) => eprintln!("Failed to switch active connection: {e}"),
    }

    return;
}

pub fn add(file_path: &str, name: &str, connection_string: &str) {
    let res = manage::add_database(
        file_path.to_owned(),
        DatabaseConfig {
            name: name.clone().to_owned(),
            connection_string: connection_string.to_owned(),
        },
    );

    match res {
        Ok(_) => println!("Remote config added successfully. Database: {}", name),
        Err(e) => eprintln!("Failed to add remote config: {e}"),
    }
}
