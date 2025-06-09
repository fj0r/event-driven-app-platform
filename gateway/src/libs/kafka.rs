use super::message::{Created, Event, MessageQueueEvent, MessageQueuePush};
use super::settings::{QueueEvent, QueuePush};
use rdkafka::Timestamp;
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};
use tokio::task::spawn;
use tracing::{error, info, warn};

impl From<Timestamp> for Created {
    fn from(value: Timestamp) -> Self {
        if let Timestamp::CreateTime(ts) = value {
            if let Ok(ts) = OffsetDateTime::from_unix_timestamp(ts / 1000) {
                return Self(ts);
            }
        }
        Self(OffsetDateTime::from_unix_timestamp(0).unwrap())
    }
}

#[derive(Clone)]
pub struct KafkaManagerEvent<T>
where
    T: Send + Serialize + DeserializeOwned,
{
    tx: Option<UnboundedSender<T>>,
    producer: QueueEvent,
}

impl<T> KafkaManagerEvent<T>
where
    T: Send + Serialize + DeserializeOwned + 'static,
{
    pub fn new(producer: QueueEvent) -> Self {
        Self { tx: None, producer }
    }
}

impl<T> MessageQueueEvent for KafkaManagerEvent<T>
where
    T: Debug + Clone + Send + Serialize + DeserializeOwned + 'static,
{
    type Item = T;

    async fn run(&mut self) {
        let (producer_tx, mut producer_rx) = unbounded_channel::<Self::Item>();
        let producer_cfg = self.producer.clone();

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &producer_cfg.broker[0])
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create Kafka producer");

        spawn(async move {
            // let topic : Vec<&str> = producer_cfg.topic.iter().map(<_>::as_ref).collect();
            while let Some(value) = producer_rx.recv().await {
                let value = serde_json::to_string(&value).expect("serde to string");
                let _delivery_status = producer
                    .send(
                        FutureRecord::to(&producer_cfg.topic)
                            .payload(&value)
                            .key("")
                            .headers(OwnedHeaders::new().insert(Header {
                                key: "",
                                value: Some(""),
                            })),
                        Duration::from_secs(0),
                    )
                    .await;
            }
        });

        self.tx = Some(producer_tx);
    }

    fn get_tx(&self) -> Option<UnboundedSender<Self::Item>> {
        self.tx.clone()
    }
}

#[derive(Clone)]
pub struct KafkaManagerPush<T>
where
    T: Send + Serialize + DeserializeOwned,
{
    rx: Option<Arc<Mutex<UnboundedReceiver<T>>>>,
    consumer: QueuePush,
}

impl<T> KafkaManagerPush<T>
where
    T: Send + Serialize + DeserializeOwned + 'static,
{
    pub fn new(consumer: QueuePush) -> Self {
        Self { rx: None, consumer }
    }
}

impl<T> MessageQueuePush for KafkaManagerPush<T>
where
    T: Debug + Clone + Send + Serialize + DeserializeOwned + Event + 'static,
{
    type Item = T;

    async fn run(&mut self) {
        let (consumer_tx, consumer_rx) = unbounded_channel::<Self::Item>();
        let consumer_cfg = self.consumer.clone();

        let context = CustomContext;

        let consumer: LoggingConsumer = ClientConfig::new()
            .set("group.id", consumer_cfg.group.unwrap_or("default".into()))
            .set("bootstrap.servers", consumer_cfg.broker[0].clone())
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Failed to create Kafka consumer");

        spawn(async move {
            let topic: Vec<&str> = consumer_cfg.topic.iter().map(<_>::as_ref).collect();

            consumer
                .subscribe(topic.as_slice())
                .expect("Can't subscribe to specified topics");
            loop {
                match consumer.recv().await {
                    Err(e) => warn!("Kafka error: {}", e),
                    Ok(m) => {
                        let payload = match m.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
                                warn!("Error while deserializing message payload: {:?}", e);
                                ""
                            }
                        };

                        match serde_json::from_str::<Self::Item>(payload) {
                            Ok(mut value) => {
                                value.set_time(m.timestamp().into());
                                if let Err(e) = consumer_tx.send(value) {
                                    error!("Failed to send message from consumer: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize: {:?}", e);
                            }
                        }
                        /*
                        info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                              m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                        if let Some(headers) = m.headers() {
                            for header in headers.iter() {
                                info!("  Header {:#?}: {:?}", header.key, header.value);
                            }
                        }
                        */
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                    }
                }
            }
        });
        self.rx = Some(Arc::new(Mutex::new(consumer_rx)));
    }

    fn get_rx(&self) -> Option<Arc<Mutex<UnboundedReceiver<Self::Item>>>> {
        self.rx.clone()
    }
}

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;
