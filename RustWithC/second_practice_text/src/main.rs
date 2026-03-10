use std::os::raw::c_char;
use std::ffi::CString;

unsafe extern "C" {
    fn text_count_vowels(s: *const c_char) -> i32;
    fn text_count_spaces(s: *const c_char) -> i32;
    fn text_has_digit(s: *const c_char) -> bool;
}

fn main() {
    let text = CString::new("Hello World").unwrap();
    let count_v;
    let count_s;
    let has_d;

    unsafe {
        count_v = text_count_vowels(text.as_ptr());
        count_s = text_count_spaces(text.as_ptr());
        has_d = text_has_digit(text.as_ptr());
    }

    println!("{}", count_v);
    println!("{}", count_s);
    println!("{}", has_d);
}
