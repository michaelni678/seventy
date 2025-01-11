//! Comparison built-ins.

use std::ops::RangeBounds;

use crate::core::Validator;

pub struct lt<T>(pub T);

impl<T> Validator<T> for lt<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.lt(&self.0)
    }
}

pub struct le<T>(pub T);

impl<T> Validator<T> for le<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.le(&self.0)
    }
}

pub struct gt<T>(pub T);

impl<T> Validator<T> for gt<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.gt(&self.0)
    }
}

pub struct ge<T>(pub T);

impl<T> Validator<T> for ge<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.ge(&self.0)
    }
}

pub struct eq<T>(pub T);

impl<T> Validator<T> for eq<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.eq(&self.0)
    }
}

pub struct ne<T>(pub T);

impl<T> Validator<T> for ne<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.ne(&self.0)
    }
}

pub struct within<R>(pub R);

impl<T, R> Validator<T> for within<R>
where
    T: PartialOrd,
    R: RangeBounds<T>,
{
    fn validate(&self, target: &T) -> bool {
        self.0.contains(target)
    }
}
