//! Procedural macros for Seventy.

use proc_macro::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, Error, ItemStruct, Meta, Token};

mod seventy;

/// Newtype attribute.
///
/// Automatically implements the `Newtype`, `Sanitizable`, and `Validatable`
/// traits.
///
/// Other functionality may also be implemented depending on enabled upgrades.
///
/// # Upgrades
///
/// ## as_ref
///
/// Implements `AsRef` for the newtype. The `Newtype` trait already
/// has an equivalent `to_inner` method, but this provides compatability with
/// APIs that expect `AsRef`.
///
/// ```
/// use seventy::{core::Newtype, seventy};
///
/// #[seventy(upgrades(as_ref))]
/// pub struct Velocity(f64);
///
/// assert_eq!(*Velocity::try_new(70.0).unwrap().as_ref(), 70.0);
/// ```
///
/// ## bypassable
///
/// Enables bypass functionality for the newtype.
///
/// ```
/// use seventy::{
///     builtins::{compare::*, string::*},
///     core::{Bypassable, Validatable},
///     seventy, Newtype,
/// };
///
/// #[seventy(
///     upgrades(bypassable),
///     sanitize(trim),
///     validate(ascii, length::chars(within(5..=20)))
/// )]
/// pub struct Username(String);
///
/// /* `Bypassable::unchecked_new` */
///
/// let username = unsafe { Username::unchecked_new("   username!   ") };
/// assert_eq!(username.to_inner(), "   username!   ");
///
/// /* `Bypassable::unsanitized_new` */
///
/// let username = unsafe { Username::unsanitized_new("   username   ") }.unwrap();
/// assert_eq!(username.to_inner(), "   username   ");
///
/// /* `Bypassable::unvalidated_new` */
///
/// let username = unsafe { Username::unvalidated_new("   username!   ") };
/// assert_eq!(username.to_inner(), "username!");
///
/// /* `Bypassable::to_inner_mut` */
///
/// let mut username = Username::try_new("username").unwrap();
///
/// // Passes validation.
/// assert!(Username::validator().validate(username.to_inner()));
///
/// // Unsafely mutate the value.
/// unsafe { username.to_inner_mut() }.push_str("\u{00BF}");
///
/// // Fails validation.
/// assert!(!Username::validator().validate(username.to_inner()));
/// ```
///
/// ## deref
///
/// Implements `Deref` for the newtype.
///
/// ```
/// use seventy::{seventy, Newtype};
///
/// #[seventy(upgrades(deref))]
/// pub struct Sentence(String);
///
/// let sentence = Sentence::try_new("Hello, World!").unwrap();
/// assert_eq!(*sentence, "Hello, World!");
/// ```
///
/// ## deserializable
///
/// Implements `serde::Deserialize` for the newtype. You
/// must have `serde` as a dependency!
///
/// ```
/// use seventy::{seventy, Newtype};
///
/// #[seventy(upgrades(deserializable))]
/// pub struct Message(String);
///
/// let json = "\"Seventy is a cool crate\"";
///
/// let message: Message = serde_json::from_str(json).unwrap();
/// assert_eq!(message.into_inner(), "Seventy is a cool crate");
/// ```
///
/// ## inherent
///
/// Makes the `Newtype` trait methods callable without the trait
/// in scope.
///
/// The code below fails to compile, since the `Newtype` trait is not in scope.
///
/// ```compile_fail,E0599
/// use seventy::{builtins::compare::*, seventy};
///
/// #[seventy(
///     validate(within(1..=10))
/// )]
/// pub struct Rating(u8);
///
/// assert!(Rating::try_new(5).is_ok());
/// ```
///
/// The code below compiles due to the `inherent` upgrade.
///
/// ```
/// use seventy::{builtins::compare::*, seventy};
///
/// #[seventy(
///     upgrades(inherent),
///     validate(within(1..=10))
/// )]
/// pub struct Rating(u8);
///
/// assert!(Rating::try_new(5).is_ok());
/// ```
///
/// ## instanced
///
/// By default, sanitizers and validators for a newtype are
/// declared once per type definition and shared across all instances of the
/// newtype using a static variable. This approach is efficient but does not
/// support generics, as Rust prohibits generic parameters in static
/// variables. Enabling this option ensures that the sanitizer and validator
/// are constructed dynamically for each call to `sanitize` or `validate`,
/// making it compatible with generic newtypes at the cost of some
/// performance.
///
/// The code below fails to compile, due to the generic static issue.
///
/// ```compile_fail,E0401,E0282
/// use seventy::{builtins::collection::*, seventy};
///
/// #[seventy(sanitize(sort))]
/// pub struct SortedVec<T>(Vec<T>) where T: Ord + 'static;
/// ```
///
/// The code below compiles due to the `instanced` upgrade.
///
/// ```
/// use seventy::{builtins::collection::*, seventy, Newtype};
///
/// #[seventy(upgrades(instanced), sanitize(sort))]
/// pub struct SortedVec<T>(Vec<T>)
/// where
///     T: Ord + 'static;
///
/// assert_eq!(
///     SortedVec::try_new([3, 0, 2, 1]).unwrap().into_inner(),
///     [0, 1, 2, 3]
/// );
///
/// assert_eq!(
///     SortedVec::try_new(['s', 'e', 'v', 'e', 'n', 't', 'y'])
///         .unwrap()
///         .into_inner(),
///     ['e', 'e', 'n', 's', 't', 'v', 'y']
/// );
/// ```
///
/// ## serializable
///
/// Implements `serde::Serialize` for the newtype. You must
/// have `serde` as a dependency!
///
/// ```
/// use seventy::{seventy, Newtype};
///
/// #[seventy(upgrades(serializable))]
/// pub struct Message(String);
///
/// let message = Message::try_new("Seventy is a cool crate").unwrap();
/// let json = serde_json::to_string(&message).unwrap();
///
/// assert_eq!(json, "\"Seventy is a cool crate\"");
/// ```
///
/// ## try_from
///
/// Implements `TryFrom` for the newtype. The `Newtype` trait
/// already has the method `Newtype::try_new`, which is similar to
/// `TryFrom::try_from`, however the latter expects a concrete type, whereas
/// the former `Newtype::try_new` does not.
///
/// ```
/// use seventy::{seventy, Newtype};
///
/// #[seventy(upgrades(try_from))]
/// pub struct Number(i32);
///
/// assert!(Number::try_from(5).is_ok());
/// ```
///
/// ## unexposed
///
/// Prevents accessing the field directly from the same module.
///
/// **NOTE**:
/// When this upgrade is enabled, all attributes (such as derives) must be below
/// the `seventy` macro.
///
/// The code below modifies a newtype's value by directly accessing the field,
/// which is not good!
///
/// ```
/// use seventy::{seventy, Newtype};
///
/// #[seventy()]
/// pub struct ExposedToModule(i32);
///
/// let mut etm = ExposedToModule::try_new(70).unwrap();
/// etm.0 = 444;
///
/// assert_eq!(etm.into_inner(), 444);
/// ```
///
/// The code below unexposes the inner field, so the bad code now produces a
/// compilation error.
///
/// ```compile_fail,E0616
/// use seventy::{Newtype, seventy};
///
/// #[seventy(upgrades(unexposed))]
/// pub struct UnexposedToModule(i32);
///
/// let mut utm = UnexposedToModule::try_new(70).unwrap();
/// utm.0 = 444;
/// ```
#[proc_macro_attribute]
pub fn seventy(metas: TokenStream, item: TokenStream) -> TokenStream {
    let metas = match Punctuated::<Meta, Token![,]>::parse_terminated.parse(metas) {
        Ok(metas) => metas,
        Err(error) => return error.into_compile_error().into(),
    };

    let item = match syn::parse::<ItemStruct>(item) {
        Ok(item) => item,
        Err(error) => return error.into_compile_error().into(),
    };

    seventy::expand(metas, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
