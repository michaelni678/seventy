//! Option built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if [`Some`].
pub struct some;

impl<T> Validator<Option<T>> for some {
    fn validate(&self, target: &Option<T>) -> bool {
        target.is_some()
    }
}

/// [`Sanitizer`] and [`Validator`] forwards unwrapped target if [`Some`].
/// 
/// For [`Sanitizer`], if [`None`] the inner sanitizer is skipped.
/// 
/// For [`Validator`], if [`None`] the inner validator is skipped and the validation is valid.
/// See [`unwrap_then`] if the target must be [`Some`].
pub struct some_then<SV>(pub SV);

impl<T, S> Sanitizer<Option<T>> for some_then<S>
where
    S: Sanitizer<T>,
{
    fn sanitize(&self, target: &mut Option<T>) {
        if let Some(inner) = target {
            self.0.sanitize(inner);
        }
    }
}

impl<T, V> Validator<Option<T>> for some_then<V>
where
    V: Validator<T>,
{
    fn validate(&self, target: &Option<T>) -> bool {
        if let Some(inner) = target {
            self.0.validate(inner)
        } else {
            true
        }
    }
}

/// [`Validator`] forwards unwrapped target if [`Some`].
/// 
/// If [`None`] the inner validator is skipped and the validation is invalid.
/// See [`some_then`] if the target can be [`None`].
pub struct unwrap_then<V>(pub V);

impl<T, V> Validator<Option<T>> for unwrap_then<V>
where
    V: Validator<T>,
{
    fn validate(&self, target: &Option<T>) -> bool {
        if let Some(inner) = target {
            self.0.validate(inner)
        } else {
            false
        }
    }
}
