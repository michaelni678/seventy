//! The example on the README.

use seventy::{
    builtins::{compare::*, string::*},
    seventy, Newtype,
};

#[seventy(
    upgrades(display),
    sanitize(trim),
    validate(alphanumeric, length::chars(within(5..=20))),
)]
pub struct Username(String);

fn main() {
    // Trims and then fails the alphanumeric validation.
    assert!(Username::try_new("   u$ername   ").is_err());

    // Trims and then fails the alphanumeric validation.
    assert!(Username::try_new("u$ername").is_err());

    // Trims and then fails the length validation.
    assert!(Username::try_new("   user   ").is_err());

    // Trims and then fails the length validation.
    assert!(Username::try_new("user").is_err());

    // Trims and then passes all validations.
    assert_eq!(
        Username::try_new("   username   ").unwrap().into_inner(),
        "username"
    );

    // Trims and then passes all validations.
    assert_eq!(
        Username::try_new("username").unwrap().into_inner(),
        "username"
    );
}
