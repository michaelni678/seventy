//! Collection built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if among the collection.
///
/// # Examples
///
/// The example below validates the sunscreen's SPF is among the array of
/// common SPFs. Because of the newtype's guarantees, it is impossible to
/// construct `SunscreenSPF` with an SPF that is not listed.
///
/// ```
/// use seventy::{builtins::collection::*, seventy, Newtype};
///
/// #[seventy(validate(among([8, 15, 30, 40, 50, 100])))]
/// struct SunscreenSPF(u8);
///
/// assert!(SunscreenSPF::try_new(50).is_ok());
///
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
/// The example below sanitizes a vec of chars to be sorted.
/// Because of the newtype's guarantees, the constructed `SortedChars` will
/// always have an inner vec that has been sorted.
///
/// ```
/// use seventy::{builtins::collection::*, seventy, Newtype};
///
/// #[seventy(sanitize(sort))]
/// struct SortedChars(Vec<char>);
///
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
/// The example below validates the length is less than or equal to 5
/// characters. Because of the newtype's guarantees, it is impossible to
/// construct `CharVec5` with an inner [`Vec`] containing more
/// than 5 characters.
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
/// assert!(CharVec5::try_new(['a', 'b', 'c', 'd', 'e']).is_ok());
///
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
