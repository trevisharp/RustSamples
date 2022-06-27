fn main() {
    let mut s = String::from("rust");

    change(&mut s);

    print_len(s);
}

fn print_len(s: String)
{
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" is cool");
}