//! Comparison built-ins.

use std::ops::RangeBounds;

use crate::core::Validator;

/// [`Validator`] checks if less than other value.
///
/// # Examples
///
/// The example below validates the percentage of germs killed for a cleaning
/// product is less than 100.0 units. Because of the newtype's guarantees,
/// it is impossible to construct `PercentGermsKilled` with an inner f32 that is
/// 100.0 or greater.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(lt(100.0)))]
/// struct PercentGermsKilled(f32);
///
/// assert!(PercentGermsKilled::try_new(99.9).is_ok());
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
/// The example below validates the package weight is less than or equal
/// to the limit of 70.0. Because of the newtype's guarantees, it is impossible
/// to construct `PackageWeight` with an inner f32 that is greater than 70.0.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// const MAX_PACKAGE_WEIGHT: f32 = 70.0;
///
/// #[seventy(validate(le(MAX_PACKAGE_WEIGHT)))]
/// struct PackageWeight(f32);
///
/// assert!(PackageWeight::try_new(68.3).is_ok());
/// assert!(PackageWeight::try_new(70.0).is_ok());
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
/// The example below validates the inner i32 is positive. Because of the
/// newtype's guarantees, it is impossible to construct `PositiveNumber` with
/// a negative or zero i32.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(gt(0)))]
/// struct PositiveNumber(i32);
///
/// assert!(PositiveNumber::try_new(70).is_ok());
/// assert!(PositiveNumber::try_new(0).is_err());
/// assert!(PositiveNumber::try_new(-70).is_err());
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
/// The example below validates the age is over 13. Because of the newtype's
/// guarantees, it is impossible to construct `Age` with an inner u8 less than 13.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(ge(13)))]
/// struct Age(u8);
///
/// assert!(Age::try_new(70).is_ok());
/// assert!(Age::try_new(13).is_ok());
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
/// The example below validates the inner u8 is 70. Because of the newtype's
/// guarantees, it is impossible to construct `Seventy` with a non-70 u8.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(eq(70)))]
/// struct Seventy(u8);
///
/// assert!(Seventy::try_new(70).is_ok());
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
/// The example below validates the inner u32 is not 0. Because of the newtype's
/// guarantees, it is impossible to construct `NonZeroU32` with a zero.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(ne(0)))]
/// struct NonZeroU32(u32);
///
/// assert!(NonZeroU32::try_new(70u32).is_ok());
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
/// The example below validates the loan request amount is between 5000 and 100000.
/// Because of the newtype's guarantees, it is impossible to construct `LoanAmount`
/// with a u32 less than 5000 or greater than 100000.
///
/// ```
/// use seventy::{builtins::compare::*, core::Newtype, seventy};
///
/// #[seventy(validate(within(5000..=100000)))]
/// struct LoanAmount(u32);
///
/// assert!(LoanAmount::try_new(70u32).is_err());
/// assert!(LoanAmount::try_new(5000u32).is_ok());
/// assert!(LoanAmount::try_new(70000u32).is_ok());
/// assert!(LoanAmount::try_new(100000u32).is_ok());
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
