fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 1. Convert this async block into the async_main fn sketched below.
    let async_main = async {
        let fut1 = async {
            1 * 7
        };
        let fut2 = async {
            5 * 7
        };

        let task1 = tokio::spawn(fut1);
        let task2 = tokio::spawn(fut2);

        let result1 = task1.await.unwrap();
        let result2 = task2.await.unwrap();

        result1 + result2
    };

    let result = rt.block_on(async_main);

    println!("Result = {}", result);
}

// 2. For extra learning, investigate the `#[tokio::main]` attribute
// async fn async_main() -> i32 {
//     0
// }
