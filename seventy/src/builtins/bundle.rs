//! Bundle built-ins.
//!
//! A bundle combines multiple sanitizers / validators into one.
//!
//! [`Sanitizer`] and [`Validator`] are both implemented for up to a tuple arity
//! of 12.
//!
//! For example, `some_then` only takes a single validator. Using a bundle,
//! two validators (`alphabetic` and `length`) can be combined into one.
//!
//! ```
//! use seventy::{
//!     builtins::{compare::*, option::*, string::*},
//!     seventy,
//! };
//!
//! #[seventy(
//!     validate(some_then((alphabetic, length::chars(gt(2))))),
//! )]
//! pub struct MiddleName(Option<String>);
//! ```

use crate::core::{Sanitizer, Validator};

impl<T> Sanitizer<T> for () {
    fn sanitize(&self, _target: &mut T) {}
}

macro_rules! impl_bundle_sanitizer {
    ($($name:ident),*) => {
        impl<T, $($name),*> $crate::core::Sanitizer<T> for ($($name,)*)
        where
            $($name: $crate::core::Sanitizer<T>,)*
        {
            fn sanitize(&self, target: &mut T) {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                $($name.sanitize(target);)*
            }
        }
    };
}

impl_bundle_sanitizer!(S1);
impl_bundle_sanitizer!(S1, S2);
impl_bundle_sanitizer!(S1, S2, S3);
impl_bundle_sanitizer!(S1, S2, S3, S4);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7, S8);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7, S8, S9);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11);
impl_bundle_sanitizer!(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12);

impl<T> Validator<T> for () {
    fn validate(&self, _target: &T) -> bool {
        true
    }
}

macro_rules! impl_bundle_validator {
    ($($name:ident),*) => {
        impl<T, $($name),*> $crate::core::Validator<T> for ($($name,)*)
        where
            $($name: $crate::core::Validator<T>,)*
        {
            fn validate(&self, target: &T) -> bool {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                $($name.validate(target))&&*
            }
        }
    };
}

impl_bundle_validator!(V1);
impl_bundle_validator!(V1, V2);
impl_bundle_validator!(V1, V2, V3);
impl_bundle_validator!(V1, V2, V3, V4);
impl_bundle_validator!(V1, V2, V3, V4, V5);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7, V8);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
impl_bundle_validator!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);

/// [`Sanitizer`] and [`Validator`] expands into bundle tuples.
///
/// Only tuples up to an arity of 12 implement [`Sanitizer`] and [`Validator`].
/// Luckily, bundles can be nested within each other! This macro takes any
/// number of sanitizers or validators and expands into the necessary bundle
/// tuples.
#[doc(hidden)]
#[macro_export]
macro_rules! _bundle {
    () => {
        ()
    };
    ($sv1:expr) => {
        $sv1
    };
    ($sv1:expr, $sv2:expr) => {
        ($sv1, $sv2)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr) => {
        ($sv1, $sv2, $sv3)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr) => {
        ($sv1, $sv2, $sv3, $sv4)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr, $sv8:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7, $sv8)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr, $sv8:expr, $sv9:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7, $sv8, $sv9)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr, $sv8:expr, $sv9:expr, $sv10:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7, $sv8, $sv9, $sv10)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr, $sv8:expr, $sv9:expr, $sv10:expr, $sv11:expr) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7, $sv8, $sv9, $sv10, $sv11)
    };
    ($sv1:expr, $sv2:expr, $sv3:expr, $sv4:expr, $sv5:expr, $sv6:expr, $sv7:expr, $sv8:expr, $sv9:expr, $sv10:expr, $sv11:expr, $($rest:tt)*) => {
        ($sv1, $sv2, $sv3, $sv4, $sv5, $sv6, $sv7, $sv8, $sv9, $sv10, $sv11, $crate::builtins::bundle::bundle!($($rest)*))
    };
}

#[doc(inline)]
pub use _bundle as bundle;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_camel_case_types)]
    pub struct sv<const TAG: u128 = 0>;

    #[test]
    fn bundle_1() {
        matches!(bundle!(sv::<1>), sv::<1>);
    }

    #[test]
    fn bundle_2() {
        matches!(bundle!(sv::<1>, sv::<2>), (sv::<1>, sv::<2>));
    }

    #[test]
    fn bundle_3() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>),
            (sv::<1>, sv::<2>, sv::<3>)
        );
    }

    #[test]
    fn bundle_4() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>, sv::<4>),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>)
        );
    }

    #[test]
    fn bundle_5() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>)
        );
    }

    #[test]
    fn bundle_6() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>)
        );
    }

    #[test]
    fn bundle_7() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>)
        );
    }

    #[test]
    fn bundle_8() {
        matches!(
            bundle!(sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>)
        );
    }

    #[test]
    fn bundle_9() {
        matches!(
            bundle!(
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>
            ),
            (sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>)
        );
    }

    #[test]
    fn bundle_10() {
        matches!(
            bundle!(
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>
            ),
            (
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>
            )
        );
    }

    #[test]
    fn bundle_11() {
        matches!(
            bundle!(
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>, sv::<11>
            ),
            (
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>, sv::<11>
            )
        );
    }

    #[test]
    fn bundle_12() {
        matches!(
            bundle!(
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>, sv::<11>, sv::<12>
            ),
            (
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>, sv::<11>, sv::<12>
            )
        );
    }

    #[test]
    fn bundle_13() {
        matches!(
            bundle!(
                sv::<1>, sv::<2>, sv::<3>, sv::<4>, sv::<5>, sv::<6>, sv::<7>, sv::<8>, sv::<9>,
                sv::<10>, sv::<11>, sv::<12>, sv::<13>
            ),
            (
                sv::<1>,
                sv::<2>,
                sv::<3>,
                sv::<4>,
                sv::<5>,
                sv::<6>,
                sv::<7>,
                sv::<8>,
                sv::<9>,
                sv::<10>,
                sv::<11>,
                (sv::<12>, sv::<13>)
            )
        );
    }
}
