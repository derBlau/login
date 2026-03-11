use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("username is too short")]
    TooShort,
    #[error("username is too long")]
    TooLong,
    #[error("username contains forbidden characters")]
    ForbiddenChar,
}
