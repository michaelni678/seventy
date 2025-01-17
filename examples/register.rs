//! A realistic example using the newtypes.

use serde::Deserialize;
use seventy::{
    builtins::{compare::*, email::*, string::*},
    seventy,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: Email,
    username: Username,
    password: Password,
}

#[seventy(
    upgrades(deserializable, inherent, serializable, unexposed),
    sanitize(trim),
    validate(email)
)]
pub struct Email(String);

#[seventy(
    upgrades(deserializable, inherent, unexposed),
    sanitize(trim),
    validate(alphanumeric, length::chars(within(5..=20)))
)]
pub struct Username(String);

#[seventy(
    upgrades(deserializable, inherent, unexposed),
    validate(ascii, length::chars(gt(8)))
)]
pub struct Password(String);

fn main() {
    // Successfully constructs the newtype.
    {
        let json = r#"
            {
                "email": "   seventy70@example.com   ",
                "username": "Seventy70   ",
                "password": "p455w0rd70!"
            }
        "#;

        let result = serde_json::from_str(json);
        assert!(result.is_ok());

        let request: RegisterRequest = result.unwrap();

        assert_eq!(request.email.into_inner(), "seventy70@example.com");
        assert_eq!(request.username.into_inner(), "Seventy70");
        assert_eq!(request.password.into_inner(), "p455w0rd70!");
    }

    // Fails password validation (\u{7070} is not ASCII).
    {
        let json = r#"
            {
                "email": "   seventy70@example.com   ",
                "username": "Seventy70   ",
                "password": "p455w0rd\u{7070}!"
            }
        "#;

        let result: Result<RegisterRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
