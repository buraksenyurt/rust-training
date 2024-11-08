struct MembershipManager;

impl MembershipManager {
    fn validate_nickname(&self, nick_name: &str) -> Result<(), UserError> {
        if nick_name.len() < 5 {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname is too short",
            )));
        }
        if nick_name.len() > 15 {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname is too long",
            )));
        }
        if nick_name.contains(' ') {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname contains space",
            )));
        }
        if nick_name.chars().any(|c| !c.is_alphanumeric()) {
            return Err(UserError::InvalidNickname(String::from(
                "Nickname contains alphanumeric",
            )));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum UserError {
    InvalidNickname(String),
    WrongPassword,
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::memership::UserError::InvalidNickname;

    #[test]
    fn test_valid_nick_name() {
        let membership_manager = MembershipManager;
        assert!(membership_manager.validate_nickname("rustropic").is_ok());
    }

    #[test]
    fn test_short_username() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("none");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname is too short".to_string()
            ))
        );
    }

    #[test]
    fn test_long_username() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("ToooooLonnnnnnnnngggggggggggNickkkk");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname is too long".to_string()
            ))
        );
    }

    #[test]
    fn test_username_with_space() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("The Red Barron");
        assert_eq!(
            result,
            Err(UserError::InvalidNickname(
                "Nickname contains space".to_string()
            ))
        );
    }

    #[test]
    fn test_username_with_special_chars() {
        let membership_manager = MembershipManager;
        let result = membership_manager.validate_nickname("Rust@Nickname!");
        assert_eq!(
            result,
            Err(InvalidNickname(
                "Nickname contains alphanumeric".to_string()
            ))
        );
    }
}
