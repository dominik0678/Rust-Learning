enum Direction {
        North,
        South,
        East,
        West,
    }

enum LoginStatus {
        Success(String),
        Error(String),
    }


fn main() {
    // An enum, short for "enumeration", is a way to define a type that can be one of several different variants.
    // To create an enum in Rust, you use the `enum` keyword followed by the name of the enum and a block containing its variants.
    // Usually you define enums at the module level (outside of main).
    
    let go = Direction::North;


    // Enums work well with match statements. You can run different code depending on which variant the enum is.
    match go {
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East  => println!("Going East!"),
        Direction::West  => println!("Going West!"),
    }


    // Enums can also have data associated with each variant.
    // Again, you define this at the module level.

    let result1 =  LoginStatus::Success(String::from("User logged in successfully."));
    let result2 =  LoginStatus::Error(String::from("Invalid username or password."));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message)   => println!("Error: {}", message),
    }

    // For me it's still complicated to remember the syntax and the structure of enums.
    // But with practice, it becomes easier to use them effectively in Rust programs.
}
