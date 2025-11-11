#![feature(decl_macro)]
// try uncommenting the following line, and commenting out the line right after

// macro_rules! foo {
macro foo {
    ($name: ident) => {
        pub struct $name;

        impl $name {
            pub fn new() -> $name {
                $name
            }
        }
    },
}

foo!(Foo);

fn main() {
    // this fails with a `macro`, but succeeds with a `macro_rules`
    let foo = Foo::new();
}
