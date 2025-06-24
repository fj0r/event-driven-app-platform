//! ```cargo
//! [dependencies]
//! serde = { version = "1.0.219", features = ["derive"] }
//! serde_derive = "1.0.219"
//! serde_json = "1.0.140"
//! time = { version = "0.3.41", features = ["formatting", "serde", "parsing"] }
//! chrono = { version = "0.4.41", features = ["serde"] }
//! ```
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;
use time::UtcDateTime;
use chrono::{DateTime, Utc};

impl Default for MyData {
    fn default() -> Self {
        Self {
            my_date: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    #[serde(with = "rfc3339")]
    my_date: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct UData (MyData);


impl Default for Y {
    fn default() -> Self {
        Self (OffsetDateTime::now_utc())
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Y(#[serde(with = "rfc3339")]OffsetDateTime);

#[derive(Debug, Serialize, Deserialize, Default)]
struct X {
    date: Y
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let n = std::time::Instant::now();
    println!("Instant::now() => {:?}", n);
    println!("Utc::now() => {:?}", Utc::now());
    let o1n = OffsetDateTime::now_utc();
    println!("OffsetDateTime::now_utc() => {:?}", &o1n);
    println!("Y => {:?}", serde_json::to_string(&Y(o1n)).unwrap());

    let json_string = r#"{"my_date": "2023-11-08T12:34:56Z"}"#; // Example JSON string
    let data: MyData = serde_json::from_str(json_string).unwrap(); // Deserialize
    println!("{:?}", data);

    let serialized_data = serde_json::to_string(&data).unwrap(); // Serialize
    println!("{}", serialized_data);


    let serialized_data = MyData {my_date: OffsetDateTime::now_utc()}; // Serialize
    println!("{:?}", serialized_data);

    let serialized_data = UData ( serialized_data ); // Serialize
    println!("{:?}", serialized_data);

    println!("{:?}", serde_json::to_string(&X::default()));

    let t = 1747384961261;
    println!("{:?}", OffsetDateTime::from_unix_timestamp_nanos(t * 1000_000));
    println!("{:?}", OffsetDateTime::from_unix_timestamp((t / 1000) as i64));

    Ok(())
}
