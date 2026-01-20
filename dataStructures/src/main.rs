use std::collections::HashMap;

fn main() {
    // In Rust, data structures are used to store and organize values.
    // It provides many build-in data structures to hande data diffferently.
    // The most common ones are: Arrays, Vectors, Tuples, HashMaps, and Structs.


    // Arrays:
    // In Rust an array is a fixed-size collection of elements of the same type. After its declaration, you cannot change its size.
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);   // To print the whole array you use {:?}.
    println!("First element of the array: {}", arr[0]); // To print a specific element, you use its index.


    // Vectors:
    // Vectors are similar to arrays, but they are dynamic in size. You can add or remove elements from a vector.
    let mut vec = vec![1, 2, 3];
    vec.push(4); // Adding an element to the vector.
    println!("Vector: {:?}", vec);


    // Tuples:
    // Tuples are like arrays, but for any kind of data type. You can create your own data salad here.
    // It's kinda object-orientated-like. You call the element, with a dot and its index.
    let person = ("Dominik", 19, 1.75);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);


    // HashMaps:
    // A HashMap is a collection of key-value pairs. You can use it to store data that you want to access using a specific key.
    // Import HashMap (up on the first line of this file).
    let mut capitalCities = HashMap::new();
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("France", "Paris");
    capitalCities.insert("Switzerland", "Bern");

    println!("Capital of Switzerland is {}", capitalCities["Switzerland"]);


    
    // Quick overview:
    // Can grow: Vectors, HashMaps
    // Fixed size: Arrays, Tuples

}
