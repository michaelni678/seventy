//! Email built-ins.

use std::sync::LazyLock;

use crate::core::Validator;

use regex_util::Regex;

/// [`Validator`] checks if valid email.
/// 
/// Uses the regex found in this post:
/// <https://stackoverflow.com/a/201378>
/// 
/// # Examples
///
/// The example below validates the inner string is a valid email. 
/// Because of the newtype's guarantees, it is impossible to construct
/// an `EmailAddress` with an inner string that is not a valid email.
/// 
/// ```
/// use seventy::{builtins::email::*, core::Newtype, seventy};
///
/// #[seventy(validate(email))]
/// struct EmailAddress(String);
///
/// assert!(EmailAddress::try_new("seventy70@example.com").is_ok());
/// assert!(EmailAddress::try_new("seventy70@@example.com").is_err());
/// ```
pub struct email;

impl<T> Validator<T> for email
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        static EMAIL_ADDRESS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap()
        });

        EMAIL_ADDRESS_REGEX.is_match(target.as_ref())
    }
}
