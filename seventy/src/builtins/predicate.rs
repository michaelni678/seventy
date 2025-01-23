//! Predicate built-ins.

use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if satisfies predicate.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::predicate::*, seventy, Newtype};
///
/// #[seventy(validate(satisfies(char::is_ascii_digit)))]
/// pub struct AsciiDigit(char);
///
/// // Successfully constructed because '0' satisfies `char::is_ascii_digit`.
/// assert!(AsciiDigit::try_new('0').is_ok());
///
/// // Unsuccessfully constructed because 'X' doesn't satisfy `char::is_ascii_digit`.
/// assert!(AsciiDigit::try_new('X').is_err());
/// ```
pub struct satisfies<F>(pub F);

impl<T, F> Validator<T> for satisfies<F>
where
    for<'a> F: Fn(&'a T) -> bool,
{
    fn validate(&self, target: &T) -> bool {
        (self.0)(target)
    }
}

/// [`Sanitizer`] and [`Validator`] forwards target if satisfies predicate.
///
/// For [`Sanitizer`], if not satisfied the inner sanitizer is skipped.
///
/// For [`Validator`], if not satisfied the inner validator is skipped and the
/// validation is valid.
///
/// Note that there is no built-in validator that mandates the predicate must be
/// satisfied for the validation to be considered valid, as this functionality
/// is effectively covered by using a bundle validator.
///
/// # Examples
///
/// An example of the `satisfies_then` sanitizer, which converts all ASCII
/// whitespace to spaces (U+0020).
///
/// ```
/// use seventy::{
///     builtins::{op_assign::*, predicate::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(sanitize(satisfies_then(char::is_ascii_whitespace, assign(' '))))]
/// pub struct SpacedChar(char);
///
/// // No changes because satisfies `char::is_ascii_whitespace`.
/// assert_eq!(SpacedChar::try_new('c').unwrap().into_inner(), 'c');
/// assert_eq!(SpacedChar::try_new(' ').unwrap().into_inner(), ' ');
///
/// // Assigns to ' ' because '\n' satisfies `char::is_ascii_whitespace`.
/// assert_eq!(SpacedChar::try_new('\n').unwrap().into_inner(), ' ');
/// ```
///
/// An example of the `satisfies_then` validator, which requires uppercase for
/// only ASCII alphabetic characters.
///
/// ```
/// use seventy::{
///     builtins::{char::*, predicate::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(validate(satisfies_then(char::is_ascii_alphabetic, uppercase)))]
/// pub struct MyChar(char);
///
/// // Successfully constructed because 'A' satisfies `char::is_ascii_alphabetic` and is uppercase.
/// assert!(MyChar::try_new('A').is_ok());
///
/// // Successfully constructed because '\u{0180}' does not satisfy `char::is_ascii_alphabetic`,
/// // and so it doesn't matter if it is uppercase or not.
/// assert!('\u{0180}'.is_lowercase());
/// assert!(MyChar::try_new('\u{0180}').is_ok());
///
/// // Unsuccessfully constructed because 'a' satisfies `char::is_ascii_alphabetic` but is not
/// // uppercase.
/// assert!(MyChar::try_new('a').is_err());
/// ```
pub struct satisfies_then<F, SV>(pub F, pub SV);

impl<T, F, S> Sanitizer<T> for satisfies_then<F, S>
where
    for<'a> F: Fn(&'a T) -> bool,
    S: Sanitizer<T>,
{
    fn sanitize(&self, target: &mut T) {
        if (self.0)(target) {
            self.1.sanitize(target);
        }
    }
}

impl<T, F, V> Validator<T> for satisfies_then<F, V>
where
    for<'a> F: Fn(&'a T) -> bool,
    V: Validator<T>,
{
    fn validate(&self, target: &T) -> bool {
        if (self.0)(target) {
            self.1.validate(target)
        } else {
            true
        }
    }
}
