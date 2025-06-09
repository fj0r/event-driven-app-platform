use std::{
    collections::{HashMap, hash_map::Iter},
    ops::Deref,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Session(pub u32);

#[derive(Clone, Debug)]
pub struct SessionManager<T> {
    map: HashMap<Session, T>,
}

impl<T> SessionManager<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // TODO:
    fn insert(&mut self, k: Session, v: T) -> Option<T> {
        self.map.insert(k, v)
    }

    // TODO:
    fn remove(&mut self, k: &Session) -> Option<T> {
        self.map.remove(k)
    }
}


fn main () -> Result<(), Box<dyn std::error::Error>> {
    let mut a = SessionManager::<String>::new();
    a.insert(Session(1), "adf".to_string());
    a.insert(Session(2), "adf".to_string());
    a.remove(&Session(1));
    println!("{:?}", a);
    Ok(())
}
