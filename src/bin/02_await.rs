fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let result = rt.block_on(async {
        // 1. Divide the work of computing 42 into 2 async blocks
        // let fut1 = ...;
        // let fut2 = ...;

        // 2. Spawn them in the runtime using `tokio::spawn(...)`
        // let task1 = ...;
        // let task2 = ...;

        // 3. Await their results by calling `.await`
        // let result1 = ...;
        // let result2 = ...;

        // 4. Return the sum of their results from this block
        // result1 + result2
        42
    });

    println!("Result = {}", result);
}
