#[tokio::main]
async fn main() {
    // DRY this code by creating a factory function that spawns async tasks
    // which return an i32

    let fut1 = async {
        2 * 7
    };
    let fut2 = async {
        4 * 7
    };

    // 1. Replace these two calls with the commented suggestions below.
    let task1 = tokio::spawn(fut1);
    let task2 = tokio::spawn(fut2);
    // let task1 = spawn_compute(2);
    // let task2 = spawn_compute(4);

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    println!("Result = {}", result1 + result2);
}
