//! ```cargo
//! [dependencies]
//! rdkafka = { version = "0.37", features = ["cmake-build"] }
//! serde_json = "1.0.140"
//! tokio = { version = "1.44.2", features = ['full'] }
//! libc = "0.2.0"
//! tracing = "0.1.41"
//! tracing-subscriber = "0.3.19"
//! ```

use tracing::{info, warn};
use tracing_subscriber;

use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::{Value, from_str, to_string};
use std::time::Duration;

static JV: &str = r#"
{
  "sender": "test",
  "content": {
    "action": "data",
    "event": "test-data",
    "data": {
      "type": "Text",
      "data": {
        "upload": true,
        "event": "user-message"
      },
      "value": "Guide"
    }
  }
}
"#;

async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let jv = from_str::<Value>(JV).unwrap();
    let jv = to_string(&jv).unwrap();
    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..1)
        .map(|i| {
            let value = jv.clone();
            async move {
                // The send operation on the topic returns a future, which will be
                // completed once the result or failure from Kafka is received.
                let delivery_status = producer
                    .send(
                        FutureRecord::to(topic_name)
                            .payload(&value)
                            .key(&format!("Key {}", i))
                            .headers(OwnedHeaders::new().insert(Header {
                                key: "header_key",
                                value: Some("header_value"),
                            })),
                        Duration::from_secs(0),
                    )
                    .await;

                // This will be executed when the result is received.
                info!("Delivery status for message {} received", i);
                delivery_status
            }
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("start");
    let topics = "chat";
    let brokers = "localhost:19092";

    produce(brokers, topics).await;
}
