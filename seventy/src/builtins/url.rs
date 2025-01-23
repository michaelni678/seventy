//! URL built-ins.

use crate::core::Validator;

/// [`Validator`] checks if valid URL.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::url::*, seventy, Newtype};
///
/// #[seventy(validate(url))]
/// pub struct URL(String);
///
/// // Successfully constructed because the URL is valid.
/// assert!(URL::try_new("https://github.com/michaelni678/seventy").is_ok());
///
/// // Unsuccessfully constructed because the URL is invalid.
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
