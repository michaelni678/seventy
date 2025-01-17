//! A generic newtype.

use seventy::{builtins::collection::*, seventy, Newtype};

#[seventy(upgrades(independent), sanitize(sort))]
pub struct SortedVec<T>(Vec<T>)
where
    T: Ord;

fn main() {
    assert_eq!(
        SortedVec::try_new([0, 3, 4, 2, 1]).unwrap().into_inner(),
        [0, 1, 2, 3, 4]
    );

    assert_eq!(
        SortedVec::try_new(['h', 'e', 'l', 'l', 'o'])
            .unwrap()
            .into_inner(),
        ['e', 'h', 'l', 'l', 'o']
    );
}
