use std::io;

fn main() {
    println!("Hello, world!");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    
    println!("Value: {}", value.len());
}
