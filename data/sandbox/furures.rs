//! ```cargo
//! [dependencies]
//! futures = "0.3"
//! ```
use futures::{executor::block_on, stream::{self, StreamExt}};

async fn count_stream() {
    let mut stream = stream::iter(1..=5);
    while let Some(value) = stream.next().await {
        println!("Value: {}", value);
    }
}

fn main() {
    block_on(count_stream());
}
