fn main() {
    // Arrays are simply created using the square brackets: []
    // Just make sure all the elements are of the same type.
    let a = [1, 2, 3, 4, 5];


    // To access an element in an array, we use indexing.
    println!("The first element of the array is: {}", a[0]);
    // or
    let first = a[0];
    println!("The first element of the array is: {}", first);


    // To change an element in an array, we need to make the array mutable. Then we can use indexing to assign a new value.
    let mut b = [10, 20, 30, 40, 50];
    b[2] = 100; // Change the third element (index 2)
    println!("The modified array is: {:?}", b);


    // You can get the length of an array using the len() method.
    println!("The length of the array is: {}", b.len());


    // You can loop through the array using a for loop.
    let fruits = ["apple", "banana", "cherry", "kiwi"];
    for fruit in fruits {
        println!("I like to eat: {}", fruit);
    }
}
