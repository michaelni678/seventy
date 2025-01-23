use crate::core::Validator;

/// [`Validator`] checks if alphabetic.
///
/// # Examples
///
/// The example below validates the inner char is alphabetic. Because of the
/// newtype's guarantees, it is impossible to construct an `AlphabeticChar` with
/// a non-alphabetic character.
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(alphabetic))]
/// pub struct AlphabeticChar(char);
///
/// assert!(AlphabeticChar::try_new('c').is_ok());
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
/// The example below validates the inner char is alphanumeric. Because of the
/// newtype's guarantees, it is impossible to construct an `AlphanumericChar`
/// with a non-alphanumeric character.
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(alphanumeric))]
/// pub struct AlphanumericChar(char);
///
/// assert!(AlphanumericChar::try_new('0').is_ok());
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
/// The example below validates the inner char is ASCII. Because of the
/// newtype's guarantees, it is impossible to construct an `ASCIIChar` with a
/// non-ASCII character.
/// ```
/// use seventy::{builtins::char::*, seventy, Newtype};
///
/// #[seventy(validate(ascii))]
/// pub struct ASCIIChar(char);
///
/// assert!(ASCIIChar::try_new('$').is_ok());
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
