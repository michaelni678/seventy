//! Collection built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if among the collection.
/// 
/// # Examples
///
/// The example below validates the sunscreen's SPF is among the array of
/// common SPFs. Because of the newtype's guarantees, it is impossible to construct 
/// `SunscreenSPF` with an SPF that is not listed.
/// 
/// ```
/// use seventy::{builtins::collection::*, core::Newtype, seventy};
///
/// #[seventy(validate(among([8, 15, 30, 40, 50, 100])))]
/// struct SunscreenSPF(u8);
///
/// assert!(SunscreenSPF::try_new(50).is_ok());
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
/// use seventy::{builtins::collection::*, core::Newtype, seventy};
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
