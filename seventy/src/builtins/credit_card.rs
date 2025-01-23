//! Credit card built-ins.

use crate::core::Validator;

pub use credit_card_util::Type as CreditCardIssuer;

/// [`Validator`] checks if valid credit card number.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::credit_card::*, seventy, Newtype};
///
/// #[seventy(validate(credit_card_number))]
/// struct CreditCardNumber(String);
///
/// // Credit card numbers generated with <https://dnschecker.org/credit-card-generator.php>.
///
/// // Successfully constructed because the card number is valid.
/// let mastercard = "5265187007972395";
/// assert!(CreditCardNumber::try_new(mastercard).is_ok());
///
/// // Unsuccessfully constructed because the card number is invalid.
/// let invalid = "7070707070707070";
/// assert!(CreditCardNumber::try_new(invalid).is_err());
/// ```
pub struct credit_card_number;

impl<T> Validator<T> for credit_card_number
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        credit_card_util::Validate::from(target.as_ref()).is_ok()
    }
}

/// [`Validator`] checks if valid credit card number and forwards issuer to
/// inner validator.
///
/// # Examples
///
/// ```
/// use seventy::{
///     builtins::{collection::*, credit_card::*},
///     seventy, Newtype,
/// };
///
/// const ACCEPTED_ISSUERS: [CreditCardIssuer; 2] =
///     [CreditCardIssuer::Amex, CreditCardIssuer::Discover];
///
/// #[seventy(validate(credit_card_number_then(among(ACCEPTED_ISSUERS))))]
/// struct CreditCardNumber(String);
///
/// // Credit card numbers generated with <https://dnschecker.org/credit-card-generator.php>.
///
/// // Successfully constructed because the card number is valid and the issuer
/// // is among `ACCEPTED_ISSUERS`.
/// let amex = "377947337532813";
/// assert!(CreditCardNumber::try_new(amex).is_ok());
///
/// // Unsuccessfully constructed because although the card number is valid, the issuer
/// // is not among `ACCEPTED_ISSUERS`.
/// let mastercard = "5265187007972395";
/// assert!(credit_card_util::Validate::from(mastercard).is_ok());
/// assert!(CreditCardNumber::try_new(mastercard).is_err());
///
/// // Unsuccessfully constructed because the card number is invalid.
/// let invalid = "7070707070707070";
/// assert!(CreditCardNumber::try_new(invalid).is_err());
/// ```
pub struct credit_card_number_then<V>(pub V);

impl<T, V> Validator<T> for credit_card_number_then<V>
where
    T: AsRef<str>,
    V: Validator<CreditCardIssuer>,
{
    fn validate(&self, target: &T) -> bool {
        match credit_card_util::Validate::from(target.as_ref()) {
            Ok(cc_issuer) => self.0.validate(&cc_issuer.card_type),
            _error => false,
        }
    }
}
