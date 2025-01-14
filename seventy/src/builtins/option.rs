//! Option built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if [`Some`].
///
/// # Examples
///
/// The example below validates the field is filled.
/// Because of the newtype's guarantees, it is impossible to construct
/// `RequiredField` with an inner option that is [`None`].
///
/// ```
/// use seventy::{builtins::option::*, seventy, Newtype};
///
/// #[seventy(validate(some))]
/// pub struct RequiredField(Option<String>);
///
/// assert!(RequiredField::try_new(Some(String::from("Seventy is a cool crate."))).is_ok());
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
/// The example below sanitizes a middle name and validates it is alphabetic if
/// it is given. Because of the newtype's guarantees, the constructed
/// `MiddleName` will always be trimmed, and cannot exist if the inner string is
/// not alphabetic.
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
/// assert_eq!(
///     MiddleName::try_new(Some(String::from("   John   ")))
///         .unwrap()
///         .into_inner()
///         .unwrap(),
///     "John"
/// );
///
/// assert!(MiddleName::try_new(None).is_ok());
///
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
/// The example below validates a rating is given and between 1 and 10.
/// Because of the newtype's guarantees, it is impossible to construct
/// `RequiredFeedback` with an inner option that is [`None`] or not between 0
/// and 10.
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
/// assert!(RequiredFeedback::try_new(Some(7)).is_ok());
/// assert!(RequiredFeedback::try_new(Some(11)).is_err());
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
