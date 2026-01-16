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
        _ => println!("A number greater than thirteen"),
    }
}
