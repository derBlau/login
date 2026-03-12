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
    pub fn validate_username(username: &str) -> Result<(), ValidationError<'_>> {
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
    /// * [`ValidationError::TooShort`](crate::errors::validation_errors::ValidationError::TooShort)
    ///   if the username is less than 6 chars long
    /// * [`ValidationError::TooLong`](crate::errors::validation_errors::ValidationError::TooLong)
    ///   if the username is longer than 14 chars
    fn check_length_of_username(username_size: usize) -> Result<(), ValidationError<'static>> {
        if username_size < 6 {
            Err(ValidationError::TooShort { field: "username" })
        } else if username_size > 14 {
            Err(ValidationError::TooLong { field: "username" })
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
    /// * `OK(())`
    /// * if username passes the check
    /// * [`ValidationError::ForbiddenChar`](crate::errors::validation_errors::ValidationError::ForbiddenChar)
    ///   if the username contains forbidden characters
    fn username_contains_only_allowed_chars(username: &str) -> Result<(), ValidationError<'_>> {
        if username
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '!' | '#' | '@'))
        {
            Ok(())
        } else {
            Err(ValidationError::ForbiddenChar { field: "username" })
        }
    }

    /// checks that a password is of an appropriate length and does not
    /// contain forbidden chars
    ///
    /// # Arguments
    ///
    /// * `password` - a reference to the string that is to be checked
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    ///   if the password passes the check
    /// * [`ValidationError`](crate::errors::validation_errors::ValidationError)
    ///   if the password does not pass the check
    pub fn validate_password(password: &str) -> Result<(), ValidationError<'_>> {
        Self::check_length_of_password(password.len())?;
        Self::password_contains_only_allowed_chars(password)?;

        Ok(())
    }

    /// checks that a password is of an appropriate length
    ///
    /// # Arguments
    ///
    /// * `password_size` - the length of the password that is to be checked
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    ///   if the password passes the check
    /// * [`ValidationError::TooShort`](crate::errors::validation_errors::ValidationError::TooShort)
    ///   if the password is less than 8 characters long
    /// * [`ValidationError::TooLong`](crate::errors::validation_errors::ValidationError::TooLong)
    ///   if the password is longer than 32 characters
    fn check_length_of_password(password_size: usize) -> Result<(), ValidationError<'static>> {
        if password_size < 8 {
            Err(ValidationError::TooShort { field: "password" })
        } else if password_size > 32 {
            Err(ValidationError::TooLong { field: "password" })
        } else {
            Ok(())
        }
    }

    /// checks that a password does not contain forbidden characters
    ///
    /// # Arguments
    ///
    /// * `password` - a reference to the string that is to be checked
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    ///   if the password passes the check
    /// * [`ValidationError::ForbiddenChar`](crate::errors::validation_errors::ValidationError::ForbiddenChar)
    ///   if the password contains forbidden characters
    fn password_contains_only_allowed_chars(password: &str) -> Result<(), ValidationError<'_>> {
        if password
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '!' | '#' | '@' | '$'))
        {
            Ok(())
        } else {
            Err(ValidationError::ForbiddenChar { field: "password" })
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
        assert_eq!(
            res.unwrap_err(),
            ValidationError::TooShort { field: "username" }
        );
    }

    #[test]
    fn should_fail_when_username_is_too_long() {
        let username = "ThisIsAVeryLongUsernameThatShouldNotPassTheTest01!@";
        let res = Validator::validate_username(username);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ValidationError::TooLong { field: "username" }
        );
    }

    #[test]
    fn shoud_fail_when_username_contains_invalid_chars() {
        let username = "@illformed01$$";
        let res = Validator::validate_username(username);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ValidationError::ForbiddenChar { field: "username" }
        );
    }

    #[test]
    fn should_pass_when_username_is_valid() {
        let username = "testUser01!";
        let res = Validator::validate_username(username);

        assert!(res.is_ok());
    }

    #[test]
    fn should_fail_when_password_is_too_short() {
        let password = "pwd1$";
        let res = Validator::validate_password(password);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ValidationError::TooShort { field: "password" }
        );
    }

    #[test]
    fn should_fail_when_password_is_too_long() {
        let password =
            "thisisanexceedinglylongPasswordthatSHOULDNEVERpassthetestatall01234567890123456789";
        let res = Validator::validate_password(password);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ValidationError::TooLong { field: "password" }
        );
    }

    #[test]
    fn should_fail_when_pasword_contains_forbidden_chars() {
        let password = "testpass01/";
        let res = Validator::validate_password(password);

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ValidationError::ForbiddenChar { field: "password" }
        );
    }

    #[test]
    fn should_pass_when_password_is_valid() {
        let password = "ValidPass01$!";
        let res = Validator::validate_password(password);

        assert!(res.is_ok());
    }
}
