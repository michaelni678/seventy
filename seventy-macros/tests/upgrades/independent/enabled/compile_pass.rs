use seventy::{seventy, Newtype};

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
