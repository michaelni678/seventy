//! Assignment operator built-ins.

use crate::core::Sanitizer;

/// [`Sanitizer`] assigns the target to the given value.
///
/// # Examples
///
/// ```
/// use seventy::{builtins::op_assign::*, seventy, Newtype};
///
/// #[seventy(sanitize(assign(5)))]
/// pub struct Five(i32);
///
/// // Assigns to 5.
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
