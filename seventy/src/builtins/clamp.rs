//! Clamp built-ins.

use crate::core::Sanitizer;

pub struct clamp<T> {
    pub min: T,
    pub max: T,
}

impl<T> Sanitizer<T> for clamp<T>
where
    T: PartialOrd + Copy,
{
    fn sanitize(&self, target: &mut T) {
        if *target < self.min {
            *target = self.min;
        }

        if *target > self.max {
            *target = self.max;
        }
    }
}

pub struct clamp_min<T>(pub T);

impl<T> Sanitizer<T> for clamp_min<T>
where
    T: PartialOrd + Copy,
{
    fn sanitize(&self, target: &mut T) {
        let min = self.0;

        if *target < min {
            *target = min;
        }
    }
}

pub struct clamp_max<T>(pub T);

impl<T> Sanitizer<T> for clamp_max<T>
where
    T: PartialOrd + Copy,
{
    fn sanitize(&self, target: &mut T) {
        let max = self.0;

        if *target > max {
            *target = max;
        }
    }
}
