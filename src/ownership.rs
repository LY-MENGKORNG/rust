pub fn ownership() {
    let a = String::from("Hello");
    let b = a;
    // println!("{}", a); // BUG: `a` no longer owns the value
    println!("{}", b); // Ok: b now owns the value

    // INFO: With simple types like `numbers`, `characters`, and `boolean`
    // When we assign `a` to `b`, the ownership moves. This means only `b` can use the value now, because `a` is no longer valid.
    // But simple types like numbers, characters and booleans are copied, not moved.
    // This means you can still use the original variable after assigning it to another:
    let a = 5;
    let b = a;
    println!("a = {}", a); // Works
    println!("b = {}", b); // Works

    // INFO: With `clone`
    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value
    println!("a = {}", a); // Works
    println!("b = {}", b); // Works
}
