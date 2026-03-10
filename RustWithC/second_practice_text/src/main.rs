mod student_ffi;

fn main() {
    println!("Rust <-> C starter: text_tools");

    let sample = "Rust and C together";
    match student_ffi::count_vowels(sample) {
        Ok(count) => println!("count_vowels(\"{sample}\") = {count}"),
        Err(err) => println!("count_vowels error: {err}"),
    }

    match student_ffi::to_upper_ascii("ffi starter") {
        Ok(value) => println!("to_upper_ascii -> {value}"),
        Err(err) => println!("to_upper_ascii: {err}"),
    }

    match student_ffi::starts_with("RUSTACEAN", "RUST") {
        Ok(value) => println!("starts_with -> {value}"),
        Err(err) => println!("starts_with: {err}"),
    }

    println!("Open src/student_ffi.rs and implement TODO 2 + TODO 3.");
}
