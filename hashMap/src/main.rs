use std::collections::HashMap;

fn main() {
    // A HashMap is a collection of key-value pairs.
    // HashMaps are great when you want to store values and find them by a key.

    // To use HashMaps, you have to import the HashMap type from the standard library.


    // You can create a new, empty HashMap and add items to it.
    let mut capitalCities = HashMap::new();

    // Add keys and values (country, city)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");
    capitalCities.insert("Switzerland", "Bern");

    println!("{:?}", capitalCities);


    // You can use the get() method to access values in a HashMap.
    if let Some(city) = capitalCities.get("Germany") {  // Some is used to handle the case where the key might not exist. 
        println!("The capital of Germany is: {}", city);
    } else {
        println!("City not found");
    }


    // To update a value, you can use the insert() method again.
    capitalCities.insert("England", "Manchester"); // This will update the value for the key "England".
    println!("{:?}", capitalCities);
    capitalCities.insert("England", "London");  // I have to change it back for my own sanity.

    // To remove a key-value pair, you can use the remove() method.
    capitalCities.remove("Norway");
    println!("{:?}", capitalCities);


    // And just like arrays and vectors, you can use a for loop to iterate through the keys and values of a HashMap.
    for (country, city) in &capitalCities { // Again, the & is used to borrow the HashMap.
        println!("The capital of {} is {}", country, city);
    }
}
