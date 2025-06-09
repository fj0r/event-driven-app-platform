
//! ```cargo
//! [dependencies]
//! rand = "0.9.1"
//! ```
use rand::Rng;
fn main() {
    let mut rng = rand::rng();
    let x: u64 = rng.random();
    println!("{}", x);
}
