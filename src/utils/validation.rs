use crate::errors::validation_errors::ValidationError;

/// This struct is used to encapsulate all the logic
/// that is used to validate user input: username and
/// password.
pub struct Validator {}

impl Validator {
    /// checks that the username is of an appropriate length and that it
    /// does not contain any forbidden characters
    ///
    /// # Arguments
    ///
    /// * `username` - a reference to the string that is to be validated
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    ///   if the username passes the check
    /// * [`ValidationError`](crate::errors::validation_errors::ValidationError)
    ///   if the username is too long and/or contains forbidden chars
    pub fn validate_username(username: &str) -> Result<(), ValidationError> {
        Self::check_length_of_username(username.len())?;
        Self::username_contains_only_allowed_chars(username)?;

        Ok(())
    }

    /// checks that the username is of an appropriate length
    ///
    /// # Arguments
    ///
    /// * `username_size` - the size of the username string that is to be checked
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    ///   if the length passes the check
    /// * [`ValidationError::TooShort`](crate::errors:validation_errors::ValidationError::TooShort)
    ///   if the username is less than 6 chars long
    /// * [`ValidationError::TooLong`](crate::errors:validation_errors::ValidationError::TooLong)
    ///   if the username is longer than 14 chars
    fn check_length_of_username(username_size: usize) -> Result<(), ValidationError> {
        if username_size < 6 {
            Err(ValidationError::TooShort)
        } else if username_size > 14 {
            Err(ValidationError::TooLong)
        } else {
            Ok(())
        }
    }

    /// checks that the username does not contain forbidden characters
    ///
    /// # Arguments
    ///
    /// * `username` - a reference to the string that is to be checked
    ///
    /// # Returns
    ///
    /// * `OK(())` if the username passes the check
    /// *
    /// * [`ValidationError::ForbiddenChar`](crate::errors:validation_errors:ValidationError::ForbiddenChar)
    ///   if the username contains forbidden characters
    fn username_contains_only_allowed_chars(username: &str) -> Result<(), ValidationError> {
        if username
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '!' | '#' | '@'))
        {
            Ok(())
        } else {
            Err(ValidationError::ForbiddenChar)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fail_when_username_is_too_short() {
        let username = "t!1";
        let res = Validator::validate_username(username);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ValidationError::TooShort);
    }

    #[test]
    fn should_fail_when_username_is_too_long() {
        let username = "ThisIsAVeryLongUsernameThatShouldNotPassTheTest01!@";
        let res = Validator::validate_username(username);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ValidationError::TooLong);
    }

    #[test]
    fn shoud_fail_when_username_contains_invalid_chars() {
        let username = "@illformed01$$";
        let res = Validator::validate_username(username);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ValidationError::ForbiddenChar);
    }

    #[test]
    fn should_pass_when_username_is_valid() {
        let username = "testUser01!";
        let res = Validator::validate_username(username);

        assert!(res.is_ok());
    }
}
