//! Credit card built-ins.

use crate::core::Validator;

pub use credit_card_util::Type as CreditCardIssuer;

/// [`Validator`] checks if valid credit card number.
pub struct credit_card_number;

impl<T> Validator<T> for credit_card_number
where
    T: AsRef<str>,
{
    fn validate(&self, target: &T) -> bool {
        credit_card_util::Validate::from(target.as_ref()).is_ok()
    }
}

/// [`Validator`] checks if valid credit card number and forwards issuer to inner validator.
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
