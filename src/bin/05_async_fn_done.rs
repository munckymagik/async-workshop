#[tokio::main]
async fn main() {
    let task1 = spawn_compute(2);
    let task2 = spawn_compute(4);

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    println!("Result = {}", result1 + result2);
}

fn spawn_compute(mult: i32) -> tokio::task::JoinHandle<i32> {
    tokio::spawn(compute(mult))
}

async fn compute(mult: i32) -> i32 {
    mult * 7
}
