/// Represents the name of an account and its password
/// which are stored in the database and are required to
/// log in
pub struct User {
    username: String,
    password: String,
}

impl User {
    /// This function returns an instance of [`User`]
    ///
    /// # Arguments
    ///
    /// * `username` - a string representing the name of the account
    /// * `password` - a string representing the key to log into the account
    ///
    /// # Returns
    ///
    /// * [`User`]
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    /// This function retrieves a reference to the username in [`User::username`]
    ///
    /// # Returns
    ///
    /// * `&str`
    ///   a reference to the string representing a username
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// This function retrieves a reference to the password in [`User::password`]
    ///
    /// # Returns
    ///
    /// * `&str`
    ///   a reference to the string representing a password
    pub fn get_password(&self) -> &str {
        &self.password
    }
}
