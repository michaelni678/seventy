//! String built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Sanitizer`] trims whitespace.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(sanitize(trim))]
/// pub struct Username(String);
///
/// // Trims surrounding whitespace.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(sanitize(trim_left))]
/// pub struct Username(String);
///
/// // Trims left whitespace, leaving the right.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(sanitize(trim_right))]
/// pub struct Username(String);
///
/// // Trims right whitespace, leaving the left.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(alphabetic))]
/// pub struct FirstName(String);
///
/// // Successfully constructed because the string is only alphabetic.
/// assert!(FirstName::try_new("Michael").is_ok());
///
/// // Unsuccessfully constructed because the string is not only alphabetic.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(alphanumeric))]
/// pub struct Username(String);
///
/// // Successfully constructed because the string is only alphanumeric.
/// assert!(Username::try_new("Seventy70").is_ok());
///
/// // Unsuccessfully constructed because the string is not only alphanumeric.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(ascii))]
/// pub struct Password(String);
///
/// // Successfully constructed because the string is only ASCII.
/// assert!(Password::try_new("Seventy#70!").is_ok());
///
/// // Unsuccessfully constructed because the string is not only ASCII.
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
/// An example of the `lowercase` validator.
///
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(sanitize(lowercase))]
/// pub struct SearchQuery(String);
///
/// // Converts uppercase characters to lowercase.
/// assert_eq!(
///     SearchQuery::try_new("What is the SeVeNTy crate?!")
///         .unwrap()
///         .into_inner(),
///     "what is the seventy crate?!"
/// );
/// ```
///
/// An example of the `lowercase` sanitizer.
///
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(lowercase))]
/// pub struct LowercaseString(String);
///
/// // Successfully constructed because the string is only lowercase.
/// assert!(LowercaseString::try_new("whisper").is_ok());
///
/// // Unsuccessfully constructed because the string is not only lowercase.
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
/// An example of the `uppercase` sanitizer.
///
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(sanitize(uppercase))]
/// pub struct ID(String);
///
/// // Converts lowercase characters to uppercase.
/// assert_eq!(
///     ID::try_new("70sV-Nty70").unwrap().into_inner(),
///     "70SV-NTY70"
/// );
/// ```
///
/// An example of the `uppercase` validator.
///
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(uppercase))]
/// pub struct UppercaseString(String);
///
/// // Successfully constructed because the string is only uppercase.
/// assert!(UppercaseString::try_new("SHOUT").is_ok());
///
/// // Unsuccessfully constructed because the string is not only uppercase.
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
/// ```
/// use seventy::{
///     builtins::{compare::*, string::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(length::chars(le(15))))]
/// pub struct TextBox(String);
///
/// // Successfully constructed because the string is <= 15 characters.
/// assert!(TextBox::try_new("Hello, World!").is_ok());
///
/// // Unsuccessfully constructed because the string is not <= 15 characters.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(not_empty))]
/// pub struct TextBox(String);
///
/// // Successfully constructed because the string is not empty.
/// assert!(TextBox::try_new("Hello, World!").is_ok());
///
/// // Unsuccessfully constructed because the string is empty.
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
/// ```
/// use seventy::{builtins::string::*, seventy, Newtype};
///
/// #[seventy(validate(regex(r"^[A-Z]([a-zA-Z0-9]|[- @\.#&!])*$")))]
/// pub struct Company(String);
///
/// // Successfully constructed because "Seven Tea Inc." matches the regex.
/// assert!(Company::try_new("Seven Tea Inc.").is_ok());
///
/// // Unsuccessfuly constructed because "Seven \u{1F375} Inc." does not match the regex.
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
