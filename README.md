<h1 align=center>Seventy</h1>
<h3 align=center>Rust newtype sanitization & validation</h3>

<div align="center">

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/seventy.svg
[crates-url]: https://crates.io/crates/seventy
[docs-badge]: https://docs.rs/seventy/badge.svg
[docs-url]: https://docs.rs/seventy

</div>

## Overview

Seventy is a simple [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) 
sanitizer and validator. 

There is no error handling. If you need to know why the newtype couldn't be 
created, this crate isn't for you.

## Usage

```rust
use seventy::{
    builtins::{compare::*, string::*},
    seventy, Newtype,
};

#[seventy(
    sanitize(trim),
    validate(alphanumeric, length::chars(within(5..=20))),
)]
pub struct Username(String);

assert_eq!(
    Username::try_new("   username   ").unwrap().into_inner(),
    "username"
);

assert!(Username::try_new("   u$ername   ").is_err());
```