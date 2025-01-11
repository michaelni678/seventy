//! Collection built-ins.

use crate::core::{Sanitizer, Validator};

pub struct among<C>(pub C);

impl<T, const N: usize> Validator<T> for among<[T; N]>
where
    T: PartialEq,
{
    fn validate(&self, target: &T) -> bool {
        self.0.contains(target)
    }
}

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
