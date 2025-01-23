//! Option built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if [`Some`].
///
/// # Examples
///
/// ```
/// use seventy::{builtins::option::*, seventy, Newtype};
///
/// #[seventy(validate(some))]
/// pub struct RequiredField(Option<String>);
///
/// // Successfully constructed because `Some`.
/// assert!(RequiredField::try_new(Some(String::from("Seventy is a cool crate."))).is_ok());
///
/// // Unsuccessfully constructed because `None`.
/// assert!(RequiredField::try_new(None).is_err());
/// ```
pub struct some;

impl<T> Validator<Option<T>> for some {
    fn validate(&self, target: &Option<T>) -> bool {
        target.is_some()
    }
}

/// [`Sanitizer`] and [`Validator`] forwards unwrapped target if [`Some`].
///
/// For [`Sanitizer`], if [`None`] the inner sanitizer is skipped.
///
/// For [`Validator`], if [`None`] the inner validator is skipped and the
/// validation is valid. See [`unwrap_then`] if the target must be [`Some`].
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{option::*, string::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(sanitize(some_then(trim)), validate(some_then(alphabetic)))]
/// pub struct MiddleName(Option<String>);
///
/// // Trims the string because `Some`.
/// // Successfully constructed because "John" (the trimmed string) is alphabetic.
/// assert_eq!(
///     MiddleName::try_new(Some(String::from("   John   ")))
///         .unwrap()
///         .into_inner()
///         .unwrap(),
///     "John"
/// );
///
/// // Successfully constructed because `None`.
/// assert!(MiddleName::try_new(None).is_ok());
///
/// // Trims the string because `Some`.
/// // Unsuccessfully constructed because "J0hn" (the trimmed string) is not alphabetic.
/// assert!(MiddleName::try_new(Some(String::from("   J0hn   "))).is_err());
/// ```
pub struct some_then<SV>(pub SV);

impl<T, S> Sanitizer<Option<T>> for some_then<S>
where
    S: Sanitizer<T>,
{
    fn sanitize(&self, target: &mut Option<T>) {
        if let Some(inner) = target {
            self.0.sanitize(inner);
        }
    }
}

impl<T, V> Validator<Option<T>> for some_then<V>
where
    V: Validator<T>,
{
    fn validate(&self, target: &Option<T>) -> bool {
        if let Some(inner) = target {
            self.0.validate(inner)
        } else {
            true
        }
    }
}

/// [`Validator`] forwards unwrapped target if [`Some`].
///
/// If [`None`] the inner validator is skipped and the validation is invalid.
/// See [`some_then`] if the target can be [`None`].
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{compare::*, option::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(unwrap_then(within(1..=10))))]
/// pub struct RequiredFeedback(Option<u8>);
///
/// // Successfuly constructed because 7 is between 1 and 10.
/// assert!(RequiredFeedback::try_new(Some(7)).is_ok());
///
/// // Unsuccessfully constructed because 11 is not between 1 and 10.
/// assert!(RequiredFeedback::try_new(Some(11)).is_err());
///
/// // Unsuccessfully constructed because `None`.
/// assert!(RequiredFeedback::try_new(None).is_err());
/// ```
pub struct unwrap_then<V>(pub V);

impl<T, V> Validator<Option<T>> for unwrap_then<V>
where
    V: Validator<T>,
{
    fn validate(&self, target: &Option<T>) -> bool {
        if let Some(inner) = target {
            self.0.validate(inner)
        } else {
            false
        }
    }
}
