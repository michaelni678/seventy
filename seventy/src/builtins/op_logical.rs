//! Logical operator built-ins.

use crate::core::Validator;

/// [`Validator`] checks if target is not the inner validation.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{char::*, op_logical::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(not(alphabetic)))]
/// pub struct NotAlphabeticChar(char);
///
/// // Successfully constructed because '0' is not alphabetic.
/// assert!(NotAlphabeticChar::try_new('0').is_ok());
///
/// // Unsuccessfully constructed because 'x' is alphabetic.
/// assert!(NotAlphabeticChar::try_new('x').is_err());
/// ```
pub struct not<V>(pub V);

impl<T, V> Validator<T> for not<V>
where
    V: Validator<T>,
{
    fn validate(&self, target: &T) -> bool {
        !self.0.validate(target)
    }
}

/// [`Validator`] checks if valid for either inner validator.
///
/// If the first validator returns a valid validation, the second is skipped.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{compare::*, op_logical::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(either(eq(1), eq(2))))]
/// pub struct OneOrTwo(i32);
///
/// // Successfully constructed because the numbers are either 1 or 2.
/// assert!(OneOrTwo::try_new(1).is_ok());
/// assert!(OneOrTwo::try_new(2).is_ok());
///
/// // Unsuccessfully constructed because 3 is not either 1 or 2.
/// assert!(OneOrTwo::try_new(3).is_err());
/// ```
pub struct either<V1, V2>(pub V1, pub V2);

impl<T, V1, V2> Validator<T> for either<V1, V2>
where
    V1: Validator<T>,
    V2: Validator<T>,
{
    fn validate(&self, target: &T) -> bool {
        self.0.validate(target) || self.1.validate(target)
    }
}
