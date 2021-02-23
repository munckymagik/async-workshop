use std::thread;
use std::time;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(async {
        // 1. This is using the blocking thread::sleep. Switch it to use the
        // futures aware https://docs.rs/tokio/1.2.0/tokio/time/struct.Sleep.html
        thread::sleep(rand_duration(0, 1000));
        1i32
    });

    let t2 = tokio::spawn(async {
        // 1. Ditto as above
        thread::sleep(rand_duration(0, 1000));
        2i32
    });

    tokio::select! {
        // 2. What kinds of errors might be returned? How should we handle them?
        Ok(val) = t1 => {
            println!("got {} from t1", val);
        }
        Ok(val) = t2 => {
            println!("got {} from t2", val);
        }
    };
}

pub fn rand_duration(from_ms: u64, to_ms: u64) -> time::Duration {
    let dur_ms = thread_rng().gen_range(from_ms..to_ms);
    time::Duration::from_millis(dur_ms)
}
