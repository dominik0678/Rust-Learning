fn main() {
    // Rust uses "ownership" to manage memory in a safe way.
    // Usually a variable owns the data it points to.
    let s1 = String::from("hello"); // s1 owns the String data

    // But if I do:
    let s2 = s1; // ownership of the String data is moved from s1 to s2

    // Simple types like numbers, characters and booleans are copied, not moved.
    // Means, you can still use to the original variable.

    let a = 5;
    let b = a;
    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works


    // More complex types like String, Vec, HashMap are moved, not copied.
    // Means, you cannot use the original variable after the move.
}
