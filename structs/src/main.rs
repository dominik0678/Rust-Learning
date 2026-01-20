fn main() {
    // A struct, short for "structure", is a custom data type that lets you name and package together multiple related values.
    struct Color {
        red: u8,    // u8 meands unsigned integer with 8 bits
        green: u8,
        blue: u8, 
    }

    // Once you have a struct , you can create an object of it.
    // Then you can access the fields of the struct using dot syntax.
    let black = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    // or
    let white = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    println!("Black - R content: {}", black.red);
    println!("Black - G content: {}", black.green);
    println!("Black - B content: {}", black.blue);

    println!("White - R content: {}", white.red);
    println!("White - G content: {}", white.green);
    println!("White - B content: {}", white.blue);



    struct Person {
        name: String,
        age: u8,
    }

    // To change a value inside a struct, you have to make the entire struct mutable.
    let mut user = Person {
        name: String::from("Dominik"),
        age: 25,
    };

    user.age = 19;
    println!("{} is {} years old.", user.name, user.age);
}
