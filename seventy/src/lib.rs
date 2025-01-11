#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_notable_trait))]

//! Seventy is a simple newtype sanitizer and validator.

extern crate self as seventy;

// Re-export the procedural macro.
pub use seventy_macros::seventy;

pub mod builtins;
pub mod core;
