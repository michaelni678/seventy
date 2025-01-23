#![allow(non_camel_case_types)]

//! Built-in sanitizers and validators.
//!
//! # Naming Convention
//!
//! The built-in sanitizers and validators are all **snake_case**. This is
//! due to a couple of reasons:
//!
//! - **Avoiding name collisions**: Using snake_case prevents name collisions
//!   with newtypes, which are PascalCase (aka UpperCamelCase).
//!
//! - **Simulate attributes**: In the [`seventy`] proc-macro, sanitizers and
//!   validators are used as part of attributes and resemble nested attributes.
//!   To maintain consistency, it's more fitting for built-ins to follow the
//!   attribute naming convention.
//!
//! [`seventy`]: macro@seventy::seventy

pub mod bundle;
pub mod char;
pub mod clamp;
pub mod collection;
pub mod compare;
#[cfg(feature = "credit-card")]
pub mod credit_card;
#[cfg(feature = "email")]
pub mod email;
pub mod float;
pub mod op_assign;
pub mod op_logical;
pub mod option;
pub mod predicate;
pub mod string;
#[cfg(feature = "url")]
pub mod url;
