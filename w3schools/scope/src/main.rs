fn main() {
    // A variable created in a function only exist within that function
    fn my_function() {
        let x = 5;
        println!("The value of x is: {}", x);
    }

    my_function();

    //println!("{}", x); //This will cause an error because x is not in scope here.


    // Variables can also be created in blocks like in if statements or loops.
    let score = 85;

    if score >= 80 {
        let grade = 'A';
        println!("You passed with grade: {}", grade);
    }

    // same as before, if you would try to access grade here, it would cause an error


    // In Rust you can declare a variable with the same name in the same scope. This is called shadowing.
    let y = 10;
    let y = 5;

    println!("The value of y is: {}", y); // This will print 5


    // The variable name can be reused in inner scopes as well.
    let x = 5;

    {
        let x = 10; // This x shadows the outer x
        println!("Inside block: {}", x); // This will print 10
    }
    println!("Outside block: {}", x); // This will print 5
}
