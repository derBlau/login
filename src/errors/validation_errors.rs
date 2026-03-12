use thiserror::Error;

/// This enum contains all of the possible errors
/// that can be thrown when validating user input
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError<'a> {
    #[error("{field} is too short")]
    TooShort { field: &'a str },
    #[error("{field} is too long")]
    TooLong { field: &'a str },
    #[error("{field} contains forbidden characters")]
    ForbiddenChar { field: &'a str },
}
