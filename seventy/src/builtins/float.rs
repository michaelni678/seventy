//! Float built-ins.

use crate::core::Validator;

/// [`Validator`] checks if finite.
/// 
/// # Examples
///
/// The example below validates the inner f32 is alphanumeric. Because of the
/// newtype's guarantees, it is impossible to construct a `FiniteF32` with
/// an inner f32 that is not finite.
/// 
/// ```
/// use seventy::{builtins::float::*, core::Newtype, seventy};
///
/// #[seventy(validate(finite))]
/// struct FiniteF32(f32);
///
/// assert!(FiniteF32::try_new(70.70).is_ok());
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
