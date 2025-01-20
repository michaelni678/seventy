use std::sync::atomic::{AtomicUsize, Ordering};

use seventy::{
    core::{Sanitizer, Validator},
    seventy, Newtype,
};

#[test]
fn generic_type_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T>(T);

    assert_eq!(Generic::<i32>::try_new(70i32).unwrap().into_inner(), 70);

    assert_eq!(Generic::<char>::try_new('x').unwrap().into_inner(), 'x');

    assert_eq!(
        Generic::<&str>::try_new("value").unwrap().into_inner(),
        "value"
    );
}

#[test]
fn generic_type_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T, U>((T, U));

    assert_eq!(
        Generic::<i32, char>::try_new((70, 'x'))
            .unwrap()
            .into_inner(),
        (70, 'x')
    );

    assert_eq!(
        Generic::<&str, bool>::try_new(("value", true))
            .unwrap()
            .into_inner(),
        ("value", true)
    );
}

#[test]
fn generic_lifetime_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a>(&'a i32);

    assert_eq!(Generic::<'_>::try_new(&70).unwrap().into_inner(), &70);
}

#[test]
fn generic_lifetime_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, 'b>(&'a &'b i32);

    assert_eq!(Generic::<'_, '_>::try_new(&&70).unwrap().into_inner(), &&70);
}

#[test]
fn generic_mixed_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, T>(&'a T);

    assert_eq!(Generic::<'_, i32>::try_new(&70).unwrap().into_inner(), &70);

    assert_eq!(
        Generic::<'_, &str>::try_new(&"value").unwrap().into_inner(),
        &"value"
    );
}

#[test]
fn generic_mixed_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, 'b, T, U>(&'a (T, &'b U));

    assert_eq!(
        Generic::<'_, '_, f32, char>::try_new(&(70.70, &'x'))
            .unwrap()
            .into_inner(),
        &(70.70, &'x')
    );
}

/// NOTE: This test is closely related to `sanitizer_construction_count` in the
/// test root.
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

/// NOTE: This test is closely related to `validator_construction_count` in the
/// test root.
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
