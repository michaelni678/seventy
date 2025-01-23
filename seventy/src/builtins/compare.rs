//! Comparison built-ins.

use std::ops::RangeBounds;

use crate::core::Validator;

/// [`Validator`] checks if less than other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(lt(100.0)))]
/// struct PercentGermsKilled(f32);
///
/// // Successfully constructed because 99.9 < 100.0.
/// assert!(PercentGermsKilled::try_new(99.9).is_ok());
///
/// // Unsuccessfully constructed because the numbers are not < 100.0.
/// assert!(PercentGermsKilled::try_new(100.0).is_err());
/// assert!(PercentGermsKilled::try_new(100.1).is_err());
/// ```
pub struct lt<T>(pub T);

impl<T> Validator<T> for lt<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.lt(&self.0)
    }
}

/// [`Validator`] checks if less than or equal to other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// const MAX_PACKAGE_WEIGHT: f32 = 70.0;
///
/// #[seventy(validate(le(MAX_PACKAGE_WEIGHT)))]
/// struct PackageWeight(f32);
///
/// // Successfully constructed because the numbers are <= `MAX_PACKAGE_WEIGHT`.
/// assert!(PackageWeight::try_new(68.3).is_ok());
/// assert!(PackageWeight::try_new(70.0).is_ok());
///
/// // Unsuccessfully constructed because 70.9 is not <= `MAX_PACKAGE_WEIGHT`.
/// assert!(PackageWeight::try_new(70.9).is_err());
/// ```
pub struct le<T>(pub T);

impl<T> Validator<T> for le<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.le(&self.0)
    }
}

/// [`Validator`] checks if greater than other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(gt(0)))]
/// struct PositiveNumber(i32);
///
/// // Successfully constructed because 5 > 0.
/// assert!(PositiveNumber::try_new(5).is_ok());
///
/// // Unsuccessfully constructed because the numbers are not > 0.
/// assert!(PositiveNumber::try_new(0).is_err());
/// assert!(PositiveNumber::try_new(-5).is_err());
/// ```
pub struct gt<T>(pub T);

impl<T> Validator<T> for gt<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.gt(&self.0)
    }
}

/// [`Validator`] checks if greater than or equal to other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(ge(13)))]
/// struct Age(u8);
///
/// // Successfully constructed because the numbers are >= 13.
/// assert!(Age::try_new(70).is_ok());
/// assert!(Age::try_new(13).is_ok());
///
/// // Unsuccessfully constructed because 11 is not >= 13.
/// assert!(Age::try_new(11).is_err());
/// ```
pub struct ge<T>(pub T);

impl<T> Validator<T> for ge<T>
where
    T: PartialOrd,
{
    fn validate(&self, target: &T) -> bool {
        target.ge(&self.0)
    }
}

/// [`Validator`] checks if equal than other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(eq(70)))]
/// struct Seventy(u8);
///
/// // Successfully constructed because 70 == 70.
/// assert!(Seventy::try_new(70).is_ok());
///
/// // Unsuccessfully constructed because 60 is not == 70.
/// assert!(Seventy::try_new(60).is_err());
/// ```
pub struct eq<T>(pub T);

impl<T> Validator<T> for eq<T>
where
    T: PartialEq,
{
    fn validate(&self, target: &T) -> bool {
        target.eq(&self.0)
    }
}

/// [`Validator`] checks if not equal than other value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(ne(0)))]
/// struct NonZeroU32(u32);
///
/// // Successfully constructed because the number is not equal to 0.
/// assert!(NonZeroU32::try_new(70u32).is_ok());
///
/// // Unsuccessfully constructed because the number is equal to 0.
/// assert!(NonZeroU32::try_new(0u32).is_err());
/// ```
pub struct ne<T>(pub T);

impl<T> Validator<T> for ne<T>
where
    T: PartialEq,
{
    fn validate(&self, target: &T) -> bool {
        target.ne(&self.0)
    }
}

/// [`Validator`] checks if within range.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::compare::*, seventy, Newtype};
///
/// #[seventy(validate(within(5000..=100000)))]
/// struct LoanAmount(u32);
///
/// // Unsuccessfully constructed because 70 is not between 5K and 100k.
/// assert!(LoanAmount::try_new(70u32).is_err());
///
/// // Successfully constructed because the numbers are between 5k and 100k.
/// assert!(LoanAmount::try_new(5000u32).is_ok());
/// assert!(LoanAmount::try_new(70000u32).is_ok());
/// assert!(LoanAmount::try_new(100000u32).is_ok());
///
/// // Unsuccessfully constructed because 100070 is not between 5K and 100k.
/// assert!(LoanAmount::try_new(100070u32).is_err());
/// ```
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
