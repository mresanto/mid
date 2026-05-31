/// The config file name.
pub const CONFIG_FILE_NAME: &str = ".midconfig.toml";

pub fn get_global_config_file_path() -> String {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_file_path = home_dir.join(CONFIG_FILE_NAME);
    config_file_path.to_str().unwrap().to_string()
}
