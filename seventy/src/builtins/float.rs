//! Float built-ins.

use crate::core::Validator;

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
