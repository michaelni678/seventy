use std::sync::atomic::{AtomicUsize, Ordering};

use seventy::{
    core::{Sanitizer, Validator},
    seventy, Newtype,
};

#[allow(non_camel_case_types)]
pub struct sanitizer;

impl sanitizer {
    pub fn new(count: &AtomicUsize) -> Self {
        let c = count.load(Ordering::Relaxed);
        count.store(c + 1, Ordering::Relaxed);

        Self
    }
}

impl Sanitizer<()> for sanitizer {
    fn sanitize(&self, _target: &mut ()) {}
}

#[test]
fn sanitizer_shared() {
    static NEW_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[seventy(sanitize(sanitizer::new(&NEW_COUNT)))]
    pub struct Shared(());

    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 0);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);
}

#[test]
fn sanitizer_independent() {
    static NEW_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[seventy(upgrades(independent), sanitize(sanitizer::new(&NEW_COUNT)))]
    pub struct Shared(());

    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 0);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 2);
}

#[allow(non_camel_case_types)]
pub struct validator;

impl validator {
    pub fn new(count: &AtomicUsize) -> Self {
        let c = count.load(Ordering::Relaxed);
        count.store(c + 1, Ordering::Relaxed);

        Self
    }
}

impl Validator<()> for validator {
    fn validate(&self, _target: &()) -> bool {
        true
    }
}

#[test]
fn validator_shared() {
    static NEW_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[seventy(validate(validator::new(&NEW_COUNT)))]
    pub struct Shared(());

    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 0);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);
}

#[test]
fn validator_independent() {
    static NEW_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[seventy(upgrades(independent), validate(validator::new(&NEW_COUNT)))]
    pub struct Shared(());

    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 0);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 1);

    let _ = Shared::try_new(());
    assert_eq!(NEW_COUNT.load(Ordering::Relaxed), 2);
}
