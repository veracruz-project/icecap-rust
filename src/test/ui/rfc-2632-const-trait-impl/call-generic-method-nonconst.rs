#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]

struct S;

impl PartialEq for S {
    fn eq(&self, _: &S) -> bool {
        true
    }
}

const fn equals_self<T: PartialEq>(t: &T) -> bool {
    true
}

// Calling `equals_self` with something that has a non-const impl should throw an error, despite
// it not using the impl.

pub const EQ: bool = equals_self(&S);
//~^ ERROR

fn main() {}
