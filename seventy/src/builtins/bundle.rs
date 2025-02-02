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

#[doc(hidden)]
#[macro_export]
macro_rules! _bundle {
    ($v1:expr) => {
        $v1
    };
    ($v1:expr, $v2:expr) => {
        ($v1, $v2)
    };
    ($v1:expr, $v2:expr, $v3:expr) => {
        ($v1, $v2, $v3)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr) => {
        ($v1, $v2, $v3, $v4)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr) => {
        ($v1, $v2, $v3, $v4, $v5)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr, $v9:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8, $v9)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr, $v9:expr, $v10:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8, $v9, $v10)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr, $v9:expr, $v10:expr, $v11:expr) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8, $v9, $v10, $v11)
    };
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr, $v9:expr, $v10:expr, $v11:expr, $($rest:tt)*) => {
        ($v1, $v2, $v3, $v4, $v5, $v6, $v7, $v8, $v9, $v10, $v11, $crate::builtins::bundle::bundle!($($rest)*))
    };
}

pub use _bundle as bundle;

#[cfg(test)]
mod tests {
    use crate::builtins::debug::*;

    use super::*;

    #[test]
    fn bundle_1() {
        matches!(bundle!(invalid), invalid);
    }

    #[test]
    fn bundle_2() {
        matches!(bundle!(invalid, invalid), (invalid, invalid));
    }

    #[test]
    fn bundle_3() {
        matches!(bundle!(invalid, invalid, invalid), (invalid, invalid, invalid));
    }

    #[test]
    fn bundle_4() {
        matches!(
            bundle!(invalid, invalid, invalid, invalid),
            (invalid, invalid, invalid, invalid)
        );
    }

    #[test]
    fn bundle_5() {
        matches!(
            bundle!(invalid, invalid, invalid, invalid, invalid),
            (invalid, invalid, invalid, invalid, invalid)
        );
    }

    #[test]
    fn bundle_6() {
        matches!(
            bundle!(invalid, invalid, invalid, invalid, invalid, invalid),
            (invalid, invalid, invalid, invalid, invalid, invalid)
        );
    }

    #[test]
    fn bundle_7() {
        matches!(
            bundle!(invalid, invalid, invalid, invalid, invalid, invalid, invalid),
            (invalid, invalid, invalid, invalid, invalid, invalid, invalid)
        );
    }

    #[test]
    fn bundle_8() {
        matches!(
            bundle!(invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid),
            (invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid)
        );
    }

    #[test]
    fn bundle_9() {
        matches!(
            bundle!(
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            ),
            (
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            )
        );
    }

    #[test]
    fn bundle_10() {
        matches!(
            bundle!(
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            ),
            (
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            )
        );
    }

    #[test]
    fn bundle_11() {
        matches!(
            bundle!(
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            ),
            (
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            )
        );
    }

    #[test]
    fn bundle_12() {
        matches!(
            bundle!(
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            ),
            (
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            )
        );
    }

    #[test]
    fn bundle_13() {
        matches!(
            bundle!(
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid
            ),
            (
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                invalid,
                (invalid, invalid)
            )
        );
    }
}
