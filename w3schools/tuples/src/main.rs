fn get_user() -> (String, i32) {
        (String::from("Dominik"), 19)   // This is a tuple being returned from a function. While in this function the tuple has no name. But when we call it, we can assign it to a variable name.
    }

fn main() {
    // A tuple is a group of values of different types, stored in a singe variable.
    // These are useful when you want to return or work withh multiple values together.

    // Creating a tuple is similar to creating an array, but you use parentheses instead of square brackets. () <- []
    let person = ("Dominik", 19, 1.75);

    // Now to access the values, we call the tuple (like object-oriented programming) and use a dot followed by the index of the value we want to access.
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}m", person.2);


    // When we create a tuple, we normally assign values to it. This is called "packing" a tuple.
    // But in Rust we are also allowed to "unpack" a tuple into different variables.
    let (name, age, height) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}m", height);


    // Tuples are often used when returning multiple values from a function.
    // Up on the first line of this file.
    let user = get_user();
    println!("User: {} is {} years old.", user.0, user.1);
}
