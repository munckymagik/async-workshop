fn main() {
    // 1. Convert this block into an async block `async { }`. What happens
    //    to its return value? Choose a new more suitable variable name.
    let result = {
        42
    };

    // 2. Create an instance of the Tokio runtime: `tokio::runtime::Runtime`
    // let rt = ...;

    // 3. Block waiting for its return value by calling `block_on`
    // let result = rt.....;

    println!("Result = {}", result);
}
