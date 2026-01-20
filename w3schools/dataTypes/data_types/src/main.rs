fn main() {
    //--------------- variables ---------------\\

    // Rust doesn't have to declare data types explicitly. It knows the type by the value assigned.
    let my_integer = 10; // integer 32 by default
    let my_float = 10.5; // float 64 by default
    let my_character = 'R'; // char type
    let my_boolean = true; // bool type
    let my_string = "Hello, Rust!"; // &str type

    // However , you can explicitly declare data types if needed.
    let my_explicit_integer: i64 = 100;
    let my_explicit_float: f32 = 20.5;
    let my_explicit_character: char = 'A';
    let my_explicit_boolean: bool = false;
    let my_explicit_string: String = String::from("Explicit String");


    //--------------- Constants ---------------\\

    // Constants are declared using the 'const' keyword and must have a type annotation. Usually they are written in uppercase with underscores.
    const MAX_POINTS: u32 = 100000; // Apparently I can also write 100_000, so that it's more readable. In the million range it looks like this: 1_000_000. <-- thanks AI.
    const NAME: &str = "Dominik"; 
    println!("{} scored the this amount of points: {}", NAME, MAX_POINTS);


    //--------------- Booleans ---------------\\

    // Most of the time, there is no need to type true or false yourself. Instead, boolean values come from comparing values using operators like == or >:
    let age = 30;
    let is_adult = age >= 18; // This will be true
    println!("Is adult: {}", is_adult);
}
