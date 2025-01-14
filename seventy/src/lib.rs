#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! # Seventy
//!
//! Seventy is a simple newtype sanitizer and validator.
//!
//! The [`macro@seventy`] procedural macro is provided to automatically implement
//! sanitization, validation, and other logic.
//!
//! ## Sanitizing
//!
//! Sanitization mutates a target. Sanitization is run before validation.
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
//! ## Validating
//!
//! Validation checks if a target adheres to a set of rules. Validation
//! is run after sanitization.
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
//! ## Upgrading
//!
//! Upgrades automatically implement useful functionality. More about upgrades
//! and the different types of upgrades can be found in the documentation
//! for the [`macro@seventy`] procedural macro.
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

extern crate self as seventy;

// Re-export the procedural macro.
pub use seventy_macros::seventy;

// Re-export the newtype trait.
pub use core::Newtype;

pub mod builtins;
pub mod core;
