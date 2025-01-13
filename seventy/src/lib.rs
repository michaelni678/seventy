#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Seventy is a simple newtype sanitizer and validator.

extern crate self as seventy;

// Re-export the procedural macro.
pub use seventy_macros::seventy;

// Re-export the newtype trait.
pub use core::Newtype;

pub mod builtins;
pub mod core;
