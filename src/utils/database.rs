use crate::utils::user::User;
use rusqlite::{Connection, OptionalExtension, Result, params};
use std::fs;
use std::path::PathBuf;

/// This struct represents the file for user data storage
pub struct Database {
    /// This field represents the connection to the database file
    /// which is generated upon creating an instance of [`Database`]
    connection: Connection,
}

impl Database {
    /// creates an instance of [`Database`]
    ///
    /// # Arguments
    ///
    /// * `path` - a reference to a string representing the filepath to the database file
    ///
    /// # Returns
    ///
    /// * [`Database`]
    ///
    /// # Panics
    ///
    /// Panics if either the directories cannot be created or the database connection fails
    pub fn new(path: &str) -> Self {
        let db_path = PathBuf::from(path);

        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("should be able to generate directories");
            }
        }

        let connection = Connection::open(&db_path)
            .expect("should be able to establish connection with database");

        Self { connection }
    }

    /// adds a new user to the database
    ///
    /// # Arguments
    ///
    /// * `user` - the username and password linked to an account in an instance of
    /// [`User`](crate::utils::user::User)
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the entry could be added to the database
    /// * [`rusqlite::Error`] if the operation fails
    pub fn add_new_user(&self, user: User) -> Result<()> {
        let con = &self.connection;
        let mut stmt =
            con.prepare("INSERT OR IGNORE INTO users(username, password) values (?1, ?2)")?;
        stmt.execute(params![user.get_username(), user.get_password()])?;

        Ok(())
    }

    /// searches for user in the database
    ///
    /// # Arguments
    ///
    /// * `username` - a reference to a string represent the name of the account
    ///
    /// # Returns
    ///
    /// * `Ok(Some(User))` if the operation was processed without errors and a user was found
    /// * `Ok(None)` if the operation was processed without errors, but no user was found
    /// * [`rusqlite::Error`] if the operation failed
    pub fn search_user(&self, username: &str) -> Result<Option<User>> {
        let con = &self.connection;
        let mut stmt = con.prepare("SELECT * from users WHERE username = ?1")?;

        stmt.query_row(params!(username), |row| {
            let username: String = row.get(0)?;
            let password: String = row.get(1)?;

            Ok(User::new(username, password))
        })
        .optional()
    }
}
