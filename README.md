<h1 align=center><code>Seventy</code></h1>
<h3 align=center>Rust newtype sanitization & validation</h3>

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