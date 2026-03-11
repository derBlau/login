use crate::errors::validation_errors::ValidationError;

pub struct Validator {}

impl Validator {
    pub fn validate_username(username: &str) -> Result<(), ValidationError> {
        Self::check_length_of_username(username.len())?;
        Self::username_contains_only_allowed_chars(username)?;

        Ok(())
    }

    fn check_length_of_username(username_size: usize) -> Result<(), ValidationError> {
        if username_size < 6 {
            Err(ValidationError::TooShort)
        } else if username_size > 14 {
            Err(ValidationError::TooLong)
        } else {
            Ok(())
        }
    }

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
