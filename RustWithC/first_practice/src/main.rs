unsafe extern "C" {
    fn math_add(a: i32, b: i32) -> i32;
    fn math_sub(a: i32, b: i32) -> i32;
    fn math_mul(a: i32, b: i32) -> i32;
    fn math_abs(a: i32) -> i32;
    fn math_max(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 5;
    let b = 10; 
    let add;
    let sub;
    let mul;
    let abs;
    let max;

    unsafe { // Here the C funcions are used.
        add = math_add(a, b);
        sub = math_sub(a, b);
        mul = math_mul(a, b);
        abs = math_abs(b);
        max = math_max(a, b);
    }

    println!("{add}");
    println!("{sub}");
    println!("{mul}");
    println!("{abs}");
    println!("{max}");
}
