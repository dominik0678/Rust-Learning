use rand::seq::IndexedRandom;

fn main() {
    let new_password;
    let length = 50;

    new_password = password_gen(length);
    println!("{}", new_password);

}

fn password_gen(digits: i32) -> String {

    let characters = [
        // Grossbuchstaben
        'A','B','C','D','E','F','G','H','I','J','K','L','M',
        'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',

        // Kleinbuchstaben
        'a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z',

        // Zahlen
        '0','1','2','3','4','5','6','7','8','9',

        // Sonderzeichen (Standard ASCII)
        '!','"','#','$','%','&','\'','(',')','*','+',
        ',','-','.','/',
        ':',';','<','=','>','?',
        '@',
        '_',
        '{','|','}'
    ];


    let mut pw = String::from("");
    let mut rng = rand::rng();
    

    for _ in 1..=digits {
        pw += &characters.choose(&mut rng).unwrap().to_string();
    }
    // 
    //let pw = digits;
    pw.to_string()
}