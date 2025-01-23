//! Collection built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if among the collection.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::collection::*, seventy, Newtype};
///
/// #[seventy(validate(among([8, 15, 30, 40, 50, 100])))]
/// struct SunscreenSPF(u8);
///
/// // Successfully constructed because 50 is among the given array.
/// assert!(SunscreenSPF::try_new(50).is_ok());
///
/// // Unsuccessfully constructed because 55 is not among the given array.
/// assert!(SunscreenSPF::try_new(55).is_err());
/// ```
pub struct among<C>(pub C);

impl<T, const N: usize> Validator<T> for among<[T; N]>
where
    T: PartialEq,
{
    fn validate(&self, target: &T) -> bool {
        self.0.contains(target)
    }
}

/// [`Sanitizer`] sorts elements.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::collection::*, seventy, Newtype};
///
/// #[seventy(sanitize(sort))]
/// struct SortedChars(Vec<char>);
///
/// // Sorts the array.
/// assert_eq!(
///     SortedChars::try_new(['b', 'c', 'a']).unwrap().into_inner(),
///     ['a', 'b', 'c']
/// );
/// ```
pub struct sort;

impl<T, const N: usize> Sanitizer<[T; N]> for sort
where
    T: Ord,
{
    fn sanitize(&self, target: &mut [T; N]) {
        target.sort();
    }
}

impl<T> Sanitizer<Vec<T>> for sort
where
    T: Ord,
{
    fn sanitize(&self, target: &mut Vec<T>) {
        target.sort();
    }
}

/// [`Validator`] forwards length to inner validator.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{collection::*, compare::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(length(le(5))))]
/// pub struct CharVec5(Vec<char>);
///
/// // Successfully constructed because collection length is 5.
/// assert!(CharVec5::try_new(['a', 'b', 'c', 'd', 'e']).is_ok());
///
/// // Unsuccessfully constructed because collection length isn't 5.
/// assert!(CharVec5::try_new(['a', 'b', 'c', 'd', 'e', 'f']).is_err());
/// ```
pub struct length<V>(pub V);

impl<T, const N: usize, V> Validator<[T; N]> for length<V>
where
    V: Validator<usize>,
{
    fn validate(&self, _target: &[T; N]) -> bool {
        self.0.validate(&N)
    }
}

impl<T, V> Validator<Vec<T>> for length<V>
where
    V: Validator<usize>,
{
    fn validate(&self, target: &Vec<T>) -> bool {
        self.0.validate(&target.len())
    }
}
