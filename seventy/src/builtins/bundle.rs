//! Bundle built-ins.
//!
//! A bundle combines multiple sanitizers / validators into one.
//!
//! [`Sanitizer`] and [`Validator`] are both implemented for up to a tuple arity of 12.
//! 
//! For example, `some_then` only takes a single validator. Using a bundle,
//! two validators (`alphabetic` and `length`) can be combined into one.
//!
//! ```
//! use seventy::{
//!     builtins::{compare::*, string::*},
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
