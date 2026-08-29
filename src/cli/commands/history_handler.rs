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
    let file_path = core::globals::get_global_history_file_path();
    let res = core::history::read_history(file_path);

    match res {
        Ok(history) => match history.requests.last() {
            Some(request) => {
                println!("id: {}", request.id);
                println!("query: {}", request.query);
                println!("created_at: {}", request.created_at);
                println!("database: {}", request.database);
                println!("");
            }
            _ => println!("No history found"),
        },
        Err(e) => eprintln!("No history found: {e}"),
    }
}
