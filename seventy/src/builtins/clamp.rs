//! Clamp built-ins.

use crate::core::Sanitizer;

/// [`Sanitizer`] restricts to between `min` and `max`.
///
/// # Examples
///
/// The example below sanitizes an f32 to be clamped to between -1.0 and 1.0.
/// Because of the newtype's guarantees, the constructed `Movement` will
/// always have an inner f32 between -1.0 and 1.0.
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp { min: -1.0, max: 1.0 }))]
/// struct Movement(f32);
///
/// assert_eq!(Movement::try_new(-2.0).unwrap().into_inner(), -1.0);
/// assert_eq!(Movement::try_new(-1.0).unwrap().into_inner(), -1.0);
/// assert_eq!(Movement::try_new(0.0).unwrap().into_inner(), 0.0);
/// assert_eq!(Movement::try_new(1.0).unwrap().into_inner(), 1.0);
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
/// The example below sanitizes an f32 to be clamped to greater than or equal to
/// 0.0. Because of the newtype's guarantees, the constructed `Distance` will
/// always have an inner f32 greater than or equal to 0.0.
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp_min(0.0)))]
/// struct Distance(f32);
///
/// assert_eq!(Distance::try_new(-1.0).unwrap().into_inner(), 0.0);
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
/// The example below sanitizes an f32 to be clamped to less than or equal to
/// 100.0. Because of the newtype's guarantees, the constructed `BatteryCharge`
/// will always have an inner f32 less than or equal to 100.0.
///
/// ```
/// use seventy::{builtins::clamp::*, seventy, Newtype};
///
/// #[seventy(sanitize(clamp_max(100)))]
/// struct BatteryCharge(u8);
///
/// assert_eq!(BatteryCharge::try_new(101).unwrap().into_inner(), 100);
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
