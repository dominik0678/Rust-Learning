fn main() {
    // A vector is a rezizable array. 
    // To create a new vector, we use the Vec::new() function or the vec! macro.
    let fruits = vec!["Apple", "Banana", "Cherry"];

    // Just like in arrays, we can access elements in a vector using indexing.
    println!("First fruit: {}", fruits[0]);


    // To change a value in a vetor, refer the the index number and assign a new value.
    let mut vegetables = vec!["Carrot", "Broccoli", "Spinach"];
    vegetables[1] = "Cauliflower";
    println!("Updated vegetables: {:?}", vegetables);


    // Unlike arrays, vectors can grow and shrink in size.
    // To add an element to the end of a vector, we use the push() method.
    // To remove the last element from a vector, we use the pop() method.
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    println!("Numbers after push: {:?}", numbers);
    numbers.pop();
    println!("Numbers after pop: {:?}", numbers);


    // To add or remove elements at specific positions, we can use the insert() and remove() methods.
    numbers.insert(1, 5); // Insert 5 at index 1
    println!("Numbers after insert: {:?}", numbers);
    numbers.remove(2); // Remove element at index 2
    println!("Numbers after remove: {:?}", numbers);


    // Like arrays, you can get the length of a vector using the len() method.
    println!("Length of numbers vector: {}", numbers.len());

    // You can even loop through it.
    for number in &numbers {    // & is used to borrow the vector. It would work without it, but then we wouldn't be able to use numbers after the loop.
        println!("Number: {}", number);
    }
}
