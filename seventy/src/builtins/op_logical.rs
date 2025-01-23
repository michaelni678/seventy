use crate::core::Validator;

/// [`Validator`] checks if target is not the inner validation.
///
/// # Examples
///
/// The example below validates the inner char is not alphabetic. Because of the
/// newtype's guarantees, it is impossible to construct `NotAlphabeticChar` with
/// an inner char that is alphabetic.
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
/// assert!(NotAlphabeticChar::try_new('0').is_ok());
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
