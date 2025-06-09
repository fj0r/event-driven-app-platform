//! ```cargo
//! [dependencies]
//! notify = "8.0.0"
//! ```

use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use std::path::Path;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // Use recommended_watcher() to automatically select the best implementation
    // for your platform. The `EventHandler` passed to this constructor can be a
    // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
    // another type the trait is implemented for.
    let mut watcher = notify::recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("./config.toml"), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    println!("event: {:?}", event)
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
