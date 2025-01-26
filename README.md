<h1 align="center">Seventy</h1>
<h3 align="center">Rust newtype sanitization & validation</h3>
<div align="center">

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-seventy-58a78a?style=for-the-badge&logo=Docs.rs">](https://docs.rs/seventy)
&nbsp;&nbsp;&nbsp;
[<img alt="crates.io" src="https://img.shields.io/crates/v/seventy?style=for-the-badge&logo=Rust">](https://crates.io/crates/seventy)
&nbsp;&nbsp;&nbsp;
[<img alt="github" src="https://img.shields.io/badge/github-seventy-gray?style=for-the-badge&logo=GitHub&color=669bbc">](https://github.com/michaelni678/seventy)

</div>

**Seventy** is a simple [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
sanitizer and validator. 

- **Why newtypes?**

    Newtypes provide compile-time guarantees that your program is using the correct values by wrapping existing types.

- **Why sanitize?**

    Newtypes are sanitized during construction, ensuring the values conform to the formats you expect.

- **Why validate?**

    Newtypes are validated during construction, ensuring it's impossible to create a newtype with values that don't meet the defined rules.

There is no error handling. If you need to know why the newtype couldn't be created, this crate 
isn't for you.

## Usage

The example below first trims the string and then validates if the trimmed string is both 
alphanumeric and between 5 to 20 characters long. The `display` upgrade automatically implements
the `Display` trait.

```rust
use seventy::{
    builtins::{compare::*, string::*},
    seventy, Newtype,
};

#[seventy(
    upgrades(display),
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
