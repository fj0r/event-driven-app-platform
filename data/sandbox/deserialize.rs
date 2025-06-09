//! ```cargo
//! [dependencies]
//! kafka = "0.10.0"
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_derive = "1.0.219"
//! serde_json = "1.0.140"
//! ```

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct Session(pub u128);

impl From<u128> for Session {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Session {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Envelope {
    pub receiver: Vec<Session>,
}

fn main() {
    let a = Envelope{receiver: vec![Session(1), Session(2)]};
    println!("{}", serde_json::to_string(&a).unwrap());
    let b = r#"{"receiver":[1,2]}"#;
    println!("{:?}", serde_json::from_str::<Envelope>(&b).unwrap());
    let b = r#"{"receiver":[]}"#;
    println!("{:?}", serde_json::from_str::<Envelope>(&b).unwrap());
}
