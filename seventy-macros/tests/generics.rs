use seventy::seventy;

#[test]
fn generic_type_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T>(T);
}

#[test]
fn generic_type_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<T, U>((T, U));
}

#[test]
fn generic_lifetime_single() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a>(&'a i32);
}

#[test]
fn generic_lifetime_multiple() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, 'b>(&'a &'b i32);
}

#[test]
fn generic_mixed() {
    #[seventy(upgrades(independent))]
    pub struct Generic<'a, T>(&'a T);
}
