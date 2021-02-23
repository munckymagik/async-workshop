#[tokio::main]
async fn main() {
    println!("hello from main");

    let mut receiver = generator();
    let mut result: i32 = 0;

    for _ in 0i32..5 {
        if let Some(val) = receiver.recv().await {
            println!("received {}", val);
            result += val
        } else {
            println!("None returned.")
            // 3. what does it mean if we get here? What should we do?
        }
    }

    // 2. We drop the receiver, but there might be messages queued and unread.
    //    Use [close](https://docs.rs/tokio/1.2.0/tokio/sync/mpsc/struct.Receiver.html#method.close)
    //    and drain the buffer before exiting. You might want to speed up the
    //    producer to see the effect of this.
    drop(receiver);

    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Result of async op: {}", result);
}

fn generator() -> tokio::sync::mpsc::Receiver<i32> {
    let (sender, receiver) = tokio::sync::mpsc::channel::<i32>(1);

    tokio::spawn(async move {
        for i in 0i32.. {
            // 1. When the consumer goes away we panic. How can we detect this
            //    and shutdown cleanly?
            sender.send(i).await.expect("sending failed");

            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    });

    receiver
}
