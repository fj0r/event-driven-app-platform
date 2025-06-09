//! ```cargo
//! [dependencies]
//! kafka = "0.10.0"
//! serde_json = "1.0.140"
//! ```

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = vec!["localhost:19092".into()];
    let mut c = Consumer::from_hosts(host)
        .with_topic("chat".into())
        .with_group("ws".into())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        //.with_client_id("kafka-rust-console-consumer".into())
        .create()?;

    loop {
        for ms in c.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{}:{}@{}:", ms.topic(), ms.partition(), m.offset);
            }
            let _ = c.consume_messageset(ms);
        }
        c.commit_consumed()?;
    }
}
