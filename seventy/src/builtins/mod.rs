#![allow(non_camel_case_types)]

//! Built-in sanitizers and validators.

pub mod bundle;
pub mod clamp;
pub mod collection;
pub mod compare;
#[cfg(feature = "credit-card")]
pub mod credit_card;
#[cfg(feature = "email")]
pub mod email;
pub mod float;
pub mod option;
pub mod string;
#[cfg(feature = "url")]
pub mod url;
