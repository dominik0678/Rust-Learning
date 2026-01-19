fn main() {
    // To create a function, the keyword "fn" is used followed by the function name and parentheses.
    fn greet() {
        println!("Hello, world!");
    }


    // To use a function with parameters, define them within the parentheses. Just like you would with variables.
    fn greet_person(name: &str) {
        println!("Hello, {}!", name);
    }


    // A function can also return a value using the "->" syntax followed by the return type.
    fn add(a: i32, b: i32) -> i32 {
        a + b   // return can be used, but the you shouldn't forget the semicolon if you do. a + b == return a + b;
    }


    // Call functions here:
    greet();
    greet_person("Dominik");
    let sum = add(5, 7);
    println!("The sum is: {}", sum);

}
