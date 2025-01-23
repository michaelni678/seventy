use crate::core::Sanitizer;

/// [`Sanitizer`] assigns the target to the given value.
///
/// # Examples
///
/// The example below assigns the target to 5. Because of the newtype's
/// guarantees, the constructed `Five` will always have an inner number that is
/// 5.
///
/// ```
/// use seventy::{builtins::operator::*, seventy, Newtype};
///
/// #[seventy(sanitize(assign(5)))]
/// pub struct Five(i32);
///
/// assert_eq!(Five::try_new(3).unwrap().into_inner(), 5);
/// ```
pub struct assign<T>(pub T);

impl<T> Sanitizer<T> for assign<T>
where
    T: Clone,
{
    fn sanitize(&self, target: &mut T) {
        *target = self.0.clone();
    }
}
