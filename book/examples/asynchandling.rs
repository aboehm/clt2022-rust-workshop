use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use tokio::task;
use tokio::time::{sleep, Duration};

/// Produziert Werte sendet sie über den Channel
async fn async_producer(tx: Sender<u32>, number: u32) {
    loop {
        let value: u32 = rand::random::<u32>() % 1000;
        println!("{:02}: Sende {}", number, value);

        let res = tx.send(value);
        if res.is_err() {
            break;
        }

        let delay = value + 1000u32;
        sleep(Duration::from_millis(delay.into())).await;
    }
}

/// Konsumiert die Werte aus dem Channel. Nach `count` Werten wird die Ausführung beendet
async fn async_consumer(rx: Receiver<u32>, count: u32) {
    for _ in 0..count {
        let value = rx.recv().unwrap();
        println!("Empfangen: {}", value);
    }
}

/// Ausführung in Tokio Runtime.
///
/// ```
/// let rt = tokio::runtime::Runtime::new().unwrap();
/// rt.block_on(async {
///   // async code
/// });
/// ```
#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    for num in 0..7 {
        task::spawn(async_producer(tx.clone(), num.clone()));
    }

    task::spawn(async_consumer(rx, 10)).await.unwrap();
}
