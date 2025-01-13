<h1 align=center><code>Seventy</code></h1>
<h3 align=center>Rust newtype sanitization & validation</h3>

## Overview

- **Customizable**: Define your own sanitizers and validators to suit your needs.
- **Simplistic**: There is no error handling. If you need to know why the newtype couldn't be created, this crate isn't for you.

## Installation

Currently not on crates.io. Specify the dependency using this git repository instead.
```
seventy = { git = "https://github.com/michaelni678/seventy" }
```

## Usage

```rust
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