fn main() {
    // Sometime you want to use a value without taking ownership of it.
    // Rust lets you do this using a reference - this is called borrowing.

    let a = String::from("Hello");
    let b = &a; // b is a reference to a
    // Since b is only a reference, a is still the owner of the String.

    println!("a: {}, b: {}", a, b);


    // You can even do it with mutable references. But the you have to daclare it.
    let mut name = String::from("Dominik");
    let name_ref = &mut name;

    // Note: You can only have one mutable reference to a value at a time.



    // But why borrowing?
    // It lets you use calues without taking ownership of them. 
    // It avoids cloning, which can be slow for large data.
    // It makes your programs safer and faster.
}
