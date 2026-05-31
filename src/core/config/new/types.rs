/// This file contains the types used in the application.
pub struct CreateNewConfigOptions {
    /// The connection unique display name.
    pub name: String,

    /// The Database Connection String.
    pub connection_string: String,

    /// If the new config should be added on a global file
    pub global: bool,
}
