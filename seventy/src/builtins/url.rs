//! URL built-ins.

use crate::core::Validator;

/// [`Validator`] checks if valid URL.
///
/// # Examples
///
/// The example below validates the URL is valid.
/// Because of the newtype's guarantees, it is impossible to construct a
/// `URL` with an inner string that is not a valid URL.
///
/// ```
/// use seventy::{builtins::url::*, seventy, Newtype};
///
/// #[seventy(validate(url))]
/// pub struct URL(String);
///
/// assert!(URL::try_new("https://github.com/michaelni678/seventy").is_ok());
///
/// assert!(URL::try_new("http://[:::1]").is_err());
/// ```
pub struct url;

impl<T> Validator<T> for url
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        url_util::Url::parse(target.as_ref()).is_ok()
    }
}
