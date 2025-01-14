#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! # Seventy
//!
//! Seventy is a simple [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) sanitizer and validator.
//!
//! The [`macro@seventy`] procedural macro is provided to automatically implement sanitization, validation, and other logic.
//!
//! ## Sanitizing
//!
//! Sanitization mutates a target. Sanitization is run before validation.
//!
//! ### Examples
//!
//! The example below sanitizes the inner string by trimming surrounding whitespace.
//!
//! ```
//! use seventy::{builtins::string::*, seventy, Newtype};
//!
//! #[seventy(sanitize(trim))]
//! pub struct Username(String);
//!
//! assert_eq!(
//!     Username::try_new("   username   ").unwrap().into_inner(),
//!     "username"
//! );
//! ```
//!
//! ### Custom Sanitizers
//!
//! Custom sanitizers can be defined with the [`Sanitizer`] trait.
//!
//! The sanitizer below divides the target by the given value.
//!
//! ```
//! use std::ops::DivAssign;
//!
//! use seventy::{core::Sanitizer, seventy, Newtype};
//!
//! #[allow(non_snake_case)]
//! pub struct divide_by<T>(pub T);
//!
//! impl<T> Sanitizer<T> for divide_by<T>
//! where
//!     T: DivAssign<T> + Copy,
//! {
//!     fn sanitize(&self, target: &mut T) {
//!         target.div_assign(self.0);
//!     }
//! }
//!
//! #[seventy(sanitize(divide_by(5.0)))]
//! pub struct DivideBy(f32);
//!
//! assert_eq!(DivideBy::try_new(10.0).unwrap().into_inner(), 2.0);
//! ```
//!
//! ## Validating
//!
//! Validation checks if a target adheres to a set of rules. Validation
//! is run after sanitization.
//!
//! ### Examples
//!
//! The example below validates the inner string is alphanumeric.
//!
//! ```
//! use seventy::{builtins::string::*, seventy, Newtype};
//!
//! #[seventy(validate(alphanumeric))]
//! pub struct Username(String);
//!
//! assert!(Username::try_new("username").is_ok());
//! assert!(Username::try_new("u$ername").is_err());
//! ```
//!
//! ### Custom Validators
//!
//! Custom validators can be defined with the [`Validator`] trait.
//!
//! The validator below checks if the target is even.
//!
//! ```
//! use seventy::{core::Validator, seventy, Newtype};
//!
//! #[allow(non_snake_case)]
//! pub struct even_i64;
//!
//! impl Validator<i64> for even_i64 {
//!     fn validate(&self, target: &i64) -> bool {
//!         target.abs() % 2 == 0
//!     }
//! }
//!
//! #[seventy(validate(even_i64))]
//! pub struct EvenI64(i64);
//!
//! assert!(EvenI64::try_new(2).is_ok());
//! assert!(EvenI64::try_new(3).is_err());
//! ```
//!
//! ### Errors?
//!
//! Seventy does not support error handling. A validator only returns a boolean indicating
//! whether the validation result is valid or invalid. If you need to know the specific
//! reason why a newtype couldn't be created, Seventy is not the crate for you.
//!
//! ## Upgrading
//!
//! Upgrades automatically implement useful functionality. More about upgrades
//! and the different types of upgrades can be found in the documentation
//! for the [`macro@seventy`] procedural macro.
//!
//! ### Examples
//!
//! The example below uses the `deref` upgrade, which implements `Deref` on
//! the newtype.
//!
//! ```
//! use seventy::{builtins::string::*, seventy, Newtype};
//!
//! #[seventy(upgrades(deref))]
//! pub struct Username(String);
//!
//! let username = Username::try_new("username").unwrap();
//! assert_eq!(*username, "username");
//! ```
//!
//! ## Incorporating it all
//!
//! ### Examples
//!
//! The example below first trims the target and then validates if the trimmed
//! target is alphanumeric.
//!
//! ```
//! use seventy::{builtins::string::*, seventy, Newtype};
//!
//! #[seventy(upgrades(deref), sanitize(trim), validate(alphanumeric))]
//! pub struct Username(String);
//!
//! let username = Username::try_new("   username   ").unwrap();
//! assert_eq!(*username, "username");
//!
//! assert!(Username::try_new("   u$ername   ").is_err());
//! ```
//!
//! [`Sanitizer`]: seventy::core::Sanitizer
//! [`Validator`]: seventy::core::Validator

extern crate self as seventy;

// Re-export the procedural macro.
pub use seventy_macros::seventy;

// Re-export the newtype trait.
pub use core::Newtype;

pub mod builtins;
pub mod core;
