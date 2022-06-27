fn first_word(s: &mut String) -> String {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return String::from(&s[0..i]);
        }
    }

    String::from(&s[..])
}

fn main() {
    let mut s = String::from("rust is cool");

    let word = first_word(&mut s);

    s.clear();

    println!("the first word is: {}", word);
    println!("{}", s);
}
