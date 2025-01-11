//! String built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Sanitizer`] trims whitespace.
///
/// # Examples
///
/// The example below sanitizes a username to remove surrounding whitespace.
/// Because of the newtype's guarantees, the constructed `Username` will
/// always have an inner string that has been trimmed.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(sanitize(trim))]
/// pub struct Username(String);
///
/// assert_eq!(
///     Username::try_new("   username   ").unwrap().into_inner(),
///     "username"
/// );
/// ```
pub struct trim;

impl Sanitizer<String> for trim {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim().to_string();
    }
}

/// [`Sanitizer`] trims left whitespace.
///
/// # Examples
///
/// The example below sanitizes a username to remove left whitespace.
/// Because of the newtype's guarantees, the constructed `Username` will
/// always have an inner string that has been left-trimmed.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(sanitize(trim_left))]
/// pub struct Username(String);
///
/// assert_eq!(
///     Username::try_new("   username   ").unwrap().into_inner(),
///     "username   "
/// );
/// ```
pub struct trim_left;

impl Sanitizer<String> for trim_left {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim_start().to_string();
    }
}

/// [`Sanitizer`] trims right whitespace.
///
/// # Examples
///
/// The example below sanitizes a username to remove right whitespace.
/// Because of the newtype's guarantees, the constructed `Username` will
/// always have an inner string that has been right-trimmed.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(sanitize(trim_right))]
/// pub struct Username(String);
///
/// assert_eq!(
///     Username::try_new("   username   ").unwrap().into_inner(),
///     "   username"
/// );
/// ```
pub struct trim_right;

impl Sanitizer<String> for trim_right {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim_end().to_string();
    }
}

/// [`Validator`] checks if only alphabetic.
///
/// # Examples
///
/// The example below validates the inner string is alpabetic. Because of the
/// newtype's guarantees, it is impossible to construct `FirstName` with
/// an inner string that is not alphabetic.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(alphabetic))]
/// pub struct FirstName(String);
///
/// assert!(FirstName::try_new("Michael").is_ok());
/// assert!(FirstName::try_new("Mich4el").is_err());
/// ```
pub struct alphabetic;

impl<T> Validator<T> for alphabetic
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().chars().all(char::is_alphabetic)
    }
}

/// [`Validator`] checks if only alphanumeric.
///
/// # Examples
///
/// The example below validates the inner string is alphanumeric. Because of the
/// newtype's guarantees, it is impossible to construct `Username` with
/// an inner string that is not alphanumeric.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(alphanumeric))]
/// pub struct Username(String);
///
/// assert!(Username::try_new("Seventy70").is_ok());
/// assert!(Username::try_new("Seventy#70!").is_err());
/// ```
pub struct alphanumeric;

impl<T> Validator<T> for alphanumeric
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().chars().all(char::is_alphanumeric)
    }
}

/// [`Validator`] checks if only ASCII.
///
/// # Examples
///
/// The example below validates the inner string is ASCII. Because of the
/// newtype's guarantees, it is impossible to construct `Password` with
/// an inner string that is not ASCII.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(ascii))]
/// pub struct Password(String);
///
/// assert!(Password::try_new("Seventy#70!").is_ok());
/// assert!(Password::try_new("Seventy\u{7070}#70!").is_err());
/// ```
pub struct ascii;

impl<T> Validator<T> for ascii
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().is_ascii()
    }
}

/// [`Sanitizer`] converts to lowercase. [`Validator`] checks if only lowercase.
///
/// # Examples
///
/// The example below sanitizes a search query to lowercase. Because of the
/// newtype's guarantees, the constructed `SearchQuery` will always have an inner
/// string that is lowercase.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(sanitize(lowercase))]
/// pub struct SearchQuery(String);
///
/// assert_eq!(
///     SearchQuery::try_new("What is the SeVeNTy crate?!")
///         .unwrap()
///         .into_inner(),
///     "what is the seventy crate?!"
/// );
/// ```
///
/// The example below validates the inner string is lowercase. Because of the
/// newtype's guarantees, it is impossible to construct `LowercaseString` with
/// an inner string that is not lowercase.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(lowercase))]
/// pub struct LowercaseString(String);
///
/// assert!(LowercaseString::try_new("whisper").is_ok());
/// assert!(LowercaseString::try_new("Whisper").is_err());
/// ```
pub struct lowercase;

impl Sanitizer<String> for lowercase {
    fn sanitize(&self, target: &mut String) {
        *target = target.to_lowercase();
    }
}

impl<T> Validator<T> for lowercase
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().chars().all(char::is_lowercase)
    }
}

/// [`Sanitizer`] converts to uppercase. [`Validator`] checks if only uppercase.
///
/// # Examples
///
/// The example below sanitizes an ID to uppercase. Because of the
/// newtype's guarantees, the constructed `ID` will always have an inner
/// string that is uppercase.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(sanitize(uppercase))]
/// pub struct ID(String);
///
/// assert_eq!(
///     ID::try_new("70sV-Nty70").unwrap().into_inner(),
///     "70SV-NTY70"
/// );
/// ```
///
/// The example below validates the inner string is uppercase. Because of the
/// newtype's guarantees, it is impossible to construct `UppercaseString` with
/// an inner string that is not uppercase.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(uppercase))]
/// pub struct UppercaseString(String);
///
/// assert!(UppercaseString::try_new("SHOUT").is_ok());
/// assert!(UppercaseString::try_new("Shout").is_err());
/// ```
pub struct uppercase;

impl Sanitizer<String> for uppercase {
    fn sanitize(&self, target: &mut String) {
        *target = target.to_uppercase();
    }
}

impl<T> Validator<T> for uppercase
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().chars().all(char::is_uppercase)
    }
}

/// [`Validator`] forwards length to inner validator.
///
/// # Examples
///
/// The example below validates the text is less than or equal to 15 characters.
/// Because of the newtype's guarantees, it is impossible to construct
/// `TextBox` with an inner string that greater than 15 characters.
///
/// ```
/// use seventy::{
///     builtins::{compare::*, string::*},
///     core::Newtype,
///     seventy,
/// };
///
/// #[seventy(validate(length::chars(le(15))))]
/// pub struct TextBox(String);
///
/// assert!(TextBox::try_new("Hello, World!").is_ok());
/// assert!(TextBox::try_new("Hello, World! I am hungry.").is_err());
/// ```
pub enum length<V> {
    bytes(V),
    chars(V),
}

impl<T, V> Validator<T> for length<V>
where
    T: AsRef<str>,
    V: Validator<usize>,
{
    fn validate(&self, target: &T) -> bool {
        let target = target.as_ref();

        match self {
            Self::bytes(v) => v.validate(&target.len()),
            Self::chars(v) => v.validate(&target.chars().count()),
        }
    }
}

/// [`Validator`] checks if not empty.
///
/// # Examples
///
/// The example below validates the text is not empty.
/// Because of the newtype's guarantees, it is impossible to construct
/// `TextBox` with an inner string that is empty.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(not_empty))]
/// pub struct TextBox(String);
///
/// assert!(TextBox::try_new("Hello, World!").is_ok());
/// assert!(TextBox::try_new("").is_err());
/// ```
pub struct not_empty;

impl<T> Validator<T> for not_empty
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        !target.as_ref().is_empty()
    }
}

/// [`Validator`] checks if matches regex.
///
/// # Examples
///
/// The example below validates the company matches the regex.
/// Because of the newtype's guarantees, it is impossible to construct
/// `Company` with an inner string that does not match the regex.
///
/// ```
/// use seventy::{builtins::string::*, core::Newtype, seventy};
///
/// #[seventy(validate(regex(r"^[A-Z]([a-zA-Z0-9]|[- @\.#&!])*$")))]
/// pub struct Company(String);
///
/// assert!(Company::try_new("Seven Tea Inc.").is_ok());
/// assert!(Company::try_new("Seven \u{1F375} Inc.").is_err());
/// ```
#[cfg(feature = "regex")]
pub fn regex(regex: &'static str) -> _regex {
    _regex(regex_util::Regex::new(regex).unwrap())
}

#[doc(hidden)]
#[cfg(feature = "regex")]
pub struct _regex(regex_util::Regex);

#[cfg(feature = "regex")]
impl<T> Validator<T> for _regex
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        self.0.is_match(target.as_ref())
    }
}
