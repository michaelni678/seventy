use std::sync::atomic::{AtomicUsize, Ordering};

use seventy::{
    core::{Sanitizer, Validator},
    seventy, Newtype,
};

pub mod compile_pass;

#[test]
fn sanitizer_construction_count() {
    static SANITIZER_CONSTRUCTION_COUNT: AtomicUsize = AtomicUsize::new(0);
    static SANITIZER_SANITIZE_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[allow(non_camel_case_types)]
    pub struct s;

    impl s {
        pub fn new() -> Self {
            let count = SANITIZER_CONSTRUCTION_COUNT.load(Ordering::Relaxed);

            SANITIZER_CONSTRUCTION_COUNT.store(count + 1, Ordering::Relaxed);

            Self
        }
    }

    impl Sanitizer<()> for s {
        fn sanitize(&self, _target: &mut ()) {
            let count = SANITIZER_SANITIZE_COUNT.load(Ordering::Relaxed);

            SANITIZER_SANITIZE_COUNT.store(count + 1, Ordering::Relaxed);
        }
    }

    #[seventy(upgrades(independent), sanitize(s::new()))]
    pub struct Sanitized(());

    assert_eq!(SANITIZER_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 0);
    assert_eq!(SANITIZER_SANITIZE_COUNT.load(Ordering::Relaxed), 0);

    let _ = Sanitized::try_new(());

    assert_eq!(SANITIZER_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 1);
    assert_eq!(SANITIZER_SANITIZE_COUNT.load(Ordering::Relaxed), 1);

    let _ = Sanitized::try_new(());

    assert_eq!(SANITIZER_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 2);
    assert_eq!(SANITIZER_SANITIZE_COUNT.load(Ordering::Relaxed), 2);
}

#[test]
fn validator_construction_count() {
    static VALIDATOR_CONSTRUCTION_COUNT: AtomicUsize = AtomicUsize::new(0);
    static VALIDATOR_VALIDATE_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[allow(non_camel_case_types)]
    pub struct v;

    impl v {
        pub fn new() -> Self {
            let count = VALIDATOR_CONSTRUCTION_COUNT.load(Ordering::Relaxed);

            VALIDATOR_CONSTRUCTION_COUNT.store(count + 1, Ordering::Relaxed);

            Self
        }
    }

    impl Validator<()> for v {
        fn validate(&self, _target: &()) -> bool {
            let count = VALIDATOR_VALIDATE_COUNT.load(Ordering::Relaxed);

            VALIDATOR_VALIDATE_COUNT.store(count + 1, Ordering::Relaxed);

            true
        }
    }

    #[seventy(upgrades(independent), validate(v::new()))]
    pub struct Validated(());

    assert_eq!(VALIDATOR_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 0);
    assert_eq!(VALIDATOR_VALIDATE_COUNT.load(Ordering::Relaxed), 0);

    let _ = Validated::try_new(());

    assert_eq!(VALIDATOR_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 1);
    assert_eq!(VALIDATOR_VALIDATE_COUNT.load(Ordering::Relaxed), 1);

    let _ = Validated::try_new(());

    assert_eq!(VALIDATOR_CONSTRUCTION_COUNT.load(Ordering::Relaxed), 2);
    assert_eq!(VALIDATOR_VALIDATE_COUNT.load(Ordering::Relaxed), 2);
}
