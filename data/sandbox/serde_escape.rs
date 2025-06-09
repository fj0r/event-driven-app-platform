//! ```cargo
//! [dependencies]
//! kafka = "0.10.0"
//! serde = { version = "1.0.219", features = ["derive"] }
//! minijinja = { version = "2.10.2", features = ["loader"] }
//! serde_derive = "1.0.219"
//! serde_json = "1.0.140"
//! ```

use serde::{Serialize, Deserialize};
use minijinja::{Environment, context};

#[derive(Deserialize, Serialize, Debug)]
struct Tx {
    test: String
}

fn main() {
    let p = "{\"test\": \"{{\"{{xxx}}\"}}\"}";
    let p = std::fs::read_to_string("data/sandbox/serde_escape.json").unwrap();

    println!("{}", &p);
    let x = serde_json::from_str::<Tx>(&p);
    let mut e = Environment::new();
    e.add_template("test", &p).unwrap();
    let c = context!{ xxx => "\"\n" };
    let y = e.get_template("test").unwrap().render(c).unwrap();
    println!("{:?}", y);

}
