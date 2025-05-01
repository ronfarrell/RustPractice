use std::io;
mod greeter;


fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("Please enter some text: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    greeter::greet(&input.trim());

}
