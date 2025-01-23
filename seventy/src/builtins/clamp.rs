//! Clamp built-ins.

use crate::core::Sanitizer;

/// [`Sanitizer`] restricts to between `min` and `max`.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp { min: -1.0, max: 1.0 }))]
/// struct Movement(f32);
///
/// // Clamps -2.0 to -1.0.
/// assert_eq!(Movement::try_new(-2.0).unwrap().into_inner(), -1.0);
///
/// // No changes, because within the clamp restriction.
/// assert_eq!(Movement::try_new(-1.0).unwrap().into_inner(), -1.0);
/// assert_eq!(Movement::try_new(0.0).unwrap().into_inner(), 0.0);
/// assert_eq!(Movement::try_new(1.0).unwrap().into_inner(), 1.0);
///
/// // Clamps 2.0 to 1.0.
/// assert_eq!(Movement::try_new(2.0).unwrap().into_inner(), 1.0);
/// ```
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

/// [`Sanitizer`] restricts to greater than the min.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp_min(0.0)))]
/// struct Distance(f32);
///
/// // Clamps -1.0 to 0.0.
/// assert_eq!(Distance::try_new(-1.0).unwrap().into_inner(), 0.0);
///
/// // No changes, because within the clamp restriction.
/// assert_eq!(Distance::try_new(0.0).unwrap().into_inner(), 0.0);
/// assert_eq!(Distance::try_new(1.0).unwrap().into_inner(), 1.0);
/// ```
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

/// [`Sanitizer`] restricts to less than the max.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp_max(100)))]
/// struct BatteryCharge(u8);
///
/// // Clamps 101 to 100.
/// assert_eq!(BatteryCharge::try_new(101).unwrap().into_inner(), 100);
///
/// // No changes, because within the clamp restriction.
/// assert_eq!(BatteryCharge::try_new(100).unwrap().into_inner(), 100);
/// assert_eq!(BatteryCharge::try_new(70).unwrap().into_inner(), 70);
/// ```
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
