use crate::core::{Sanitizer, Validator};

/// [`Validator`] checks if satisfies predicate.
///
/// # Examples
///
/// The example below validates the inner character is an ASCII digit. Because
/// of the newtype's guarantees, it is impossible to construct `AsciiDigit` with
/// an inner character that is not an ASCII digit.
///
/// ```
/// use seventy::{builtins::predicate::*, seventy, Newtype};
///
/// #[seventy(validate(satisfies(char::is_ascii_digit)))]
/// pub struct AsciiDigit(char);
///
/// assert!(AsciiDigit::try_new('0').is_ok());
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
/// The example below converts all ASCII whitespace to spaces (U+0020).
///
/// ```
/// use seventy::{
///     builtins::{operator::*, predicate::*},
///     seventy, Newtype,
/// };
///
/// #[seventy(sanitize(satisfies_then(char::is_ascii_whitespace, assign(' '))))]
/// pub struct SpacedChar(char);
///
/// assert_eq!(SpacedChar::try_new('c').unwrap().into_inner(), 'c');
/// assert_eq!(SpacedChar::try_new(' ').unwrap().into_inner(), ' ');
/// assert_eq!(SpacedChar::try_new('\n').unwrap().into_inner(), ' ');
/// ```
///
/// The example below validates the inner character is uppercase only if if is
/// also ASCII alphabetic. If the inner character is not ASCII alphabetic, it
/// can be lowercase. Because of the newtype's guarantees, it is impossible to
/// construct `MyChar` with an inner character that is lowercase if it is ASCII
/// alphabetic.
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
/// assert!(MyChar::try_new('A').is_ok());
///
/// assert!('\u{0180}'.is_lowercase());
/// assert!(MyChar::try_new('\u{0180}').is_ok());
///
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
