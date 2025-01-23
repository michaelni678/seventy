//! Float built-ins.

use crate::core::Validator;

/// [`Validator`] checks if finite.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::float::*, seventy, Newtype};
///
/// #[seventy(validate(finite))]
/// struct FiniteF32(f32);
///
/// // Unsuccessfully constructed because 70.70 is finite.
/// assert!(FiniteF32::try_new(70.70).is_ok());
///
/// // Unsuccessfully constructed because the numbers are not finite.
/// assert!(FiniteF32::try_new(f32::INFINITY).is_err());
/// assert!(FiniteF32::try_new(f32::NAN).is_err());
/// ```
pub struct finite;

impl Validator<f32> for finite {
    fn validate(&self, target: &f32) -> bool {
        target.is_finite()
    }
}

impl Validator<f64> for finite {
    fn validate(&self, target: &f64) -> bool {
        target.is_finite()
    }
}
