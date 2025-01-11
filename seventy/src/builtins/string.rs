//! String built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Sanitizer`] trims whitespace.
pub struct trim;

impl Sanitizer<String> for trim {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim().to_string();
    }
}

/// [`Sanitizer`] trims left whitespace.
pub struct trim_left;

impl Sanitizer<String> for trim_left {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim_start().to_string();
    }
}

/// [`Sanitizer`] trims right whitespace.
pub struct trim_right;

impl Sanitizer<String> for trim_right {
    fn sanitize(&self, target: &mut String) {
        // OPTIMIZE: Try trimming in-place.
        *target = target.trim_end().to_string();
    }
}

/// [`Validator`] checks if only alphabetic.
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
pub struct alphanumeric;

impl<T> Validator<T> for alphanumeric
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        target.as_ref().chars().all(char::is_alphabetic)
    }
}

/// [`Validator`] checks if only ASCII.
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
