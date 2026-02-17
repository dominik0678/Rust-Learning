fn length(s: &String) -> usize {
    return s.len();
}
fn append_rust(s: &mut String) {
    s.push_str(" rust");
}
fn consume_and_return(mut s: String) -> String {
    // TODO: take ownership, add "!", return String
    s.push('!');
    return s;
}
fn main() {
    let mut text = String::from("learn");
    println!("len = {}", length(&text));
    append_rust(&mut text);
    text = consume_and_return(text);
    println!("final = {}", text); // should be "learn rust!"
}