use seventy::{seventy, Newtype};

#[test]
fn generic_type_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T>(T);

    assert_eq!(Generic::<i32>::try_new(5i32).unwrap().into_inner(), 5);
}

#[test]
fn generic_type_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T, U>((T, U));

    assert_eq!(
        Generic::<i32, char>::try_new((5, 'x'))
            .unwrap()
            .into_inner(),
        (5, 'x')
    );
}

#[test]
fn generic_lifetime_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a>(&'a i32);

    assert_eq!(Generic::<'_>::try_new(&5).unwrap().into_inner(), &5);
}

#[test]
fn generic_lifetime_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, 'b>(&'a &'b i32);

    assert_eq!(Generic::<'_, '_>::try_new(&&5).unwrap().into_inner(), &&5);
}

#[test]
fn generic_mixed_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, T>(&'a T);

    assert_eq!(Generic::<'_, i32>::try_new(&5).unwrap().into_inner(), &5);
}

#[test]
fn generic_mixed_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, 'b, T, U>(&'a (T, &'b U));

    assert_eq!(
        Generic::<'_, '_, bool, f32>::try_new(&(true, &5.0))
            .unwrap()
            .into_inner(),
        &(true, &5.0)
    );
}
