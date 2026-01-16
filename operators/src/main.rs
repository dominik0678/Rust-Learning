fn main() {
    // Rust supports many common operators, like: Arithmetic operators, Assignment operators, Comparison operators, Logical operators, Bitwise operators, and more.
    // Arithmetic operators are used to do basic math:

    // Arithmetic Operators:
    let a = 10;
    let b = 20;

    // Addition
    let sum = a + b;
    println!("Sum: {}", sum);

    // Subtraction
    let difference = b - a;
    println!("Difference: {}", difference);

    // Multiplication
    let product = a * b;
    println!("Product: {}", product);

    // Division
    let quotient = b / a;
    println!("Quotient: {}", quotient);

    // Remainder
    let remainder = b % a;
    println!("Remainder: {}", remainder);


    // Assignment Operators:
    let mut c = 5;
    c += 10; // Equivalent to c = c + 10
    c -= 3;  // Equivalent to c = c - 3
    c *= 2;  // Equivalent to c = c * 2
    c /= 4;  // Equivalent to c = c / 4
    c %= 3;  // Equivalent to c = c % 3
    println!("Final value of c: {}", c);


    // Comparison Operators:
    let x = 15;
    let y = 25;

    println!("x == y: {}", x == y); // Equal to
    println!("x != y: {}", x != y); // Not equal
    println!("x > y: {}", x > y);   // Greater than
    println!("x < y: {}", x < y);   // Less than
    println!("x >= y: {}", x >= y); // Greater than or equal to
    println!("x <= y: {}", x <= y); // Less than or equal to


    // Logical Operators:
    let p = true;
    let q = false;

    println!("p && q: {}", p && q); // Logical AND
    println!("p || q: {}", p || q); // Logical OR
    println!("!p: {}", !p);         // Logical NOT
}
