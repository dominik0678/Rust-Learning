fn main() {
    // Rust has 3 types of loops: loop, while, and for.
    // 1. Infinite loop using `loop`

    loop {
        println!("This will print forever until you break out of the loop.");
        break; // Break to avoid infinite printing (or use Ctrl+C to stop)
    }

    // Use with break
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("Breaking out of the loop at count = {}", count);
            break;
        }
    }

    // It can also return values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Returns 20
        }
    };
    println!("The result from the loop is: {}", result);



    // 2. Conditional loop using `while`
    let mut number = 3;
    while number <= 5 {
        println!("{} is less than or equal to 5", number);
        number += 1;
    }

    // A while loop can be stopped with break
    let mut n = 0;
    while n < 10 {
        if n == 7 {
            println!("Breaking out of the while loop at n = {}", n);
            break;
        }
        n += 1;
    }

    // Values can be skipped with continue
    let mut m = 0;
    while m < 5 {
        if m == 3 {
            m += 1;
            continue; // Skip the rest of the loop when m is 3
        }
        println!("m = {}", m);
        m += 1;
    }



    // 3. Iterative loop using `for`
    for i in 1..4 { // .. means from 1 to 3. 4 is excluded (the given number won't be included)
        println!("i = {}", i);
    }
    // Rust manages the counter variable automatically.

    // If the last number needs to be included, use ..=
    for j in 1..=4 { // ..= means from 1 to 4. 4 is included
        println!("j = {}", j);
    }

    // Just like other loops, break and continue can be used
    for k in 0..=10 {
        if k == 3 {
            println!("Skipping k = {}", k);
            continue; // Skip when k is 3
        }
        if k == 8 {
            println!("Breaking out of the for loop at k = {}", k);
            break; // Break when k is 8
        }
    }
}