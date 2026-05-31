use super::{create, types::CreateNewConfigOptions};

/// Use this method to create a new remote configuration for a user. The remote configuration will
/// be used to connect to the user's remote server and fetch data from it. The remote configuration
/// will be stored in a config file, that can be either global or local, depending on the user's
/// choice. The remote configuration will.
///
/// This function only appends new configs, don't override existing ones. If a config with the same
/// name already exists, it return an error.
pub fn add_remote_config(options: CreateNewConfigOptions) -> Result<(), create::Error> {
    if options.global {
        create::add_remote_config_to_global_file(options)?;
        return Ok(());
    }

    create::add_remote_config_to_local_file(options)?;

    return Ok(());
}
