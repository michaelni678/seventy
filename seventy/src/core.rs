//! Core functionality.

/// Defines a newtype.
///
/// If you want to call this function without the trait being in scope,
/// consider enabling the `inherent` upgrade.
pub trait Newtype: Sized {
    /// The inner value of the new type.
    type Inner;

    /// Attempt to construct the newtype.
    ///
    /// Returns the inner value if the newtype couldn't be constructed.
    ///
    /// If a concrete parameter is preferred, use the `try_from` upgrade.
    fn try_new(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner>;

    /// Get the inner value.
    ///
    /// This is an alternative to the `as_ref` upgrade.
    fn to_inner(&self) -> &Self::Inner;

    /// Convert to the inner value.
    fn into_inner(self) -> Self::Inner;
}

/// A newtype that can be sanitized.
///
/// This is implemented automatically when using the [`seventy`] macro.
pub trait Sanitizable: Newtype {
    /// Get the sanitizer.
    fn sanitizer() -> &'static dyn Sanitizer<Self::Inner>;
}

/// A newtype that can be validated.
///
/// This is implemented automatically when using the [`seventy`] macro.
pub trait Validatable: Newtype {
    /// Get the validator.
    fn validator() -> &'static dyn Validator<Self::Inner>;
}

/// A newtype with the `bypassable` upgrade.
///
/// This is implemented automatically when using the [`seventy`] macro
/// if the `bypassable` upgrade is enabled.
///
/// All functions this trait provides are marked as unsafe, because they
/// violate the newtype's guarantees.
pub trait Bypassable: Newtype {
    /// Construct the newtype, skipping sanitization and validation.
    ///
    /// # Safety
    ///
    /// The constructed instance may violate the newtype's guarantees.
    unsafe fn unchecked_new(inner: impl Into<Self::Inner>) -> Self;

    /// Construct the newtype, skipping sanitization.
    ///
    /// # Safety
    ///
    /// The constructed instance may violate the newtype's sanitization
    /// guarantee.
    unsafe fn unsanitized_new(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner>;

    /// Construct the newtype, skipping validation.
    ///
    /// # Safety
    ///
    /// The constructed instance may violate the newtype's validation
    /// guarantee.
    unsafe fn unvalidated_new(inner: impl Into<Self::Inner>) -> Self;

    /// Get the inner value mutably.
    ///
    /// # Safety
    ///
    /// Mutating the instance may violate the newtype's guarantees.
    unsafe fn to_inner_mut(&mut self) -> &mut Self::Inner;
}

/// Sanitization logic.
pub trait Sanitizer<T>
where
    T: ?Sized,
{
    /// Sanitizes the given target.
    fn sanitize(&self, target: &mut T);
}

/// Validation logic.
pub trait Validator<T>
where
    T: ?Sized,
{
    /// Validates the given target.
    fn validate(&self, target: &T) -> bool;
}
