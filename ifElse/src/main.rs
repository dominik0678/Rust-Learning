fn main() {
    // Use if to specify a block of code to be executed if a condition is true.
    let number = 7;
    if number < 5 {
        println!("The number is less than 5."); 
    } else {
        println!("The number is 5 or greater.");
    }


    // You can also test variables:
    let x = 10;
    let y = 20;
    if x > y {
        println!("x is greater than y.");
    } else {    
        println!("x is not greater than y.");
    }


    // You can check multiple conditions using else if:
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }


    // In Rust, if...else can also be used as an expression.
    let time = 14;
    let greeting = if time < 12 {
        "Good morning!"
    } else {
        "Good afternoon!"
    };
    // same as let greeting = if time < 12 { "Good morning!" } else { "Good afternoon!" };
    println!("{}", greeting);
}
