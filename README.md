<h1 align="center"">Seventy</h1>
<h3 align="center">Rust newtype sanitization & validation</h3>
<div align="center">

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-seventy-58a78a?style=for-the-badge&logo=Docs.rs">](https://docs.rs/seventy)
&nbsp;&nbsp;&nbsp;
[<img alt="crates.io" src="https://img.shields.io/crates/v/seventy?style=for-the-badge&logo=Rust">](https://crates.io/crates/seventy)
&nbsp;&nbsp;&nbsp;
[<img alt="github" src="https://img.shields.io/badge/github-seventy-gray?style=for-the-badge&logo=GitHub&color=669bbc">](https://github.com/michaelni678/seventy)

</div>

Seventy is a simple [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) 
sanitizer and validator. 

There is no error handling. If you need to know why the newtype couldn't be 
created, this crate isn't for you.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
seventy = "0.1.0"
```

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

See the [examples](examples) directory for more!
