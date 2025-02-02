//! Operator built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Sanitizer`] assigns the target to the given value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::operator::*, seventy, Newtype};
///
/// #[seventy(sanitize(assign(5)))]
/// pub struct Five(i32);
///
/// // Assigns to 5.
/// assert_eq!(Five::try_new(3).unwrap().into_inner(), 5);
/// ```
pub struct assign<T>(pub T);

impl<T> Sanitizer<T> for assign<T>
where
    T: Clone,
{
    fn sanitize(&self, target: &mut T) {
        *target = self.0.clone();
    }
}

/// [`Validator`] checks if target is not the inner validation.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{char::*, operator::*},
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
///     builtins::{compare::*, operator::*},
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

/// [`Validator`] checks if valid for any inner validator.
///
/// If a validator returns a valid validation, the remaining are skipped.
///
/// Internally expands to a bunch of [`either`] validators.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{compare::*, operator::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(any!(eq(1), eq(2), eq(3))))]
/// pub struct OneOrTwoOrThree(i32);
///
/// // Sucessfully constructed because the numbers are either 1, 2, or 3.
/// assert!(OneOrTwoOrThree::try_new(1).is_ok());
/// assert!(OneOrTwoOrThree::try_new(2).is_ok());
/// assert!(OneOrTwoOrThree::try_new(3).is_ok());
///
/// // Unsuccessfully constructed because 4 is not either 1, 2, or 3.
/// assert!(OneOrTwoOrThree::try_new(4).is_err());
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! _any {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::builtins::operator::either($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::builtins::operator::either(
            $a,
            $crate::builtins::operator::any!($($rest)*)
        )
    };
}

#[doc(inline)]
pub use _any as any;
