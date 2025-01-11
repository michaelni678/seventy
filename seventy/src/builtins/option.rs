//! Option built-ins.

use crate::core::{Sanitizer, Validator};

pub struct some;

impl<T> Validator<Option<T>> for some {
    fn validate(&self, target: &Option<T>) -> bool {
        target.is_some()
    }
}

pub struct some_then<SV>(pub SV);

impl<T, SV> Sanitizer<Option<T>> for some_then<SV>
where
    SV: Sanitizer<T>,
{
    fn sanitize(&self, target: &mut Option<T>) {
        if let Some(inner) = target {
            self.0.sanitize(inner);
        }
    }
}

impl<T, SV> Validator<Option<T>> for some_then<SV>
where
    SV: Validator<T>,
{
    fn validate(&self, target: &Option<T>) -> bool {
        if let Some(inner) = target {
            self.0.validate(inner)
        } else {
            true
        }
    }
}

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
