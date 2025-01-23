use crate::core::Validator;

/// [`Validator`] checks if lowercase.
///
/// # Examples
///
/// The example below validates the inner char is lowercase. Because of the
/// newtype's guarantees, the constructed `LowercaseChar` will always have an
/// inner character that is lowercase.
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(lowercase))]
/// pub struct LowercaseChar(char);
///
/// assert!(LowercaseChar::try_new('c').is_ok());
/// assert!(LowercaseChar::try_new('X').is_err());
/// ```
pub struct lowercase;

impl Validator<char> for lowercase {
    fn validate(&self, target: &char) -> bool {
        target.is_lowercase()
    }
}

/// [`Validator`] checks if uppercase.
///
/// # Examples
///
/// The example below validates the inner char is uppercase. Because of the
/// newtype's guarantees, the constructed `UppercaseChar` will always have an
/// inner character that is uppercase.
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(uppercase))]
/// pub struct UppercaseChar(char);
///
/// assert!(UppercaseChar::try_new('C').is_ok());
/// assert!(UppercaseChar::try_new('x').is_err());
/// ```
pub struct uppercase;

impl Validator<char> for uppercase {
    fn validate(&self, target: &char) -> bool {
        target.is_uppercase()
    }
}
