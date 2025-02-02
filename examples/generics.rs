//! A generic newtype.

use seventy::{builtins::collection::*, core::Bypassable, seventy};

/// A vec that is always sorted.
#[seventy(
    upgrades(bypassable, deref, independent, inherent, unexposed),
    sanitize(sort)
)]
pub struct SortedVec<T>(Vec<T>)
where
    T: Ord;

impl<T> SortedVec<T>
where
    T: Ord,
{
    /// Push an item, maintaining sorted order.
    pub fn push(&mut self, item: T) {
        let index = self.binary_search(&item).unwrap_or_else(|index| index);

        // SAFETY: Sorted order will be maintained after insertion, so the
        // newtype's guarantees will not be violated.
        unsafe { self.as_inner_mut() }.insert(index, item);
    }
}

fn main() {
    {
        assert_eq!(
            *SortedVec::try_new([0, 3, 4, 2, 1]).unwrap(),
            [0, 1, 2, 3, 4]
        );
    }

    {
        let mut sv = SortedVec::try_new(['h', 'e', 'l', 'l', 'o']).unwrap();

        assert_eq!(*sv, ['e', 'h', 'l', 'l', 'o']);

        sv.push('i');

        // Assert the sorted vec is still sorted.
        assert_eq!(*sv, ['e', 'h', 'i', 'l', 'l', 'o']);
    }
}
