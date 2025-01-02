use string_cache::DefaultAtom;

pub type Atom = DefaultAtom;

#[macro_export]
macro_rules! atom {
    ($value:expr) => {
        Atom::from($value)
    };
}
