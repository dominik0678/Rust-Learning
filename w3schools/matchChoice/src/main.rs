fn main() {
    // When you have many choices, using match is easier than writing lots of if...else.
    // match is used to select one of many code blocks to be executed. It's like a switch statement in other languages.

    let number = 13;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6 => println!("Six"),
        7 => println!("Seven"),
        8 => println!("Eight"),
        9 => println!("Nine"),
        10 => println!("Ten"),
        11 => println!("Eleven"),
        12 => println!("Twelve"),
        13 => println!("Thirteen"),
        _ => println!("A number greater than thirteen (or less than one)"),
    }


    // If multiple patterns should do the same thing, you can combine them with | (OR operator).
    let another_number = 7;

    match another_number {
        1 | 3 | 5 | 7 | 9 | 11 | 13 => println!("Odd number"),
        2 | 4 | 6 | 8 | 10 | 12 => println!("Even number"),
        _ => println!("A number greater than thirteen (or less than one)"),
    }

    
    // Match can also return a value.
    let day = 4;

    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    };
    println!("The day is: {}", result);
}
