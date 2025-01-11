//! URL built-ins.

use crate::core::Validator;

/// [`Validator`] checks if valid URL.
pub struct url;

impl<T> Validator<T> for url
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        url_util::Url::parse(target.as_ref()).is_ok()
    }
}
