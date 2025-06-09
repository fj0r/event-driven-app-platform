use std::{
    hash::Hash,
    collections::HashMap,
    path::Path,
};

#[derive(Debug, Default, Eq, PartialEq)]
struct Container<Key: Hash + Eq, Value> {
    container: HashMap<Key, Value>,
}

fn main() {
    let x: Container<String, String> = Default::default();
    dbg!(&x);
}
