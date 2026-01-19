fn main() {
    // Same as variables, strings can be mutable. 
    // It's done by either using mut keyword:
    let mut str = "Hi".to_string();
    // or by using String type:
    let text = String::from("Hello");
    println!("{} {}", str, text);


    // Now to change the value of the string, we can use push_str method:
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("{}", greeting);

    // Or use push() to add a single character:
    greeting.push('D'); // there you have to use single quotes for characters
    println!("{}", greeting);


    // To combine two strings, we can use the + operator or format! macro:
    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let combined = str1 + &str2; // note str1 is moved here and can no longer be used. Only a string slice can be added.
    println!("{}", combined);
    // or using format! macro:
    let str3 = String::from("Hello, ");
    let str4 = String::from("world!");
    let combined_macro = format!("{}{}", str3, str4); // here str3 is still valid
    println!("{}", combined_macro);
    println!("{}, {}, {}", str2, str3, str4); // just to show that str1 is no longer valid (I removed it, so the code compiles)


    // To change the string completely, we can simply assign a new value:
    let mut message = String::from("Good morning");
    println!("{}", message);
    message = String::from("Good evening");
    println!("{}", message);


    // To measure the length of a string, we can use the len() method:
    let sample = String::from("Sali zeammae!"); // Note: don't use ä, ö, ü or ß. They take more than one byte. (ä = ae, ö = oe, ü = ue, ß = ss)
    println!("Length of '{}': {}", sample, sample.len());

}
