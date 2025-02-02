//! Debugging built-ins.

use crate::core::Validator;

/// [`Validator`] that always validates as invalid.
pub struct invalid;

impl<T> Validator<T> for invalid {
    fn validate(&self, _target: &T) -> bool {
        false
    }
}