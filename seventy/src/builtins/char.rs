//! Character built-ins.

use crate::core::Validator;

/// [`Validator`] checks if alphabetic.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(alphabetic))]
/// pub struct AlphabeticChar(char);
///
/// // Successfully constructed because 'c' is alphabetic.
/// assert!(AlphabeticChar::try_new('c').is_ok());
///
/// // Unsuccessfully constructed because '0' is not alphabetic.
/// assert!(AlphabeticChar::try_new('0').is_err());
/// ```
pub struct alphabetic;

impl Validator<char> for alphabetic {
    fn validate(&self, target: &char) -> bool {
        target.is_alphabetic()
    }
}

/// [`Validator`] checks if alphanumeric.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(alphanumeric))]
/// pub struct AlphanumericChar(char);
///
/// // Successfully constructed because '0' is alphnaumeric.
/// assert!(AlphanumericChar::try_new('0').is_ok());
///
/// // Unsuccessfully constructed because '$' is not alphabetic.
/// assert!(AlphanumericChar::try_new('$').is_err());
/// ```
pub struct alphanumeric;

impl Validator<char> for alphanumeric {
    fn validate(&self, target: &char) -> bool {
        target.is_alphanumeric()
    }
}

/// [`Validator`] checks if ASCII.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(ascii))]
/// pub struct ASCIIChar(char);
///
/// // Successfully constructed because '$' is ASCII.
/// assert!(ASCIIChar::try_new('$').is_ok());
///
/// // Unsuccessfully constructed because '\u{7070}' is not ASCII.
/// assert!(ASCIIChar::try_new('\u{7070}').is_err());
/// ```
pub struct ascii;

impl Validator<char> for ascii {
    fn validate(&self, target: &char) -> bool {
        target.is_ascii()
    }
}

/// [`Validator`] checks if lowercase.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(lowercase))]
/// pub struct LowercaseChar(char);
///
/// // Successfully constructed because 'c' is lowercase.
/// assert!(LowercaseChar::try_new('c').is_ok());
///
/// // Unsuccessfully constructed because 'c' is not lowercase.
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
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(uppercase))]
/// pub struct UppercaseChar(char);
///
/// // Successfully constructed because 'C' is uppercase.
/// assert!(UppercaseChar::try_new('C').is_ok());
///
/// // Unsuccessfully constructed because 'X' is not uppercase.
/// assert!(UppercaseChar::try_new('x').is_err());
/// ```
pub struct uppercase;

impl Validator<char> for uppercase {
    fn validate(&self, target: &char) -> bool {
        target.is_uppercase()
    }
}
